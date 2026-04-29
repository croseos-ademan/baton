/// Configuration for the [`Quorum`] coordinator.
#[derive(Debug, Clone)]
pub struct QuorumConfig {
    /// Minimum number of distinct votes required to reach quorum.
    pub threshold: usize,
    /// Human-readable label used in logs/audit events.
    pub label: String,
}

impl QuorumConfig {
    pub fn new(threshold: usize, label: impl Into<String>) -> Self {
        assert!(threshold > 0, "quorum threshold must be at least 1");
        Self {
            threshold,
            label: label.into(),
        }
    }

    /// Convenience: majority of `n` participants (n/2 + 1).
    pub fn majority(n: usize, label: impl Into<String>) -> Self {
        Self::new(n / 2 + 1, label)
    }
}

impl Default for QuorumConfig {
    fn default() -> Self {
        Self::new(1, "default")
    }
}
