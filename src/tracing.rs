//! Distributed tracing support for baton process handoffs.
//!
//! Provides span-based tracing across the handoff lifecycle, allowing
//! operators to observe latency, causality, and errors during zero-downtime
//! restarts. Traces are emitted as structured log lines compatible with
//! OpenTelemetry-style consumers.

use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

static SPAN_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// A unique identifier for a tracing span.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpanId(u64);

impl SpanId {
    /// Generate a new monotonically increasing span ID.
    pub fn new() -> Self {
        SpanId(SPAN_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl fmt::Display for SpanId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

/// A single tracing span representing a unit of work.
#[derive(Debug)]
pub struct Span {
    pub id: SpanId,
    pub parent_id: Option<SpanId>,
    pub name: String,
    pub start: Instant,
    pub duration: Option<Duration>,
    pub attributes: HashMap<String, String>,
    pub status: SpanStatus,
}

/// The completion status of a span.
#[derive(Debug, Clone, PartialEq)]
pub enum SpanStatus {
    Ok,
    Error(String),
    InProgress,
}

impl Span {
    /// Begin a new root span with the given name.
    pub fn start(name: impl Into<String>) -> Self {
        Span {
            id: SpanId::new(),
            parent_id: None,
            name: name.into(),
            start: Instant::now(),
            duration: None,
            attributes: HashMap::new(),
            status: SpanStatus::InProgress,
        }
    }

    /// Begin a child span under the given parent.
    pub fn child(name: impl Into<String>, parent: &Span) -> Self {
        let mut span = Span::start(name);
        span.parent_id = Some(parent.id);
        span
    }

    /// Attach a key-value attribute to this span.
    pub fn set_attr(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.attributes.insert(key.into(), value.into());
    }

    /// Mark the span as successfully completed.
    pub fn finish(&mut self) {
        self.duration = Some(self.start.elapsed());
        self.status = SpanStatus::Ok;
    }

    /// Mark the span as failed with an error message.
    pub fn finish_err(&mut self, err: impl fmt::Display) {
        self.duration = Some(self.start.elapsed());
        self.status = SpanStatus::Error(err.to_string());
    }

    /// Emit the span as a structured log line to stderr.
    pub fn emit(&self) {
        let duration_ms = self
            .duration
            .map(|d| format!("{:.3}", d.as_secs_f64() * 1000.0))
            .unwrap_or_else(|| "ongoing".to_string());

        let status = match &self.status {
            SpanStatus::Ok => "ok".to_string(),
            SpanStatus::Error(e) => format!("error: {}", e),
            SpanStatus::InProgress => "in_progress".to_string(),
        };

        let parent = self
            .parent_id
            .map(|p| p.to_string())
            .unwrap_or_else(|| "none".to_string());

        let attrs: Vec<String> = self
            .attributes
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();

        eprintln!(
            "[trace] span={} parent={} name={:?} duration_ms={} status={} attrs=[{}]",
            self.id,
            parent,
            self.name,
            duration_ms,
            status,
            attrs.join(", ")
        );
    }
}

#[cfg(test)]
#[path = "tracing_tests.rs"]
mod tests;
