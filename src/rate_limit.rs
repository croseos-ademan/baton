use std::time::{Duration, Instant};
use crate::rate_limit_config::RateLimitConfig;
use crate::error::BatonError;

/// Token bucket rate limiter for controlling restart frequency.
pub struct RateLimiter {
    config: RateLimitConfig,
    tokens: f64,
    last_refill: Instant,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        RateLimiter {
            tokens: config.burst as f64,
            last_refill: Instant::now(),
            config,
        }
    }

    /// Refill tokens based on elapsed time.
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        let new_tokens = elapsed * self.config.rate_per_second;
        self.tokens = (self.tokens + new_tokens).min(self.config.burst as f64);
        self.last_refill = now;
    }

    /// Attempt to consume one token. Returns Ok if allowed, Err if rate limited.
    pub fn try_acquire(&mut self) -> Result<(), BatonError> {
        self.refill();
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            Ok(())
        } else {
            Err(BatonError::RateLimited {
                retry_after: self.time_until_next_token(),
            })
        }
    }

    /// Returns how long until the next token is available.
    pub fn time_until_next_token(&self) -> Duration {
        if self.tokens >= 1.0 {
            Duration::ZERO
        } else {
            let deficit = 1.0 - self.tokens;
            let secs = deficit / self.config.rate_per_second;
            Duration::from_secs_f64(secs)
        }
    }

    /// Returns current token count (for metrics/diagnostics).
    pub fn available_tokens(&mut self) -> f64 {
        self.refill();
        self.tokens
    }

    pub fn config(&self) -> &RateLimitConfig {
        &self.config
    }
}
