use oneshot;
use std::{sync, thread};

pub enum Task {
    Execute(Box<dyn FnOnce() + Send + 'static>),
    Exit,
}

pub struct Worker {
    thread: Option<thread::JoinHandle<()>>,
    sender: sync::mpsc::Sender<Task>,
}

impl Worker {
    pub fn spawn() -> Self {
        let (task_sender, task_receiver) = sync::mpsc::channel();

        Self {
            sender: task_sender,
            thread: Some(thread::spawn(move || {
                while let Ok(task) = task_receiver.recv() {
                    match task {
                        Task::Execute(work) => work(),
                        Task::Exit => break,
                    }
                }
            })),
        }
    }

    pub fn run<F, T>(&mut self, task: F) -> RefHandle<'_, T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (signal_sender, signal_receiver) = oneshot::channel();

        let work = Task::Execute(Box::new(|| {
            let _ = signal_sender.send(task());
        }));

        self.sender.send(work).expect("the worker thread died");

        RefHandle {
            worker: self,
            signal: Some(signal_receiver),
        }
    }

    pub fn execute<F>(&mut self, task: F) -> RefHandle<'_, ()>
    where
        F: FnOnce() + Send + 'static,
    {
        self.run(task)
    }

    pub fn run_owned<F, T>(self, task: F) -> OwnedHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (signal_sender, signal_receiver) = oneshot::channel();

        let work = Task::Execute(Box::new(|| {
            let _ = signal_sender.send(task());
        }));

        self.sender.send(work).expect("the worker thread died");

        OwnedHandle {
            inner: Some((self, signal_receiver)),
        }
    }

    pub fn execute_owned<F>(self, task: F) -> OwnedHandle<()>
    where
        F: FnOnce() + Send + 'static,
    {
        self.run_owned(task)
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.sender.send(Task::Exit).unwrap();
        self.thread.take().unwrap().join().unwrap();
    }
}

pub struct OwnedHandle<T> {
    inner: Option<(Worker, oneshot::Receiver<T>)>,
}

impl<T> OwnedHandle<T> {
    pub fn try_join(&mut self) -> Option<(Worker, T)> {
        let value = self.inner.as_ref()?.1.try_recv().ok()?;
        let worker = self.inner.take()?.0;
        Some((worker, value))
    }

    pub fn join(mut self) -> Option<(Worker, T)> {
        let inner = self.inner.take()?;
        Some((inner.0, inner.1.recv().unwrap()))
    }
}

impl<T> Drop for OwnedHandle<T> {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            inner.1.recv_ref().expect("the worker thread died");
        }
    }
}

pub struct RefHandle<'a, T> {
    worker: &'a mut Worker,
    signal: Option<oneshot::Receiver<T>>,
}

impl<T> RefHandle<'_, T> {
    pub fn worker(&self) -> &Worker {
        self.worker
    }

    pub fn join(mut self) -> T {
        self.signal
            .take()
            .unwrap()
            .recv()
            .expect("the worker thread died")
    }
}

impl<T> Drop for RefHandle<'_, T> {
    fn drop(&mut self) {
        if let Some(signal) = self.signal.take() {
            signal.recv_ref().expect("the worker thread died");
        }
    }
}

pub struct WorkerGroup {
    available: Vec<Worker>,
}

impl WorkerGroup {
    pub fn new(workers: usize) -> Self {
        Self {
            available: (0..workers).map(|_| Worker::spawn()).collect(),
        }
    }

    pub fn from_workers(workers: Vec<Worker>) -> Self {
        Self { available: workers }
    }

    pub fn available(&self) -> usize {
        self.available.len()
    }

    pub fn extend(&mut self, workers: impl Iterator<Item = Worker>) {
        self.available.extend(workers)
    }

    pub fn take(&mut self, count: usize) -> Option<Self> {
        if count > self.available.len() {
            return None;
        }

        Some(Self {
            available: self.available.drain(0..count).collect(),
        })
    }

    pub fn take_one(&mut self) -> Option<Worker> {
        self.available.pop()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Worker> {
        self.available.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Worker> {
        self.available.iter_mut()
    }

    pub fn into_inner(self) -> Vec<Worker> {
        self.available
    }

    pub fn inner(&self) -> Vec<&Worker> {
        self.available.iter().collect()
    }

    pub fn inner_mut(&mut self) -> Vec<&mut Worker> {
        self.available.iter_mut().collect()
    }
}

impl IntoIterator for WorkerGroup {
    type Item = Worker;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.available.into_iter()
    }
}

mod tests {
    #[test]
    fn worker_refhandle() {
        let mut worker = super::Worker::spawn();

        let handle = worker.run(|| 1);

        assert_eq!(handle.join(), 1);

        let handle = worker.run(|| Some("hello".to_string()));

        assert_eq!(handle.join(), Some("hello".to_string()));
    }

    #[test]
    fn worker_ownedhandle() {
        let worker = super::Worker::spawn();

        let handle = worker.run_owned(|| 1);

        let (worker, result) = handle.join().unwrap();
        assert_eq!(result, 1);

        let handle = worker.run_owned(|| 2);

        let (_, result) = handle.join().unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn worker_drop_handles() {
        let mut worker = super::Worker::spawn();

        let (signal_sender, signal_receiver) = oneshot::channel();

        {
            let _ = worker.execute(|| {
                signal_sender.send(1).unwrap();
            });
        }

        assert_eq!(signal_receiver.recv().unwrap(), 1);

        let (signal_sender, signal_receiver) = oneshot::channel();

        {
            let _ = worker.execute_owned(|| {
                signal_sender.send(2).unwrap();
            });
        }

        assert_eq!(signal_receiver.recv().unwrap(), 2);
    }
}
