use crate::error::BatonError;
use crate::metrics::Metrics;
use crate::metrics_config::{MetricsConfig, MetricsFormat};
use crate::metrics_reporter::MetricsReporter;
use std::fs;
use std::sync::Arc;

pub struct MetricsWriter {
    config: MetricsConfig,
    reporter: MetricsReporter,
}

impl MetricsWriter {
    pub fn new(config: MetricsConfig, metrics: Arc<Metrics>) -> Self {
        Self {
            reporter: MetricsReporter::new(metrics),
            config,
        }
    }

    pub fn write(&self) -> Result<(), BatonError> {
        if !self.config.enabled {
            return Ok(());
        }

        let content = match self.config.format {
            MetricsFormat::Text => self.reporter.format_text(),
            MetricsFormat::Json => self.reporter.format_json(),
        };

        if let Some(ref path) = self.config.output_path {
            fs::write(path, &content).map_err(|e| {
                BatonError::Io(format!("failed to write metrics to {}: {}", path.display(), e))
            })?;
        } else {
            eprintln!("[baton:metrics] {}", content);
        }

        Ok(())
    }

    pub fn snapshot_text(&self) -> String {
        self.reporter.format_text()
    }

    pub fn snapshot_json(&self) -> String {
        self.reporter.format_json()
    }
}
