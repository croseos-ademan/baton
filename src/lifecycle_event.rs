use crate::lifecycle::LifecyclePhase;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct LifecycleEvent {
    pub from: LifecyclePhase,
    pub to: LifecyclePhase,
    pub timestamp: SystemTime,
}

impl LifecycleEvent {
    pub fn new(from: LifecyclePhase, to: LifecyclePhase) -> Self {
        Self {
            from,
            to,
            timestamp: SystemTime::now(),
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self.to, LifecyclePhase::Stopped | LifecyclePhase::Failed)
    }

    pub fn is_degraded(&self) -> bool {
        matches!(self.to, LifecyclePhase::Failed)
    }

    pub fn description(&self) -> String {
        format!("{:?} -> {:?}", self.from, self.to)
    }
}

impl std::fmt::Display for LifecycleEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LifecycleEvent({})", self.description())
    }
}
