use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessState {
    Starting,
    Running,
    Draining,
    Stopping,
    Stopped,
    Failed(String),
}

impl fmt::Display for ProcessState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessState::Starting => write!(f, "starting"),
            ProcessState::Running => write!(f, "running"),
            ProcessState::Draining => write!(f, "draining"),
            ProcessState::Stopping => write!(f, "stopping"),
            ProcessState::Stopped => write!(f, "stopped"),
            ProcessState::Failed(reason) => write!(f, "failed: {}", reason),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StateEntry {
    pub state: ProcessState,
    pub entered_at: Instant,
}

impl StateEntry {
    pub fn new(state: ProcessState) -> Self {
        StateEntry {
            state,
            entered_at: Instant::now(),
        }
    }

    pub fn elapsed_secs(&self) -> u64 {
        self.entered_at.elapsed().as_secs()
    }
}

#[derive(Debug, Clone)]
pub struct StateTracker {
    inner: Arc<Mutex<StateEntry>>,
}

impl StateTracker {
    pub fn new(initial: ProcessState) -> Self {
        StateTracker {
            inner: Arc::new(Mutex::new(StateEntry::new(initial))),
        }
    }

    pub fn transition(&self, next: ProcessState) {
        let mut entry = self.inner.lock().unwrap();
        *entry = StateEntry::new(next);
    }

    pub fn current(&self) -> StateEntry {
        self.inner.lock().unwrap().clone()
    }

    pub fn is(&self, state: &ProcessState) -> bool {
        self.inner.lock().unwrap().state == *state
    }
}
