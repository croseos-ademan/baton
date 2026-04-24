use std::time::Duration;

#[derive(Debug, Clone)]
pub struct StateConfig {
    pub startup_timeout: Duration,
    pub drain_timeout: Duration,
    pub stop_timeout: Duration,
    pub track_history: bool,
}

impl Default for StateConfig {
    fn default() -> Self {
        StateConfig {
            startup_timeout: Duration::from_secs(30),
            drain_timeout: Duration::from_secs(60),
            stop_timeout: Duration::from_secs(15),
            track_history: false,
        }
    }
}

impl StateConfig {
    pub fn builder() -> StateConfigBuilder {
        StateConfigBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct StateConfigBuilder {
    startup_timeout: Option<Duration>,
    drain_timeout: Option<Duration>,
    stop_timeout: Option<Duration>,
    track_history: Option<bool>,
}

impl StateConfigBuilder {
    pub fn startup_timeout(mut self, d: Duration) -> Self {
        self.startup_timeout = Some(d);
        self
    }

    pub fn drain_timeout(mut self, d: Duration) -> Self {
        self.drain_timeout = Some(d);
        self
    }

    pub fn stop_timeout(mut self, d: Duration) -> Self {
        self.stop_timeout = Some(d);
        self
    }

    pub fn track_history(mut self, v: bool) -> Self {
        self.track_history = Some(v);
        self
    }

    pub fn build(self) -> StateConfig {
        let defaults = StateConfig::default();
        StateConfig {
            startup_timeout: self.startup_timeout.unwrap_or(defaults.startup_timeout),
            drain_timeout: self.drain_timeout.unwrap_or(defaults.drain_timeout),
            stop_timeout: self.stop_timeout.unwrap_or(defaults.stop_timeout),
            track_history: self.track_history.unwrap_or(defaults.track_history),
        }
    }
}
