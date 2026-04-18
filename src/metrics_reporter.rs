use crate::metrics::{Metrics, MetricsSnapshot};
use std::fmt;
use std::sync::Arc;

pub struct MetricsReporter {
    metrics: Arc<Metrics>,
}

impl MetricsReporter {
    pub fn new(metrics: Arc<Metrics>) -> Self {
        Self { metrics }
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        self.metrics.snapshot()
    }

    pub fn format_text(&self) -> String {
        let s = self.snapshot();
        format!(
            "handoffs_total={} handoff_failures={} last_handoff_duration_ms={} \
             health_checks_total={} health_check_failures={} restarts_total={}",
            s.handoffs_total,
            s.handoff_failures,
            s.last_handoff_duration_ms,
            s.health_checks_total,
            s.health_check_failures,
            s.restarts_total,
        )
    }

    pub fn format_json(&self) -> String {
        let s = self.snapshot();
        format!(
            r#"{{"handoffs_total":{},"handoff_failures":{},"last_handoff_duration_ms":{},"health_checks_total":{},"health_check_failures":{},"restarts_total":{}}}",
            s.handoffs_total,
            s.handoff_failures,
            s.last_handoff_duration_ms,
            s.health_checks_total,
            s.health_check_failures,
            s.restarts_total,
        )
    }
}

impl fmt::Display for MetricsReporter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_text())
    }
}
