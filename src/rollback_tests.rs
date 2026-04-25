#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::rollback::{RollbackManager, RollbackReason, RollbackState};
    use crate::rollback_config::{RollbackConfig, RollbackConfigBuilder};

    fn default_manager() -> RollbackManager {
        RollbackManager::new(RollbackConfig::default())
    }

    #[test]
    fn test_initial_state_is_idle() {
        let mgr = default_manager();
        assert_eq!(*mgr.state(), RollbackState::Idle);
        assert_eq!(mgr.attempt_count(), 0);
    }

    #[test]
    fn test_trigger_sets_state() {
        let mut mgr = default_manager();
        mgr.trigger(RollbackReason::HealthCheckFailed).unwrap();
        assert!(matches!(mgr.state(), RollbackState::Triggered(_)));
        assert_eq!(mgr.attempt_count(), 1);
    }

    #[test]
    fn test_begin_after_trigger() {
        let mut mgr = default_manager();
        mgr.trigger(RollbackReason::StartupTimeout).unwrap();
        mgr.begin().unwrap();
        assert_eq!(*mgr.state(), RollbackState::InProgress);
    }

    #[test]
    fn test_begin_without_trigger_fails() {
        let mut mgr = default_manager();
        assert!(mgr.begin().is_err());
    }

    #[test]
    fn test_complete_sets_completed() {
        let mut mgr = default_manager();
        mgr.trigger(RollbackReason::ProcessCrashed).unwrap();
        mgr.begin().unwrap();
        mgr.complete();
        assert_eq!(*mgr.state(), RollbackState::Completed);
    }

    #[test]
    fn test_fail_records_reason() {
        let mut mgr = default_manager();
        mgr.trigger(RollbackReason::Manual).unwrap();
        mgr.begin().unwrap();
        mgr.fail("binary not found");
        assert!(matches!(mgr.state(), RollbackState::Failed(_)));
    }

    #[test]
    fn test_max_attempts_enforced() {
        let config = RollbackConfigBuilder::new().max_attempts(2).build();
        let mut mgr = RollbackManager::new(config);
        mgr.trigger(RollbackReason::Manual).unwrap();
        mgr.trigger(RollbackReason::Manual).unwrap();
        let result = mgr.trigger(RollbackReason::Manual);
        assert!(result.is_err());
    }

    #[test]
    fn test_disabled_rollback_returns_error() {
        let config = RollbackConfigBuilder::new().enabled(false).build();
        let mut mgr = RollbackManager::new(config);
        let result = mgr.trigger(RollbackReason::Manual);
        assert!(result.is_err());
    }

    #[test]
    fn test_elapsed_after_trigger() {
        let mut mgr = default_manager();
        mgr.trigger(RollbackReason::HealthCheckFailed).unwrap();
        assert!(mgr.elapsed().is_some());
    }

    #[test]
    fn test_is_within_window() {
        let config = RollbackConfigBuilder::new()
            .window(Duration::from_secs(10))
            .build();
        let mut mgr = RollbackManager::new(config);
        mgr.trigger(RollbackReason::Manual).unwrap();
        assert!(mgr.is_within_window());
    }
}
