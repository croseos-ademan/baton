use std::time::{Duration, Instant};
use crate::throttle_config::ThrottleConfig;

/// Throttle limits how frequently an action can be performed
/// within a sliding time window.
pub struct Throttle {
    config: ThrottleConfig,
    timestamps: Vec<Instant>,
}

impl Throttle {
    pub fn new(config: ThrottleConfig) -> Self {
        Self {
            config,
            timestamps: Vec::new(),
        }
    }

    /// Returns true if the action is allowed under the current throttle policy.
    pub fn allow(&mut self) -> bool {
        let now = Instant::now();
        self.evict_old(now);

        if self.timestamps.len() < self.config.max_events {
            self.timestamps.push(now);
            true
        } else {
            false
        }
    }

    /// Returns the duration to wait before the next action is allowed,
    /// or None if an action can be taken immediately.
    pub fn wait_time(&mut self) -> Option<Duration> {
        let now = Instant::now();
        self.evict_old(now);

        if self.timestamps.len() < self.config.max_events {
            None
        } else {
            // Oldest timestamp + window = when a slot opens
            self.timestamps.first().map(|oldest| {
                let expires = *oldest + self.config.window;
                if expires > now {
                    expires - now
                } else {
                    Duration::ZERO
                }
            })
        }
    }

    /// Resets the throttle state.
    pub fn reset(&mut self) {
        self.timestamps.clear();
    }

    fn evict_old(&mut self, now: Instant) {
        let cutoff = now.checked_sub(self.config.window).unwrap_or(now);
        self.timestamps.retain(|t| *t > cutoff);
    }
}
