use std::time::Duration;

#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub success_threshold: u32,
    pub reset_timeout: Duration,
}

impl CircuitBreakerConfig {
    pub fn new(failure_threshold: u32, success_threshold: u32, reset_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            success_threshold,
            reset_timeout,
        }
    }

    pub fn with_failure_threshold(mut self, threshold: u32) -> Self {
        self.failure_threshold = threshold;
        self
    }

    pub fn with_success_threshold(mut self, threshold: u32) -> Self {
        self.success_threshold = threshold;
        self
    }

    pub fn with_reset_timeout(mut self, timeout: Duration) -> Self {
        self.reset_timeout = timeout;
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.failure_threshold == 0 {
            return Err("failure_threshold must be greater than 0".to_string());
        }
        if self.success_threshold == 0 {
            return Err("success_threshold must be greater than 0".to_string());
        }
        if self.reset_timeout.is_zero() {
            return Err("reset_timeout must be greater than zero".to_string());
        }
        Ok(())
    }
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 2,
            reset_timeout: Duration::from_secs(30),
        }
    }
}
