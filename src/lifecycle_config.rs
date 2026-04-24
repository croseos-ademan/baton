use std::time::Duration;

#[derive(Debug, Clone)]
pub struct LifecycleConfig {
    pub startup_timeout: Duration,
    pub drain_timeout: Duration,
    pub shutdown_timeout: Duration,
    pub allow_restart_on_failure: bool,
    pub max_restarts: u32,
}

impl LifecycleConfig {
    pub fn builder() -> LifecycleConfigBuilder {
        LifecycleConfigBuilder::default()
    }
}

impl Default for LifecycleConfig {
    fn default() -> Self {
        Self {
            startup_timeout: Duration::from_secs(30),
            drain_timeout: Duration::from_secs(60),
            shutdown_timeout: Duration::from_secs(10),
            allow_restart_on_failure: true,
            max_restarts: 3,
        }
    }
}

#[derive(Default)]
pub struct LifecycleConfigBuilder {
    startup_timeout: Option<Duration>,
    drain_timeout: Option<Duration>,
    shutdown_timeout: Option<Duration>,
    allow_restart_on_failure: Option<bool>,
    max_restarts: Option<u32>,
}

impl LifecycleConfigBuilder {
    pub fn startup_timeout(mut self, d: Duration) -> Self {
        self.startup_timeout = Some(d);
        self
    }

    pub fn drain_timeout(mut self, d: Duration) -> Self {
        self.drain_timeout = Some(d);
        self
    }

    pub fn shutdown_timeout(mut self, d: Duration) -> Self {
        self.shutdown_timeout = Some(d);
        self
    }

    pub fn allow_restart_on_failure(mut self, v: bool) -> Self {
        self.allow_restart_on_failure = Some(v);
        self
    }

    pub fn max_restarts(mut self, n: u32) -> Self {
        self.max_restarts = Some(n);
        self
    }

    pub fn build(self) -> LifecycleConfig {
        let defaults = LifecycleConfig::default();
        LifecycleConfig {
            startup_timeout: self.startup_timeout.unwrap_or(defaults.startup_timeout),
            drain_timeout: self.drain_timeout.unwrap_or(defaults.drain_timeout),
            shutdown_timeout: self.shutdown_timeout.unwrap_or(defaults.shutdown_timeout),
            allow_restart_on_failure: self.allow_restart_on_failure.unwrap_or(defaults.allow_restart_on_failure),
            max_restarts: self.max_restarts.unwrap_or(defaults.max_restarts),
        }
    }
}
