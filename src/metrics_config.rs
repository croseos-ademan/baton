use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum MetricsFormat {
    Text,
    Json,
}

impl Default for MetricsFormat {
    fn default() -> Self {
        MetricsFormat::Text
    }
}

#[derive(Debug, Clone)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub format: MetricsFormat,
    pub output_path: Option<PathBuf>,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            format: MetricsFormat::default(),
            output_path: None,
        }
    }
}

impl MetricsConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_format(mut self, format: MetricsFormat) -> Self {
        self.format = format;
        self
    }

    pub fn with_output_path(mut self, path: PathBuf) -> Self {
        self.output_path = Some(path);
        self
    }
}
