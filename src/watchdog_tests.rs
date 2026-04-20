#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread;
    use crate::watchdog::{Watchdog, WatchdogHandle};
    use crate::watchdog_config::WatchdogConfig;

    #[test]
    fn test_watchdog_alive_after_heartbeat() {
        let mut wd = Watchdog::new(
            Duration::from_millis(100),
            Duration::from_s),
        );
        wd.heartbeat();
        assert!(wd.is_alive());
        assert!(wd.check().is_ok());
    }

    #[test]
    fn test_watchdog_dead_after_timeout() {
        let mut wd = Watchdog::new(
            Duration::from_millis(10),
            Duration::from_millis(50),
        );
        wd.heartbeat();
        thread::sleep(Duration::from_millis(100));
        assert!(!wd.is_alive());
        assert!(wd.check().is_err());
    }

    #[test]
    fn test_watchdog_disabled_always_alive() {
        let mut wd = Watchdog::new(
            Duration::from_millis(10),
            Duration::from_millis(1),
        );
        wd.disable();
        thread::sleep(Duration::from_millis(50));
        assert!(wd.is_alive());
        assert!(wd.check().is_ok());
    }

    #[test]
    fn test_watchdog_handle_signal_and_reset() {
        let handle = WatchdogHandle::new();
        handle.reset();
        assert!(!handle.is_alive());
        handle.signal_alive();
        assert!(handle.is_alive());
    }

    #[test]
    fn test_watchdog_config_validation_ok() {
        let cfg = WatchdogConfig::new(
            Duration::from_secs(5),
            Duration::from_secs(30),
        );
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_watchdog_config_timeout_less_than_interval_fails() {
        let cfg = WatchdogConfig::new(
            Duration::from_secs(30),
            Duration::from_secs(5),
        );
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_watchdog_config_zero_interval_fails() {
        let cfg = WatchdogConfig::new(
            Duration::from_secs(0),
            Duration::from_secs(10),
        );
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_watchdog_config_disabled() {
        let cfg = WatchdogConfig::disabled();
        assert!(!cfg.enabled);
        assert!(cfg.validate().is_ok());
    }
}
