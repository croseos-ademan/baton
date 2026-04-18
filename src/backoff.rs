use std::time::Duration;

/// Exponential backoff configuration for retry logic.
#[derive(Debug, Clone)]
pub struct Backoff {
    initial: Duration,
    max: Duration,
    multiplier: f64,
    current: Duration,
    attempts: u32,
}

impl Backoff {
    pub fn new(initial: Duration, max: Duration, multiplier: f64) -> Self {
        Self {
            initial,
            max,
            multiplier,
            current: initial,
            attempts: 0,
        }
    }

    pub fn default_config() -> Self {
        Self::new(
            Duration::from_millis(100),
            Duration::from_secs(10),
            2.0,
        )
    }

    /// Returns the next wait duration and advances state.
    pub fn next(&mut self) -> Duration {
        let wait = self.current;
        self.attempts += 1;
        let next_nanos = (self.current.as_nanos() as f64 * self.multiplier) as u128;
        self.current = Duration::from_nanos(next_nanos.min(self.max.as_nanos()) as u64);
        wait
    }

    pub fn reset(&mut self) {
        self.current = self.initial;
        self.attempts = 0;
    }

    pub fn attempts(&self) -> u32 {
        self.attempts
    }

    pub fn is_maxed(&self) -> bool {
        self.current >= self.max
    }
}
