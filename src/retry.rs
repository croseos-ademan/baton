use std::time::Duration;
use crate::backoff::Backoff;

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub backoff: Backoff,
}

impl RetryConfig {
    pub fn new(max_attempts: u32, backoff: Backoff) -> Self {
        Self { max_attempts, backoff }
    }

    pub fn default() -> Self {
        Self {
            max_attempts: 3,
            backoff: Backoff::new(Duration::from_millis(100), Duration::from_secs(5), 2.0),
        }
    }
}

pub struct Retry {
    config: RetryConfig,
    attempts: u32,
}

impl Retry {
    pub fn new(config: RetryConfig) -> Self {
        Self { config, attempts: 0 }
    }

    pub fn attempt<F, T, E>(&mut self, mut f: F) -> Result<T, E>
    where
        F: FnMut(u32) -> Result<T, E>,
        E: std::fmt::Debug,
    {
        loop {
            self.attempts += 1;
            match f(self.attempts) {
                Ok(val) => return Ok(val),
                Err(e) => {
                    if self.attempts >= self.config.max_attempts {
                        return Err(e);
                    }
                    let delay = self.config.backoff.next_delay(self.attempts);
                    std::thread::sleep(delay);
                }
            }
        }
    }

    pub fn attempts(&self) -> u32 {
        self.attempts
    }

    pub fn exhausted(&self) -> bool {
        self.attempts >= self.config.max_attempts
    }
}
