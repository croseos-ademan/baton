#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::circuit_breaker::{CircuitBreaker, CircuitState};
    use crate::circuit_breaker_config::CircuitBreakerConfig;
    use crate::circuit_breaker_error::CircuitBreakerError;

    fn make_breaker(failures: u32, successes: u32, timeout_ms: u64) -> CircuitBreaker {
        let config = CircuitBreakerConfig::new(
            failures,
            successes,
            Duration::from_millis(timeout_ms),
        );
        CircuitBreaker::new(config)
    }

    #[test]
    fn test_initial_state_is_closed() {
        let cb = make_breaker(3, 2, 100);
        assert_eq!(cb.state(), &CircuitState::Closed);
    }

    #[test]
    fn test_opens_after_threshold_failures() {
        let mut cb = make_breaker(3, 2, 100);
        for _ in 0..3 {
            let _ = cb.call(|| Err::<(), String>("err".to_string()));
        }
        assert_eq!(cb.state(), &CircuitState::Open);
    }

    #[test]
    fn test_rejects_calls_when_open() {
        let mut cb = make_breaker(1, 2, 100);
        let _ = cb.call(|| Err::<(), String>("err".to_string()));
        let result = cb.call(|| Ok::<(), String>(()));
        assert_eq!(result, Err(CircuitBreakerError::CircuitOpen));
    }

    #[test]
    fn test_transitions_to_half_open_after_timeout() {
        let mut cb = make_breaker(1, 1, 10);
        let _ = cb.call(|| Err::<(), String>("err".to_string()));
        std::thread::sleep(Duration::from_millis(20));
        let _ = cb.call(|| Ok::<(), String>(()));
        assert_eq!(cb.state(), &CircuitState::Closed);
    }

    #[test]
    fn test_closes_after_success_threshold_in_half_open() {
        let mut cb = make_breaker(1, 2, 10);
        let _ = cb.call(|| Err::<(), String>("err".to_string()));
        std::thread::sleep(Duration::from_millis(20));
        let _ = cb.call(|| Ok::<(), String>(()));
        let _ = cb.call(|| Ok::<(), String>(()));
        assert_eq!(cb.state(), &CircuitState::Closed);
    }

    #[test]
    fn test_reset_restores_closed_state() {
        let mut cb = make_breaker(1, 2, 100);
        let _ = cb.call(|| Err::<(), String>("err".to_string()));
        assert_eq!(cb.state(), &CircuitState::Open);
        cb.reset();
        assert_eq!(cb.state(), &CircuitState::Closed);
    }

    #[test]
    fn test_config_default_values() {
        let config = crate::circuit_breaker_config::CircuitBreakerConfig::default();
        assert_eq!(config.failure_threshold, 5);
        assert_eq!(config.success_threshold, 2);
        assert_eq!(config.reset_timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_config_validation_rejects_zero_failure_threshold() {
        let config = crate::circuit_breaker_config::CircuitBreakerConfig::new(
            0, 1, Duration::from_secs(1),
        );
        assert!(config.validate().is_err());
    }
}
