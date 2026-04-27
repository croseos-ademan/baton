#[cfg(test)]
mod tests {
    use crate::audit::{AuditEvent, AuditLog};

    #[test]
    fn test_record_and_retrieve() {
        let mut log = AuditLog::new(100);
        log.record(AuditEvent::ProcessStarted {
            pid: 1234,
            command: "nginx".to_string(),
        });
        assert_eq!(log.len(), 1);
        assert!(!log.is_empty());
    }

    #[test]
    fn test_max_entries_evicts_oldest() {
        let mut log = AuditLog::new(3);
        log.record(AuditEvent::ProcessStarted { pid: 1, command: "a".to_string() });
        log.record(AuditEvent::ProcessStarted { pid: 2, command: "b".to_string() });
        log.record(AuditEvent::ProcessStarted { pid: 3, command: "c".to_string() });
        log.record(AuditEvent::ProcessStarted { pid: 4, command: "d".to_string() });
        assert_eq!(log.len(), 3);
        if let AuditEvent::ProcessStarted { pid, .. } = &log.entries()[0].event {
            assert_eq!(*pid, 2);
        } else {
            panic!("unexpected event");
        }
    }

    #[test]
    fn test_last_entry() {
        let mut log = AuditLog::new(10);
        log.record(AuditEvent::HandoffStarted { old_pid: 10, new_pid: 20 });
        log.record(AuditEvent::HandoffCompleted { old_pid: 10, new_pid: 20, duration_ms: 150 });
        let last = log.last().unwrap();
        assert!(matches!(last.event, AuditEvent::HandoffCompleted { .. }));
    }

    #[test]
    fn test_filter_by() {
        let mut log = AuditLog::new(20);
        log.record(AuditEvent::HealthCheckFailed { pid: 42, attempt: 1 });
        log.record(AuditEvent::ProcessStarted { pid: 99, command: "srv".to_string() });
        log.record(AuditEvent::HealthCheckFailed { pid: 42, attempt: 2 });
        let failed = log.filter_by(|e| matches!(e.event, AuditEvent::HealthCheckFailed { .. }));
        assert_eq!(failed.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut log = AuditLog::new(10);
        log.record(AuditEvent::ProcessStopped { pid: 5, exit_code: Some(0) });
        log.clear();
        assert!(log.is_empty());
    }

    #[test]
    fn test_rollback_event_stored() {
        let mut log = AuditLog::new(10);
        log.record(AuditEvent::RollbackTriggered {
            from_pid: 100,
            to_pid: 99,
            reason: "health check timeout".to_string(),
        });
        assert_eq!(log.len(), 1);
        if let AuditEvent::RollbackTriggered { reason, .. } = &log.entries()[0].event {
            assert_eq!(reason, "health check timeout");
        } else {
            panic!("unexpected event type");
        }
    }
}
