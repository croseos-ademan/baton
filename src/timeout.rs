use std::time::{Duration, Instant};
use crate::error::BatonError;

/// Tracks a deadline and checks if it has been exceeded.
#[derive(Debug, Clone)]
pub struct Timeout {
    deadline: Instant,
    duration: Duration,
}

impl Timeout {
    pub fn new(duration: Duration) -> Self {
        Self {
            deadline: Instant::now() + duration,
            duration,
        }
    }

    pub fn from_secs(secs: u64) -> Self {
        Self::new(Duration::from_secs(secs))
    }

    pub fn is_expired(&self) -> bool {
        Instant::now() >= self.deadline
    }

    pub fn remaining(&self) -> Duration {
        self.deadline.saturating_duration_since(Instant::now())
    }

    pub fn check(&self) -> Result<(), BatonError> {
        if self.is_expired() {
            Err(BatonError::Timeout(self.duration))
        } else {
            Ok(())
        }
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }
}

/// Polls a condition function until it returns true or the timeout expires.
pub fn poll_until<F>(timeout: &Timeout, interval: Duration, mut condition: F) -> Result<(), BatonError>
where
    F: FnMut() -> bool,
{
    loop {
        if condition() {
            return Ok(());
        }
        timeout.check()?;
        std::thread::sleep(interval.min(timeout.remaining()));
    }
}
