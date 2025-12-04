use std::sync::{Arc, Condvar, Mutex};

pub struct Notifier(Arc<(Mutex<bool>, Condvar)>);

impl Notifier {
    pub fn new() -> Self {
        Self(Arc::new((Mutex::new(false), Condvar::new())))
    }

    pub fn wait(&self) {
        let mut joinable = self.0.0.lock().unwrap();
        while !*joinable {
            joinable = self.0.1.wait(joinable).unwrap();
        }

        *joinable = false;
    }

    pub fn signal(&self) {
        let mut joinable = self.0.0.lock().unwrap();
        *joinable = true;
        self.0.1.notify_one();
    }
}

impl Default for Notifier {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Notifier {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
