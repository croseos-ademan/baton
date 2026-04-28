use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub max_events: usize,
    pub flush_interval: Duration,
    pub include_labels: bool,
    pub namespace: String,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_events: 1024,
            flush_interval: Duration::from_secs(30),
            include_labels: true,
            namespace: String::from("baton"),
        }
    }
}

impl TelemetryConfig {
    pub fn builder() -> TelemetryConfigBuilder {
        TelemetryConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct TelemetryConfigBuilder {
    enabled: Option<bool>,
    max_events: Option<usize>,
    flush_interval: Option<Duration>,
    include_labels: Option<bool>,
    namespace: Option<String>,
}

impl TelemetryConfigBuilder {
    pub fn enabled(mut self, v: bool) -> Self { self.enabled = Some(v); self }
    pub fn max_events(mut self, v: usize) -> Self { self.max_events = Some(v); self }
    pub fn flush_interval(mut self, v: Duration) -> Self { self.flush_interval = Some(v); self }
    pub fn include_labels(mut self, v: bool) -> Self { self.include_labels = Some(v); self }
    pub fn namespace(mut self, v: impl Into<String>) -> Self { self.namespace = Some(v.into()); self }

    pub fn build(self) -> TelemetryConfig {
        let defaults = TelemetryConfig::default();
        TelemetryConfig {
            enabled: self.enabled.unwrap_or(defaults.enabled),
            max_events: self.max_events.unwrap_or(defaults.max_events),
            flush_interval: self.flush_interval.unwrap_or(defaults.flush_interval),
            include_labels: self.include_labels.unwrap_or(defaults.include_labels),
            namespace: self.namespace.unwrap_or(defaults.namespace),
        }
    }
}
