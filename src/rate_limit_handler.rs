use std::sync::{Arc, Mutex};
use crate::rate_limit::RateLimiter;
use crate::rate_limit_config::RateLimitConfig;
use crate::error::BatonError;

/// Thread-safe wrapper around RateLimiter for use in async/multi-threaded contexts.
pub struct RateLimitHandler {
    limiter: Arc<Mutex<RateLimiter>>,
}

impl RateLimitHandler {
    pub fn new(config: RateLimitConfig) -> Self {
        RateLimitHandler {
            limiter: Arc::new(Mutex::new(RateLimiter::new(config))),
        }
    }

    /// Check and consume a token for a restart attempt.
    pub fn check_restart(&self) -> Result<(), BatonError> {
        let mut limiter = self.limiter.lock().map_err(|_| BatonError::LockPoisoned)?;
        if !limiter.config().enabled {
            return Ok(());
        }
        limiter.try_acquire()
    }

    /// Returns available tokens without consuming.
    pub fn available_tokens(&self) -> f64 {
        let mut limiter = self.limiter.lock().unwrap_or_else(|e| e.into_inner());
        limiter.available_tokens()
    }

    /// Clone the inner Arc for sharing across threads.
    pub fn clone_handle(&self) -> Self {
        RateLimitHandler {
            limiter: Arc::clone(&self.limiter),
        }
    }
}

impl Clone for RateLimitHandler {
    fn clone(&self) -> Self {
        self.clone_handle()
    }
}
