//! Configuration for the load shedding module.

/// Configuration for `LoadShedder`.
#[derive(Debug, Clone)]
pub struct SheddingConfig {
    /// Maximum number of concurrent in-flight requests allowed.
    pub max_in_flight: u64,
    /// Optional maximum requests per second before shedding begins.
    pub max_rps: Option<u64>,
    /// Whether load shedding is enabled at all.
    pub enabled: bool,
}

impl Default for SheddingConfig {
    fn default() -> Self {
        Self {
            max_in_flight: 1000,
            max_rps: None,
            enabled: true,
        }
    }
}

impl SheddingConfig {
    pub fn new(max_in_flight: u64) -> Self {
        Self {
            max_in_flight,
            ..Default::default()
        }
    }

    pub fn with_max_rps(mut self, rps: u64) -> Self {
        self.max_rps = Some(rps);
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.max_in_flight == 0 {
            return Err("max_in_flight must be greater than zero");
        }
        if let Some(rps) = self.max_rps {
            if rps == 0 {
                return Err("max_rps must be greater than zero if set");
            }
        }
        Ok(())
    }
}
