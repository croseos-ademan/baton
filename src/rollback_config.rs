use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RollbackConfig {
    pub enabled: bool,
    pub max_attempts: u32,
    pub window: Duration,
    pub delay_before_rollback: Duration,
    pub restore_previous_binary: bool,
}

impl Default for RollbackConfig {
    fn default() -> Self {
        RollbackConfig {
            enabled: true,
            max_attempts: 3,
            window: Duration::from_secs(60),
            delay_before_rollback: Duration::from_millis(500),
            restore_previous_binary: false,
        }
    }
}

pub struct RollbackConfigBuilder {
    config: RollbackConfig,
}

impl RollbackConfigBuilder {
    pub fn new() -> Self {
        RollbackConfigBuilder {
            config: RollbackConfig::default(),
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.config.enabled = enabled;
        self
    }

    pub fn max_attempts(mut self, max: u32) -> Self {
        self.config.max_attempts = max;
        self
    }

    pub fn window(mut self, window: Duration) -> Self {
        self.config.window = window;
        self
    }

    pub fn delay_before_rollback(mut self, delay: Duration) -> Self {
        self.config.delay_before_rollback = delay;
        self
    }

    pub fn restore_previous_binary(mut self, restore: bool) -> Self {
        self.config.restore_previous_binary = restore;
        self
    }

    pub fn build(self) -> RollbackConfig {
        self.config
    }
}
