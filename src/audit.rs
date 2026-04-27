//! Audit log for tracking process handoff events and lifecycle transitions.

use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
pub enum AuditEvent {
    HandoffStarted { old_pid: u32, new_pid: u32 },
    HandoffCompleted { old_pid: u32, new_pid: u32, duration_ms: u64 },
    HandoffFailed { old_pid: u32, reason: String },
    ProcessStarted { pid: u32, command: String },
    ProcessStopped { pid: u32, exit_code: Option<i32> },
    RollbackTriggered { from_pid: u32, to_pid: u32, reason: String },
    HealthCheckFailed { pid: u32, attempt: u32 },
}

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp_ms: u64,
    pub event: AuditEvent,
}

impl AuditEntry {
    pub fn new(event: AuditEvent) -> Self {
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        Self { timestamp_ms, event }
    }
}

impl fmt::Display for AuditEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}ms] {:?}", self.timestamp_ms, self.event)
    }
}

#[derive(Debug, Default)]
pub struct AuditLog {
    entries: Vec<AuditEntry>,
    max_entries: usize,
}

impl AuditLog {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }

    pub fn record(&mut self, event: AuditEvent) {
        if self.max_entries > 0 && self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(AuditEntry::new(event));
    }

    pub fn entries(&self) -> &[AuditEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn last(&self) -> Option<&AuditEntry> {
        self.entries.last()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn filter_by<F>(&self, predicate: F) -> Vec<&AuditEntry>
    where
        F: Fn(&AuditEntry) -> bool,
    {
        self.entries.iter().filter(|e| predicate(e)).collect()
    }
}
