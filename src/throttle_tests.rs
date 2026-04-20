#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::throttle::Throttle;
    use crate::throttle_config::ThrottleConfig;

    fn make_throttle(max: usize, window_ms: u64) -> Throttle {
        Throttle::new(ThrottleConfig::new(max, Duration::from_millis(window_ms)))
    }

    #[test]
    fn allows_up_to_max_events() {
        let mut t = make_throttle(3, 1000);
        assert!(t.allow());
        assert!(t.allow());
        assert!(t.allow());
        assert!(!t.allow());
    }

    #[test]
    fn reset_clears_state() {
        let mut t = make_throttle(2, 1000);
        assert!(t.allow());
        assert!(t.allow());
        assert!(!t.allow());
        t.reset();
        assert!(t.allow());
    }

    #[test]
    fn wait_time_none_when_under_limit() {
        let mut t = make_throttle(5, 1000);
        assert!(t.wait_time().is_none());
        t.allow();
        assert!(t.wait_time().is_none());
    }

    #[test]
    fn wait_time_some_when_at_limit() {
        let mut t = make_throttle(1, 1000);
        t.allow();
        let wait = t.wait_time();
        assert!(wait.is_some());
        assert!(wait.unwrap() <= Duration::from_millis(1000));
    }

    #[test]
    fn per_second_constructor() {
        use crate::throttle_config::ThrottleConfig;
        let cfg = ThrottleConfig::per_second(5);
        assert_eq!(cfg.max_events, 5);
        assert_eq!(cfg.window, Duration::from_secs(1));
    }

    #[test]
    fn per_minute_constructor() {
        use crate::throttle_config::ThrottleConfig;
        let cfg = ThrottleConfig::per_minute(100);
        assert_eq!(cfg.max_events, 100);
        assert_eq!(cfg.window, Duration::from_secs(60));
    }

    #[test]
    fn default_config() {
        use crate::throttle_config::ThrottleConfig;
        let cfg = ThrottleConfig::default();
        assert_eq!(cfg.max_events, 10);
        assert_eq!(cfg.window, Duration::from_secs(1));
    }
}
