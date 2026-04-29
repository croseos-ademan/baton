//! Load shedding module for rejecting excess work under pressure.
//!
//! Provides a `LoadShedder` that rejects requests when system load
//! exceeds configurable thresholds, protecting service stability
//! during handoff and high-traffic periods.

use crate::shedding_config::SheddingConfig;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct LoadShedder {
    config: SheddingConfig,
    in_flight: Arc<AtomicU64>,
    rejected_total: Arc<AtomicU64>,
    window_start: Arc<std::sync::Mutex<Instant>>,
    window_count: Arc<AtomicU64>,
}

#[derive(Debug, PartialEq)]
pub enum SheddingDecision {
    Allow,
    Shed { reason: &'static str },
}

impl LoadShedder {
    pub fn new(config: SheddingConfig) -> Self {
        Self {
            config,
            in_flight: Arc::new(AtomicU64::new(0)),
            rejected_total: Arc::new(AtomicU64::new(0)),
            window_start: Arc::new(std::sync::Mutex::new(Instant::now())),
            window_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn check(&self) -> SheddingDecision {
        let in_flight = self.in_flight.load(Ordering::Relaxed);
        if in_flight >= self.config.max_in_flight {
            self.rejected_total.fetch_add(1, Ordering::Relaxed);
            return SheddingDecision::Shed { reason: "max in-flight exceeded" };
        }

        if let Some(rps_limit) = self.config.max_rps {
            let count = self.window_count.fetch_add(1, Ordering::Relaxed);
            let elapsed = {
                let start = self.window_start.lock().unwrap();
                start.elapsed()
            };
            if elapsed < Duration::from_secs(1) && count >= rps_limit {
                self.rejected_total.fetch_add(1, Ordering::Relaxed);
                return SheddingDecision::Shed { reason: "rps limit exceeded" };
            } else if elapsed >= Duration::from_secs(1) {
                self.window_count.store(1, Ordering::Relaxed);
                *self.window_start.lock().unwrap() = Instant::now();
            }
        }

        SheddingDecision::Allow
    }

    pub fn acquire(&self) -> Option<SheddingGuard> {
        if self.check() == SheddingDecision::Allow {
            self.in_flight.fetch_add(1, Ordering::Relaxed);
            Some(SheddingGuard { in_flight: Arc::clone(&self.in_flight) })
        } else {
            None
        }
    }

    pub fn in_flight(&self) -> u64 {
        self.in_flight.load(Ordering::Relaxed)
    }

    pub fn rejected_total(&self) -> u64 {
        self.rejected_total.load(Ordering::Relaxed)
    }
}

pub struct SheddingGuard {
    in_flight: Arc<AtomicU64>,
}

impl Drop for SheddingGuard {
    fn drop(&mut self) {
        self.in_flight.fetch_sub(1, Ordering::Relaxed);
    }
}
