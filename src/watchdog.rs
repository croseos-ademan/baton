use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crate::error::BatonError;

/// Watchdog monitors a child process and triggers action if it becomes unresponsive.
#[derive(Debug)]
pub struct Watchdog {
    interval: Duration,
    timeout: Duration,
    last_heartbeat: Instant,
    enabled: bool,
}

impl Watchdog {
    pub fn new(interval: Duration, timeout: Duration) -> Self {
        Watchdog {
            interval,
            timeout,
            last_heartbeat: Instant::now(),
            enabled: true,
        }
    }

    pub fn heartbeat(&mut self) {
        self.last_heartbeat = Instant::now();
    }

    pub fn is_alive(&self) -> bool {
        if !self.enabled {
            return true;
        }
        self.last_heartbeat.elapsed() < self.timeout
    }

    pub fn elapsed_since_heartbeat(&self) -> Duration {
        self.last_heartbeat.elapsed()
    }

    pub fn interval(&self) -> Duration {
        self.interval
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn check(&self) -> Result<(), BatonError> {
        if !self.is_alive() {
            return Err(BatonError::Timeout(format!(
                "watchdog timeout: no heartbeat for {:?}",
                self.elapsed_since_heartbeat()
            )));
        }
        Ok(())
    }
}

/// Shared watchdog handle for use across threads.
#[derive(Clone, Debug)]
pub struct WatchdogHandle {
    alive: Arc<AtomicBool>,
}

impl WatchdogHandle {
    pub fn new() -> Self {
        WatchdogHandle {
            alive: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn signal_alive(&self) {
        self.alive.store(true, Ordering::SeqCst);
    }

    pub fn is_alive(&self) -> bool {
        self.alive.load(Ordering::SeqCst)
    }

    pub fn reset(&self) {
        self.alive.store(false, Ordering::SeqCst);
    }
}

impl Default for WatchdogHandle {
    fn default() -> Self {
        Self::new()
    }
}
