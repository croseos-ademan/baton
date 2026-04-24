use std::time::{Duration, Instant};
use crate::backoff::Backoff;
use crate::error::BatonError;

/// Tracks process restart attempts and enforces restart policy.
#[derive(Debug)]
pub struct Supervisor {
    max_restarts: u32,
    restart_window: Duration,
    restart_count: u32,
    window_start: Instant,
    backoff: Backoff,
}

#[derive(Debug, PartialEq)]
pub enum SupervisorDecision {
    Restart,
    GiveUp,
}

impl Supervisor {
    pub fn new(max_restarts: u32, restart_window: Duration, backoff: Backoff) -> Self {
        Self {
            max_restarts,
            restart_window,
            restart_count: 0,
            window_start: Instant::now(),
            backoff,
        }
    }

    /// Called when a child process exits unexpectedly.
    /// Returns whether the supervisor should restart or give up.
    pub fn record_exit(&mut self) -> Result<SupervisorDecision, BatonError> {
        let now = Instant::now();
        if now.duration_since(self.window_start) > self.restart_window {
            self.restart_count = 0;
            self.window_start = now;
        }

        self.restart_count += 1;

        if self.restart_count > self.max_restarts {
            return Ok(SupervisorDecision::GiveUp);
        }

        Ok(SupervisorDecision::Restart)
    }

    /// Returns the delay to wait before the next restart attempt.
    pub fn next_delay(&mut self) -> Duration {
        self.backoff.next_delay()
    }

    /// Resets the backoff after a successful stable run.
    pub fn reset_backoff(&mut self) {
        self.backoff.reset();
    }

    pub fn restart_count(&self) -> u32 {
        self.restart_count
    }
}
