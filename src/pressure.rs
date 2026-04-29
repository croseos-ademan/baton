//! Backpressure signaling for process handoff coordination.
//!
//! Monitors system load indicators and signals when the process
//! should pause accepting new work before initiating a handoff.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct PressureConfig {
    /// Maximum queue depth before backpressure is applied.
    pub queue_limit: u64,
    /// Minimum time to hold backpressure once triggered.
    pub hold_duration: Duration,
    /// How often to re-evaluate pressure state.
    pub poll_interval: Duration,
}

impl Default for PressureConfig {
    fn default() -> Self {
        Self {
            queue_limit: 1000,
            hold_duration: Duration::from_millis(500),
            poll_interval: Duration::from_millis(50),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PressureHandle {
    active: Arc<AtomicBool>,
    queue_depth: Arc<AtomicU64>,
    config: PressureConfig,
    triggered_at: Arc<std::sync::Mutex<Option<Instant>>>,
}

impl PressureHandle {
    pub fn new(config: PressureConfig) -> Self {
        Self {
            active: Arc::new(AtomicBool::new(false)),
            queue_depth: Arc::new(AtomicU64::new(0)),
            config,
            triggered_at: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    /// Increment the tracked queue depth by one.
    pub fn enqueue(&self) {
        let depth = self.queue_depth.fetch_add(1, Ordering::Relaxed) + 1;
        if depth >= self.config.queue_limit && !self.active.load(Ordering::Acquire) {
            self.active.store(true, Ordering::Release);
            if let Ok(mut guard) = self.triggered_at.lock() {
                *guard = Some(Instant::now());
            }
        }
    }

    /// Decrement the tracked queue depth by one.
    pub fn dequeue(&self) {
        self.queue_depth.fetch_sub(1, Ordering::Relaxed);
        self.evaluate();
    }

    /// Returns true if backpressure is currently active.
    pub fn is_active(&self) -> bool {
        if !self.active.load(Ordering::Acquire) {
            return false;
        }
        let depth = self.queue_depth.load(Ordering::Relaxed);
        if depth < self.config.queue_limit {
            if let Ok(guard) = self.triggered_at.lock() {
                if let Some(t) = *guard {
                    if t.elapsed() >= self.config.hold_duration {
                        self.active.store(false, Ordering::Release);
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Force-clear backpressure regardless of hold duration.
    pub fn release(&self) {
        self.active.store(false, Ordering::Release);
        self.queue_depth.store(0, Ordering::Relaxed);
        if let Ok(mut guard) = self.triggered_at.lock() {
            *guard = None;
        }
    }

    /// Current queue depth.
    pub fn depth(&self) -> u64 {
        self.queue_depth.load(Ordering::Relaxed)
    }

    fn evaluate(&self) {
        if self.active.load(Ordering::Acquire) {
            let _ = self.is_active(); // side-effect: may clear flag
        }
    }
}
