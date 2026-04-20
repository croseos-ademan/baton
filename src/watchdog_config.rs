use std::time::Duration;

/// Configuration for the watchdog subsystem.
#[derive(Debug, Clone)]
pub struct WatchdogConfig {
    /// How often the watchdog checks for a heartbeat.
    pub check_interval: Duration,
    /// How long without a heartbeat before the process is considered dead.
    pub timeout: Duration,
    /// Whether the watchdog is enabled at all.
    pub enabled: bool,
    /// Number of missed heartbeats before triggering a restart.
    pub miss_threshold: u32,
}

impl WatchdogConfig {
    pub fn new(check_interval: Duration, timeout: Duration) -> Self {
        WatchdogConfig {
            check_interval,
            timeout,
            enabled: true,
            miss_threshold: 3,
        }
    }

    pub fn disabled() -> Self {
        WatchdogConfig {
            check_interval: Duration::from_secs(5),
            timeout: Duration::from_secs(30),
            enabled: false,
            miss_threshold: 3,
        }
    }

    pub fn with_miss_threshold(mut self, threshold: u32) -> Self {
        self.miss_threshold = threshold;
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.check_interval.is_zero() {
            return Err("watchdog check_interval must be greater than zero".to_string());
        }
        if self.timeout < self.check_interval {
            return Err("watchdog timeout must be >= check_interval".to_string());
        }
        if self.miss_threshold == 0 {
            return Err("watchdog miss_threshold must be at least 1".to_string());
        }
        Ok(())
    }
}

impl Default for WatchdogConfig {
    fn default() -> Self {
        WatchdogConfig::new(
            Duration::from_secs(5),
            Duration::from_secs(30),
        )
    }
}
