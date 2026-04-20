use std::time::Duration;

/// Configuration for the Throttle.
#[derive(Debug, Clone)]
pub struct ThrottleConfig {
    /// Maximum number of events allowed within the window.
    pub max_events: usize,
    /// The sliding time window over which events are counted.
    pub window: Duration,
}

impl ThrottleConfig {
    pub fn new(max_events: usize, window: Duration) -> Self {
        Self { max_events, window }
    }

    /// Convenience: allow at most `n` events per second.
    pub fn per_second(n: usize) -> Self {
        Self::new(n, Duration::from_secs(1))
    }

    /// Convenience: allow at most `n` events per minute.
    pub fn per_minute(n: usize) -> Self {
        Self::new(n, Duration::from_secs(60))
    }
}

impl Default for ThrottleConfig {
    fn default() -> Self {
        Self::new(10, Duration::from_secs(1))
    }
}
