#[cfg(test)]
mod tests {
    use crate::state::{ProcessState, StateTracker};
    use crate::state_machine::StateMachine;
    use crate::state_observer::StateObserver;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_initial_state_is_starting() {
        let sm = StateMachine::new();
        assert_eq!(sm.current_state(), ProcessState::Starting);
    }

    #[test]
    fn test_valid_transition_starting_to_running() {
        let sm = StateMachine::new();
        assert!(sm.advance(ProcessState::Running).is_ok());
        assert_eq!(sm.current_state(), ProcessState::Running);
    }

    #[test]
    fn test_invalid_transition_returns_error() {
        let sm = StateMachine::new();
        let result = sm.advance(ProcessState::Stopped);
        assert!(result.is_err());
    }

    #[test]
    fn test_full_lifecycle() {
        let sm = StateMachine::new();
        sm.advance(ProcessState::Running).unwrap();
        sm.advance(ProcessState::Draining).unwrap();
        sm.advance(ProcessState::Stopping).unwrap();
        sm.advance(ProcessState::Stopped).unwrap();
        assert_eq!(sm.current_state(), ProcessState::Stopped);
    }

    #[test]
    fn test_failed_transition_allowed_from_running() {
        let sm = StateMachine::new();
        sm.advance(ProcessState::Running).unwrap();
        sm.advance(ProcessState::Failed("oom".to_string())).unwrap();
        assert!(matches!(sm.current_state(), ProcessState::Failed(_)));
    }

    #[test]
    fn test_observer_fires_on_transition() {
        let tracker = StateTracker::new(ProcessState::Starting);
        let observer = StateObserver::new(tracker.clone());
        let fired = Arc::new(Mutex::new(false));
        let fired_clone = fired.clone();
        observer.on_transition(move |_from, _to| {
            *fired_clone.lock().unwrap() = true;
        });
        tracker.transition(ProcessState::Running);
        observer.poll();
        assert!(*fired.lock().unwrap());
    }

    #[test]
    fn test_state_display() {
        assert_eq!(ProcessState::Running.to_string(), "running");
        assert_eq!(ProcessState::Draining.to_string(), "draining");
        assert_eq!(ProcessState::Failed("err".into()).to_string(), "failed: err");
    }
}
