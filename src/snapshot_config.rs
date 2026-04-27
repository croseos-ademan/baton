//! Configuration for the snapshot capture behaviour.

/// Controls how and when snapshots are captured.
#[derive(Debug, Clone)]
pub struct SnapshotConfig {
    /// Whether snapshot capture is enabled at all.
    pub enabled: bool,
    /// Interval in seconds between automatic snapshots (0 = disabled).
    pub interval_secs: u64,
    /// Maximum number of snapshots to retain in the ring buffer.
    pub max_retained: usize,
    /// Whether to include metrics data in each snapshot.
    pub include_metrics: bool,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_secs: 30,
            max_retained: 10,
            include_metrics: true,
        }
    }
}

impl SnapshotConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_interval(mut self, secs: u64) -> Self {
        self.interval_secs = secs;
        self
    }

    pub fn with_max_retained(mut self, n: usize) -> Self {
        self.max_retained = n.max(1);
        self
    }

    pub fn with_metrics(mut self, include: bool) -> Self {
        self.include_metrics = include;
        self
    }

    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.max_retained == 0 {
            return Err("max_retained must be at least 1".into());
        }
        Ok(())
    }
}
