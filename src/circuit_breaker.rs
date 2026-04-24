use std::time::{Duration, Instant};
use crate::circuit_breaker_config::CircuitBreakerConfig;
use crate::circuit_breaker_error::CircuitBreakerError;

#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug)]
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    state: CircuitState,
    failure_count: u32,
    last_failure: Option<Instant>,
    success_count: u32,
}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: CircuitState::Closed,
            failure_count: 0,
            last_failure: None,
            success_count: 0,
        }
    }

    pub fn state(&self) -> &CircuitState {
        &self.state
    }

    pub fn call<F, T>(&mut self, f: F) -> Result<T, CircuitBreakerError>
    where
        F: FnOnce() -> Result<T, String>,
    {
        self.transition();

        if self.state == CircuitState::Open {
            return Err(CircuitBreakerError::CircuitOpen);
        }

        match f() {
            Ok(val) => {
                self.on_success();
                Ok(val)
            }
            Err(e) => {
                self.on_failure();
                Err(CircuitBreakerError::CallFailed(e))
            }
        }
    }

    fn transition(&mut self) {
        if self.state == CircuitState::Open {
            if let Some(last) = self.last_failure {
                if last.elapsed() >= self.config.reset_timeout {
                    self.state = CircuitState::HalfOpen;
                    self.success_count = 0;
                }
            }
        }
    }

    fn on_success(&mut self) {
        match self.state {
            CircuitState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.config.success_threshold {
                    self.state = CircuitState::Closed;
                    self.failure_count = 0;
                }
            }
            CircuitState::Closed => {
                self.failure_count = 0;
            }
            _ => {}
        }
    }

    fn on_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure = Some(Instant::now());
        if self.failure_count >= self.config.failure_threshold {
            self.state = CircuitState::Open;
        }
    }

    pub fn reset(&mut self) {
        self.state = CircuitState::Closed;
        self.failure_count = 0;
        self.success_count = 0;
        self.last_failure = None;
    }
}
