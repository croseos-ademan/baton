use crate::state::{ProcessState, StateTracker};
use crate::error::BatonError;

pub struct StateMachine {
    tracker: StateTracker,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            tracker: StateTracker::new(ProcessState::Starting),
        }
    }

    pub fn tracker(&self) -> StateTracker {
        self.tracker.clone()
    }

    pub fn advance(&self, next: ProcessState) -> Result<(), BatonError> {
        let current = self.tracker.current();
        if self.is_valid_transition(&current.state, &next) {
            self.tracker.transition(next);
            Ok(())
        } else {
            Err(BatonError::InvalidStateTransition(format!(
                "cannot transition from {} to {}",
                current.state, next
            )))
        }
    }

    fn is_valid_transition(&self, from: &ProcessState, to: &ProcessState) -> bool {
        matches!(
            (from, to),
            (ProcessState::Starting, ProcessState::Running)
                | (ProcessState::Starting, ProcessState::Failed(_))
                | (ProcessState::Running, ProcessState::Draining)
                | (ProcessState::Running, ProcessState::Stopping)
                | (ProcessState::Running, ProcessState::Failed(_))
                | (ProcessState::Draining, ProcessState::Stopping)
                | (ProcessState::Draining, ProcessState::Failed(_))
                | (ProcessState::Stopping, ProcessState::Stopped)
                | (ProcessState::Stopping, ProcessState::Failed(_))
        )
    }

    pub fn current_state(&self) -> ProcessState {
        self.tracker.current().state
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}
