#[cfg(test)]
mod tests {
    use crate::lifecycle::{Lifecycle, LifecyclePhase};
    use crate::lifecycle_config::LifecycleConfig;
    use crate::lifecycle_event::LifecycleEvent;
    use crate::lifecycle_hook::LifecycleHook;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    fn default_lifecycle() -> Lifecycle {
        Lifecycle::new(LifecycleConfig::default())
    }

    #[test]
    fn test_initial_phase_is_init() {
        let lc = default_lifecycle();
        assert_eq!(lc.phase(), LifecyclePhase::Init);
    }

    #[test]
    fn test_valid_transition_init_to_starting() {
        let mut lc = default_lifecycle();
        assert!(lc.transition(LifecyclePhase::Starting).is_ok());
        assert_eq!(lc.phase(), LifecyclePhase::Starting);
    }

    #[test]
    fn test_invalid_transition_returns_error() {
        let mut lc = default_lifecycle();
        let result = lc.transition(LifecyclePhase::Running);
        assert!(result.is_err());
    }

    #[test]
    fn test_full_happy_path() {
        let mut lc = default_lifecycle();
        lc.transition(LifecyclePhase::Starting).unwrap();
        lc.transition(LifecyclePhase::Running).unwrap();
        lc.transition(LifecyclePhase::Draining).unwrap();
        lc.transition(LifecyclePhase::Stopping).unwrap();
        lc.transition(LifecyclePhase::Stopped).unwrap();
        assert_eq!(lc.phase(), LifecyclePhase::Stopped);
    }

    #[test]
    fn test_event_listener_called_on_transition() {
        let mut lc = default_lifecycle();
        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();
        lc.on_event(move |_event: &LifecycleEvent| {
            *called_clone.lock().unwrap() = true;
        });
        lc.transition(LifecyclePhase::Starting).unwrap();
        assert!(*called.lock().unwrap());
    }

    #[test]
    fn test_lifecycle_event_is_terminal() {
        use crate::lifecycle_event::LifecycleEvent;
        let ev = LifecycleEvent::new(LifecyclePhase::Stopping, LifecyclePhase::Stopped);
        assert!(ev.is_terminal());
        let ev2 = LifecycleEvent::new(LifecyclePhase::Init, LifecyclePhase::Starting);
        assert!(!ev2.is_terminal());
    }

    #[test]
    fn test_hook_registered_and_run() {
        let mut hook = LifecycleHook::new();
        let ran = Arc::new(Mutex::new(false));
        let ran_clone = ran.clone();
        hook.register(&LifecyclePhase::Running, move || {
            *ran_clone.lock().unwrap() = true;
            Ok(())
        });
        assert_eq!(hook.hook_count(&LifecyclePhase::Running), 1);
        hook.run(&LifecyclePhase::Running).unwrap();
        assert!(*ran.lock().unwrap());
    }

    #[test]
    fn test_config_builder() {
        let cfg = LifecycleConfig::builder()
            .startup_timeout(Duration::from_secs(5))
            .max_restarts(10)
            .build();
        assert_eq!(cfg.startup_timeout, Duration::from_secs(5));
        assert_eq!(cfg.max_restarts, 10);
    }
}
