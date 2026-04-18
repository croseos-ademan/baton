#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread;
    use crate::timeout::{Timeout, poll_until};
    use crate::timeout_config::{TimeoutConfig, TimeoutConfigBuilder};

    #[test]
    fn test_timeout_not_expired_immediately() {
        let t = Timeout::from_secs(10);
        assert!(!t.is_expired());
    }

    #[test]
    fn test_timeout_expires() {
        let t = Timeout::new(Duration::from_millis(10));
        thread::sleep(Duration::from_millis(20));
        assert!(t.is_expired());
    }

    #[test]
    fn test_timeout_check_ok() {
        let t = Timeout::from_secs(10);
        assert!(t.check().is_ok());
    }

    #[test]
    fn test_timeout_check_err() {
        let t = Timeout::new(Duration::from_millis(10));
        thread::sleep(Duration::from_millis(20));
        assert!(t.check().is_err());
    }

    #[test]
    fn test_poll_until_succeeds() {
        let t = Timeout::from_secs(5);
        let mut count = 0;
        let result = poll_until(&t, Duration::from_millis(1), || {
            count += 1;
            count >= 3
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_poll_until_times_out() {
        let t = Timeout::new(Duration::from_millis(20));
        let result = poll_until(&t, Duration::from_millis(5), || false);
        assert!(result.is_err());
    }

    #[test]
    fn test_timeout_config_defaults() {
        let cfg = TimeoutConfig::default();
        assert_eq!(cfg.ready_timeout, Duration::from_secs(30));
        assert_eq!(cfg.shutdown_timeout, Duration::from_secs(10));
        assert_eq!(cfg.lock_timeout, Duration::from_secs(5));
    }

    #[test]
    fn test_timeout_config_builder() {
        let cfg = TimeoutConfigBuilder::new()
            .ready_timeout(60)
            .shutdown_timeout(20)
            .build();
        assert_eq!(cfg.ready_timeout, Duration::from_secs(60));
        assert_eq!(cfg.shutdown_timeout, Duration::from_secs(20));
        assert_eq!(cfg.lock_timeout, Duration::from_secs(5));
    }
}
