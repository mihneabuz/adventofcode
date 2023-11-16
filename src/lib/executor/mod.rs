mod notifier;
mod worker;

use std::{mem, thread};

use crate::challenge::{ChallengeObject, ChallengeResult};
use notifier::Notifier;
pub use worker::*;

pub struct AocExecutor {
    total: usize,
    worker_group: WorkerGroup,
    scheduled: Vec<OwnedHandle<(ChallengeResult, WorkerGroup)>>,
    notifier: Notifier,
    results: Vec<ChallengeResult>,
}

impl AocExecutor {
    pub fn new() -> Self {
        let workers = thread::available_parallelism().unwrap().get();
        Self::with_workers(workers)
    }

    pub fn with_workers(workers: usize) -> Self {
        Self {
            total: workers,
            worker_group: WorkerGroup::new(workers),
            scheduled: Vec::new(),
            notifier: Notifier::new(),
            results: Vec::new(),
        }
    }

    pub fn run_all(&mut self, challenges: Vec<ChallengeObject>) -> Vec<ChallengeResult> {
        for challenge in challenges.into_iter() {
            self.run(challenge);
        }

        self.join_all();

        mem::take(&mut self.results)
    }

    fn run(&mut self, challenge: ChallengeObject) {
        let count = (challenge.worker_hint.unwrap_or(0) + 1).min(self.total);
        let mut workers = self.wait_for_workers(count);

        let notifier = self.notifier.clone();
        let handle = workers.take_one().unwrap().run_owned(move || {
            let result = challenge.solve(&mut workers);
            notifier.signal();
            (result, workers)
        });

        self.scheduled.push(handle);
    }

    fn wait_for_workers(&mut self, count: usize) -> WorkerGroup {
        loop {
            if let Some(workers) = self.worker_group.take(count) {
                return workers;
            }

            self.notifier.wait();
            self.join();
        }
    }

    fn join(&mut self) {
        self.scheduled.retain_mut(|handle| match handle.try_join() {
            Some((main_worker, (result, extra_workers))) => {
                self.results.push(result);
                self.worker_group
                    .extend(Some(main_worker).into_iter().chain(extra_workers));

                false
            }

            None => true,
        });
    }

    fn join_all(&mut self) {
        for handle in self.scheduled.drain(..) {
            let (main_worker, (result, extra_workers)) = handle.join().unwrap();
            self.results.push(result);
            self.worker_group
                .extend(Some(main_worker).into_iter().chain(extra_workers));
        }
    }
}

impl Default for AocExecutor {
    fn default() -> Self {
        Self::new()
    }
}
