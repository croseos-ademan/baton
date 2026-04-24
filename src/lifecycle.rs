use crate::error::BatonError;
use crate::lifecycle_config::LifecycleConfig;
use crate::lifecycle_event::LifecycleEvent;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum LifecyclePhase {
    Init,
    Starting,
    Running,
    Draining,
    Stopping,
    Stopped,
    Failed,
}

pub struct Lifecycle {
    phase: Arc<Mutex<LifecyclePhase>>,
    config: LifecycleConfig,
    started_at: Option<Instant>,
    listeners: Vec<Box<dyn Fn(&LifecycleEvent) + Send + Sync>>,
}

impl Lifecycle {
    pub fn new(config: LifecycleConfig) -> Self {
        Self {
            phase: Arc::new(Mutex::new(LifecyclePhase::Init)),
            config,
            started_at: None,
            listeners: Vec::new(),
        }
    }

    pub fn transition(&mut self, next: LifecyclePhase) -> Result<(), BatonError> {
        let mut phase = self.phase.lock().map_err(|_| BatonError::LockPoisoned)?;
        if !self.is_valid_transition(&phase, &next) {
            return Err(BatonError::InvalidStateTransition(
                format!("{:?}", phase),
                format!("{:?}", next),
            ));
        }
        let event = LifecycleEvent::new(phase.clone(), next.clone());
        *phase = next;
        drop(phase);
        for listener in &self.listeners {
            listener(&event);
        }
        Ok(())
    }

    pub fn phase(&self) -> LifecyclePhase {
        self.phase.lock().unwrap().clone()
    }

    pub fn on_event<F: Fn(&LifecycleEvent) + Send + Sync + 'static>(&mut self, f: F) {
        self.listeners.push(Box::new(f));
    }

    pub fn uptime_secs(&self) -> Option<u64> {
        self.started_at.map(|t| t.elapsed().as_secs())
    }

    fn is_valid_transition(&self, from: &LifecyclePhase, to: &LifecyclePhase) -> bool {
        use LifecyclePhase::*;
        matches!(
            (from, to),
            (Init, Starting)
                | (Starting, Running)
                | (Starting, Failed)
                | (Running, Draining)
                | (Running, Stopping)
                | (Running, Failed)
                | (Draining, Stopping)
                | (Stopping, Stopped)
                | (Failed, Stopped)
        )
    }
}
