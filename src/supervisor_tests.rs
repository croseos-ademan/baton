#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::backoff::Backoff;
    use crate::supervisor::{Supervisor, SupervisorDecision};

    fn make_backoff() -> Backoff {
        Backoff::new(Duration::from_millis(10), Duration::from_millis(100), 2.0)
    }

    #[test]
    fn test_first_exit_triggers_restart() {
        let mut sup = Supervisor::new(3, Duration::from_secs(60), make_backoff());
        let decision = sup.record_exit().unwrap();
        assert_eq!(decision, SupervisorDecision::Restart);
        assert_eq!(sup.restart_count(), 1);
    }

    #[test]
    fn test_exceeding_max_restarts_gives_up() {
        let mut sup = Supervisor::new(2, Duration::from_secs(60), make_backoff());
        sup.record_exit().unwrap();
        sup.record_exit().unwrap();
        let decision = sup.record_exit().unwrap();
        assert_eq!(decision, SupervisorDecision::GiveUp);
    }

    #[test]
    fn test_window_reset_clears_count() {
        let mut sup = Supervisor::new(1, Duration::from_millis(1), make_backoff());
        sup.record_exit().unwrap();
        // Exceed window
        std::thread::sleep(Duration::from_millis(5));
        let decision = sup.record_exit().unwrap();
        assert_eq!(decision, SupervisorDecision::Restart);
        assert_eq!(sup.restart_count(), 1);
    }

    #[test]
    fn test_next_delay_increases_with_backoff() {
        let mut sup = Supervisor::new(5, Duration::from_secs(60), make_backoff());
        let d1 = sup.next_delay();
        let d2 = sup.next_delay();
        assert!(d2 >= d1);
    }

    #[test]
    fn test_reset_backoff_restores_initial_delay() {
        let mut sup = Supervisor::new(5, Duration::from_secs(60), make_backoff());
        sup.next_delay();
        sup.next_delay();
        sup.reset_backoff();
        let d = sup.next_delay();
        assert_eq!(d, Duration::from_millis(10));
    }
}
