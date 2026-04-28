use std::time::{Duration, Instant};
use std::collections::HashMap;
use crate::telemetry_config::TelemetryConfig;
use crate::error::BatonError;

#[derive(Debug, Clone)]
pub struct TelemetryEvent {
    pub name: String,
    pub timestamp: Instant,
    pub labels: HashMap<String, String>,
    pub duration: Option<Duration>,
}

impl TelemetryEvent {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            timestamp: Instant::now(),
            labels: HashMap::new(),
            duration: None,
        }
    }

    pub fn with_label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.insert(key.into(), value.into());
        self
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }
}

pub struct Telemetry {
    config: TelemetryConfig,
    events: Vec<TelemetryEvent>,
}

impl Telemetry {
    pub fn new(config: TelemetryConfig) -> Self {
        Self {
            config,
            events: Vec::new(),
        }
    }

    pub fn record(&mut self, event: TelemetryEvent) {
        if !self.config.enabled {
            return;
        }
        if self.events.len() >= self.config.max_events {
            self.events.remove(0);
        }
        self.events.push(event);
    }

    pub fn span<F, T>(&mut self, name: &str, f: F) -> Result<T, BatonError>
    where
        F: FnOnce() -> Result<T, BatonError>,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        let mut event = TelemetryEvent::new(name).with_duration(duration);
        if result.is_err() {
            event = event.with_label("status", "error");
        } else {
            event = event.with_label("status", "ok");
        }
        self.record(event);
        result
    }

    pub fn events(&self) -> &[TelemetryEvent] {
        &self.events
    }

    pub fn flush(&mut self) -> Vec<TelemetryEvent> {
        std::mem::take(&mut self.events)
    }
}
