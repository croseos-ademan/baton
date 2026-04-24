use crate::state::{ProcessState, StateTracker};
use std::sync::{Arc, Mutex};

pub type StateCallback = Box<dyn Fn(&ProcessState, &ProcessState) + Send + 'static>;

pub struct StateObserver {
    tracker: StateTracker,
    callbacks: Arc<Mutex<Vec<StateCallback>>>,
    last_seen: Arc<Mutex<ProcessState>>,
}

impl StateObserver {
    pub fn new(tracker: StateTracker) -> Self {
        let initial = tracker.current().state.clone();
        StateObserver {
            tracker,
            callbacks: Arc::new(Mutex::new(Vec::new())),
            last_seen: Arc::new(Mutex::new(initial)),
        }
    }

    pub fn on_transition<F>(&self, f: F)
    where
        F: Fn(&ProcessState, &ProcessState) + Send + 'static,
    {
        self.callbacks.lock().unwrap().push(Box::new(f));
    }

    pub fn poll(&self) {
        let current = self.tracker.current().state.clone();
        let mut last = self.last_seen.lock().unwrap();
        if current != *last {
            let callbacks = self.callbacks.lock().unwrap();
            for cb in callbacks.iter() {
                cb(&last, &current);
            }
            *last = current;
        }
    }

    pub fn wait_for(&self, target: ProcessState, max_polls: usize) -> bool {
        for _ in 0..max_polls {
            self.poll();
            if self.tracker.is(&target) {
                return true;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        false
    }
}
