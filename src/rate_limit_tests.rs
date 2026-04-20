#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::rate_limit::RateLimiter;
    use crate::rate_limit_config::RateLimitConfig;
    use crate::rate_limit_handler::RateLimitHandler;

    #[test]
    fn test_initial_burst_allows_multiple_acquires() {
        let config = RateLimitConfig::new(1.0, 3);
        let mut limiter = RateLimiter::new(config);
        assert!(limiter.try_acquire().is_ok());
        assert!(limiter.try_acquire().is_ok());
        assert!(limiter.try_acquire().is_ok());
        assert!(limiter.try_acquire().is_err());
    }

    #[test]
    fn test_rate_limited_returns_retry_after() {
        let config = RateLimitConfig::new(1.0, 1);
        let mut limiter = RateLimiter::new(config);
        assert!(limiter.try_acquire().is_ok());
        let result = limiter.try_acquire();
        assert!(result.is_err());
        let wait = limiter.time_until_next_token();
        assert!(wait > Duration::ZERO);
        assert!(wait <= Duration::from_secs(1));
    }

    #[test]
    fn test_disabled_config_always_allows() {
        let config = RateLimitConfig::disabled();
        let handler = RateLimitHandler::new(config);
        for _ in 0..100 {
            assert!(handler.check_restart().is_ok());
        }
    }

    #[test]
    fn test_config_validation_rejects_zero_rate() {
        let config = RateLimitConfig::new(0.0, 1);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_rejects_zero_burst() {
        let config = RateLimitConfig::new(1.0, 0);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_available_tokens_decreases_on_acquire() {
        let config = RateLimitConfig::new(1.0, 5);
        let mut limiter = RateLimiter::new(config);
        let before = limiter.available_tokens();
        limiter.try_acquire().unwrap();
        let after = limiter.available_tokens();
        assert!(after < before);
    }

    #[test]
    fn test_handler_clone_shares_state() {
        let config = RateLimitConfig::new(1.0, 1);
        let handler = RateLimitHandler::new(config);
        let handler2 = handler.clone();
        assert!(handler.check_restart().is_ok());
        assert!(handler2.check_restart().is_err());
    }

    #[test]
    fn test_default_config() {
        let config = RateLimitConfig::default();
        assert_eq!(config.burst, 5);
        assert_eq!(config.rate_per_second, 1.0);
        assert!(config.enabled);
        assert!(config.validate().is_ok());
    }
}
