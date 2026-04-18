use std::time::Duration;

/// Configuration for various timeout values used during process handoff.
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    /// How long to wait for the new process to signal readiness.
    pub ready_timeout: Duration,
    /// How long to wait for the old process to exit after handoff.
    pub shutdown_timeout: Duration,
    /// How long to wait when acquiring the handoff lock.
    pub lock_timeout: Duration,
}

impl TimeoutConfig {
    pub fn new(ready_secs: u64, shutdown_secs: u64, lock_secs: u64) -> Self {
        Self {
            ready_timeout: Duration::from_secs(ready_secs),
            shutdown_timeout: Duration::from_secs(shutdown_secs),
            lock_timeout: Duration::from_secs(lock_secs),
        }
    }
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self::new(30, 10, 5)
    }
}

/// Builder for TimeoutConfig.
#[derive(Default)]
pub struct TimeoutConfigBuilder {
    ready_secs: Option<u64>,
    shutdown_secs: Option<u64>,
    lock_secs: Option<u64>,
}

impl TimeoutConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ready_timeout(mut self, secs: u64) -> Self {
        self.ready_secs = Some(secs);
        self
    }

    pub fn shutdown_timeout(mut self, secs: u64) -> Self {
        self.shutdown_secs = Some(secs);
        self
    }

    pub fn lock_timeout(mut self, secs: u64) -> Self {
        self.lock_secs = Some(secs);
        self
    }

    pub fn build(self) -> TimeoutConfig {
        let defaults = TimeoutConfig::default();
        TimeoutConfig {
            ready_timeout: self.ready_secs.map(Duration::from_secs).unwrap_or(defaults.ready_timeout),
            shutdown_timeout: self.shutdown_secs.map(Duration::from_secs).unwrap_or(defaults.shutdown_timeout),
            lock_timeout: self.lock_secs.map(Duration::from_secs).unwrap_or(defaults.lock_timeout),
        }
    }
}
