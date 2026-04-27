#[cfg(test)]
mod tests {
    use crate::snapshot::Snapshot;
    use crate::snapshot_config::SnapshotConfig;
    use crate::state::ProcessState;

    fn make_snapshot(state: ProcessState, handoffs: u32) -> Snapshot {
        Snapshot::capture(state, Some(1234), 60, handoffs, None, None)
    }

    #[test]
    fn test_snapshot_is_healthy_when_running() {
        let s = make_snapshot(ProcessState::Running, 0);
        assert!(s.is_healthy());
    }

    #[test]
    fn test_snapshot_not_healthy_when_stopped() {
        let s = make_snapshot(ProcessState::Stopped, 0);
        assert!(!s.is_healthy());
    }

    #[test]
    fn test_snapshot_summary_contains_pid() {
        let s = make_snapshot(ProcessState::Running, 3);
        assert!(s.summary().contains("1234"));
    }

    #[test]
    fn test_snapshot_summary_contains_handoff_count() {
        let s = make_snapshot(ProcessState::Running, 7);
        assert!(s.summary().contains("handoffs=7"));
    }

    #[test]
    fn test_snapshot_no_pid() {
        let s = Snapshot::capture(ProcessState::Starting, None, 0, 0, None, None);
        assert!(s.summary().contains("pid=none"));
    }

    #[test]
    fn test_config_default_is_enabled() {
        let cfg = SnapshotConfig::default();
        assert!(cfg.enabled);
        assert_eq!(cfg.max_retained, 10);
    }

    #[test]
    fn test_config_disabled() {
        let cfg = SnapshotConfig::disabled();
        assert!(!cfg.enabled);
    }

    #[test]
    fn test_config_builder_chain() {
        let cfg = SnapshotConfig::new()
            .with_interval(60)
            .with_max_retained(5)
            .with_metrics(false);
        assert_eq!(cfg.interval_secs, 60);
        assert_eq!(cfg.max_retained, 5);
        assert!(!cfg.include_metrics);
    }

    #[test]
    fn test_config_validate_ok() {
        let cfg = SnapshotConfig::new().with_max_retained(3);
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_config_max_retained_floor() {
        let cfg = SnapshotConfig::new().with_max_retained(0);
        // with_max_retained clamps to 1
        assert_eq!(cfg.max_retained, 1);
        assert!(cfg.validate().is_ok());
    }
}
