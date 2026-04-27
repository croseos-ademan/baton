//! Process state snapshot for diagnostics and inspection.

use std::time::{SystemTime, UNIX_EPOCH};
use crate::state::ProcessState;
use crate::metrics::Metrics;

/// A point-in-time snapshot of the process handoff state.
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub timestamp: u64,
    pub state: ProcessState,
    pub pid: Option<u32>,
    pub uptime_secs: u64,
    pub handoff_count: u32,
    pub last_handoff_ts: Option<u64>,
    pub metrics: Option<Metrics>,
}

impl Snapshot {
    /// Capture a new snapshot from the given components.
    pub fn capture(
        state: ProcessState,
        pid: Option<u32>,
        uptime_secs: u64,
        handoff_count: u32,
        last_handoff_ts: Option<u64>,
        metrics: Option<Metrics>,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            timestamp,
            state,
            pid,
            uptime_secs,
            handoff_count,
            last_handoff_ts,
            metrics,
        }
    }

    /// Returns true if the snapshot represents a healthy running process.
    pub fn is_healthy(&self) -> bool {
        matches!(self.state, ProcessState::Running)
    }

    /// Format the snapshot as a human-readable summary string.
    pub fn summary(&self) -> String {
        format!(
            "[snapshot ts={} state={:?} pid={} uptime={}s handoffs={}]",
            self.timestamp,
            self.state,
            self.pid.map(|p| p.to_string()).unwrap_or_else(|| "none".into()),
            self.uptime_secs,
            self.handoff_count,
        )
    }
}
