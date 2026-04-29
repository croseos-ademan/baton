//! Admission control: gate new handoffs based on system readiness conditions.

use crate::admission_config::AdmissionConfig;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum AdmissionDecision {
    Allow,
    Deny(String),
}

#[derive(Debug)]
pub struct AdmissionController {
    config: AdmissionConfig,
    last_handoff: Option<Instant>,
    handoff_count: u32,
}

impl AdmissionController {
    pub fn new(config: AdmissionConfig) -> Self {
        Self {
            config,
            last_handoff: None,
            handoff_count: 0,
        }
    }

    /// Evaluate whether a new handoff should be admitted.
    pub fn evaluate(&self, load: f64) -> AdmissionDecision {
        if load > self.config.max_load_threshold {
            return AdmissionDecision::Deny(format!(
                "system load {:.2} exceeds threshold {:.2}",
                load, self.config.max_load_threshold
            ));
        }

        if let Some(last) = self.last_handoff {
            let elapsed = last.elapsed();
            if elapsed < self.config.min_interval {
                return AdmissionDecision::Deny(format!(
                    "minimum interval not elapsed: {:?} remaining",
                    self.config.min_interval - elapsed
                ));
            }
        }

        if let Some(max) = self.config.max_handoffs_per_window {
            if self.handoff_count >= max {
                return AdmissionDecision::Deny(format!(
                    "handoff quota exhausted: {} of {} used",
                    self.handoff_count, max
                ));
            }
        }

        AdmissionDecision::Allow
    }

    /// Record that a handoff was admitted and started.
    pub fn record_handoff(&mut self) {
        self.last_handoff = Some(Instant::now());
        self.handoff_count += 1;
    }

    /// Reset the rolling window counters (call periodically).
    pub fn reset_window(&mut self) {
        self.handoff_count = 0;
    }

    pub fn handoff_count(&self) -> u32 {
        self.handoff_count
    }
}
