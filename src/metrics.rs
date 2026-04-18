use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct Metrics {
    pub handoffs_total: AtomicU64,
    pub handoff_failures: AtomicU64,
    pub last_handoff_duration_ms: AtomicU64,
    pub health_checks_total: AtomicU64,
    pub health_check_failures: AtomicU64,
    pub restarts_total: AtomicU64,
}

impl Metrics {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn record_handoff(&self, duration: Duration, success: bool) {
        self.handoffs_total.fetch_add(1, Ordering::Relaxed);
        self.last_handoff_duration_ms
            .store(duration.as_millis() as u64, Ordering::Relaxed);
        if !success {
            self.handoff_failures.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_health_check(&self, success: bool) {
        self.health_checks_total.fetch_add(1, Ordering::Relaxed);
        if !success {
            self.health_check_failures.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_restart(&self) {
        self.restarts_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            handoffs_total: self.handoffs_total.load(Ordering::Relaxed),
            handoff_failures: self.handoff_failures.load(Ordering::Relaxed),
            last_handoff_duration_ms: self.last_handoff_duration_ms.load(Ordering::Relaxed),
            health_checks_total: self.health_checks_total.load(Ordering::Relaxed),
            health_check_failures: self.health_check_failures.load(Ordering::Relaxed),
            restarts_total: self.restarts_total.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetricsSnapshot {
    pub handoffs_total: u64,
    pub handoff_failures: u64,
    pub last_handoff_duration_ms: u64,
    pub health_checks_total: u64,
    pub health_check_failures: u64,
    pub restarts_total: u64,
}

pub struct HandoffTimer {
    start: Instant,
}

impl HandoffTimer {
    pub fn start() -> Self {
        Self { start: Instant::now() }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}
