//! Parsing and validation of health-check configuration.

use std::time::Duration;
use crate::health::HealthCheck;
use crate::error::{BatonError, Result};

/// Raw, string-based health check descriptor (e.g. from CLI or env).
#[derive(Debug, Clone)]
pub struct HealthCheckSpec {
    pub kind: String,
    pub value: Option<String>,
    pub timeout_secs: u64,
}

impl HealthCheckSpec {
    pub fn new(kind: impl Into<String>) -> Self {
        Self { kind: kind.into(), value: None, timeout_secs: 30 }
    }

    pub fn with_value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }

    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Convert into a resolved [`HealthCheck`] and timeout [`Duration`].
    pub fn resolve(&self) -> Result<(HealthCheck, Duration)> {
        let timeout = Duration::from_secs(self.timeout_secs);
        let check = match self.kind.as_str() {
            "ready_fd" => HealthCheck::ReadyFd,
            "tcp" => {
                let port: u16 = self
                    .value
                    .as_deref()
                    .unwrap_or("")
                    .parse()
                    .map_err(|_| BatonError::Config("tcp health check requires a valid port".into()))?;
                HealthCheck::TcpPort(port)
            }
            "delay" => {
                let ms: u64 = self
                    .value
                    .as_deref()
                    .unwrap_or("0")
                    .parse()
                    .map_err(|_| BatonError::Config("delay health check requires milliseconds".into()))?;
                HealthCheck::Delay(Duration::from_millis(ms))
            }
            other => return Err(BatonError::Config(format!("unknown health check kind: {}", other))),
        };
        Ok((check, timeout))
    }
}
