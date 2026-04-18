use std::time::{Duration, Instant};
use crate::error::BatonError;

/// Configuration and logic for draining in-flight requests before handoff.
#[derive(Debug, Clone)]
pub struct DrainConfig {
    /// Maximum time to wait for connections to drain.
    pub timeout: Duration,
    /// Polling interval when checking drain completion.
    pub poll_interval: Duration,
}

impl Default for DrainConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            poll_interval: Duration::from_millis(100),
        }
    }
}

/// Tracks the drain state of the old process.
#[derive(Debug, Clone, PartialEq)]
pub enum DrainState {
    Draining,
    Complete,
    TimedOut,
}

/// Waits for active connection count to reach zero or timeout.
pub fn wait_for_drain<F>(config: &DrainConfig, active_connections: F) -> DrainState
where
    F: Fn() -> usize,
{
    let start = Instant::now();
    loop {
        if active_connections() == 0 {
            return DrainState::Complete;
        }
        if start.elapsed() >= config.timeout {
            return DrainState::TimedOut;
        }
        std::thread::sleep(config.poll_interval);
    }
}

/// Returns an error if drain timed out, otherwise Ok.
pub fn enforce_drain<F>(config: &DrainConfig, active_connections: F) -> Result<(), BatonError>
where
    F: Fn() -> usize,
{
    match wait_for_drain(config, active_connections) {
        DrainState::Complete => Ok(()),
        DrainState::TimedOut => Err(BatonError::Timeout(
            "drain timed out waiting for connections to close".into(),
        )),
        DrainState::Draining => unreachable!(),
    }
}
