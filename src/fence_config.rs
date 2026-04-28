//! Configuration for the handoff fence.

use std::path::PathBuf;
use std::time::Duration;

/// Configuration controlling fence behaviour during handoff.
#[derive(Debug, Clone)]
pub struct FenceConfig {
    /// Directory where the fence file is created.
    pub dir: PathBuf,
    /// Maximum age of a fence before it is considered stale.
    pub stale_after: Duration,
    /// Whether to automatically clear a stale fence before acquiring.
    pub auto_clear_stale: bool,
}

impl FenceConfig {
    /// Create a new `FenceConfig` with sensible defaults.
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir,
            stale_after: Duration::from_secs(60),
            auto_clear_stale: false,
        }
    }

    /// Set the stale threshold.
    pub fn stale_after(mut self, d: Duration) -> Self {
        self.stale_after = d;
        self
    }

    /// Enable automatic clearing of stale fences.
    pub fn auto_clear_stale(mut self, v: bool) -> Self {
        self.auto_clear_stale = v;
        self
    }

    /// Validate the configuration.
    pub fn validate(&self) -> Result<(), String> {
        if !self.dir.exists() {
            return Err(format!(
                "fence dir does not exist: {}",
                self.dir.display()
            ));
        }
        if self.stale_after.as_secs() == 0 {
            return Err("stale_after must be > 0".into());
        }
        Ok(())
    }
}

impl Default for FenceConfig {
    fn default() -> Self {
        Self::new(std::env::temp_dir())
    }
}
