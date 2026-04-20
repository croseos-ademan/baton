/// Configuration for the token bucket rate limiter.
#[derive(Debug, Clone, PartialEq)]
pub struct RateLimitConfig {
    /// Tokens added per second.
    pub rate_per_second: f64,
    /// Maximum burst size (initial and max token count).
    pub burst: u32,
    /// Whether rate limiting is enabled.
    pub enabled: bool,
}

impl RateLimitConfig {
    pub fn new(rate_per_second: f64, burst: u32) -> Self {
        RateLimitConfig {
            rate_per_second,
            burst,
            enabled: true,
        }
    }

    pub fn disabled() -> Self {
        RateLimitConfig {
            rate_per_second: f64::MAX,
            burst: u32::MAX,
            enabled: false,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.rate_per_second <= 0.0 {
            return Err("rate_per_second must be positive".to_string());
        }
        if self.burst == 0 {
            return Err("burst must be at least 1".to_string());
        }
        Ok(())
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        RateLimitConfig::new(1.0, 5)
    }
}
