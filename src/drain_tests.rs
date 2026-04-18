#[cfg(test)]
mod tests {
    use super::super::drain::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    fn fast_config() -> DrainConfig {
        DrainConfig {
            timeout: Duration::from_millis(200),
            poll_interval: Duration::from_millis(10),
        }
    }

    #[test]
    fn drain_completes_immediately_when_no_connections() {
        let config = fast_config();
        let state = wait_for_drain(&config, || 0);
        assert_eq!(state, DrainState::Complete);
    }

    #[test]
    fn drain_completes_after_connections_drop() {
        let config = fast_config();
        let counter = Arc::new(AtomicUsize::new(3));
        let c = counter.clone();
        // Decrement on each poll until zero
        let state = wait_for_drain(&config, move || {
            let v = c.load(Ordering::SeqCst);
            if v > 0 {
                c.fetch_sub(1, Ordering::SeqCst);
            }
            v
        });
        assert_eq!(state, DrainState::Complete);
    }

    #[test]
    fn drain_times_out_when_connections_remain() {
        let config = fast_config();
        let state = wait_for_drain(&config, || 5);
        assert_eq!(state, DrainState::TimedOut);
    }

    #[test]
    fn enforce_drain_ok_when_complete() {
        let config = fast_config();
        let result = enforce_drain(&config, || 0);
        assert!(result.is_ok());
    }

    #[test]
    fn enforce_drain_err_when_timed_out() {
        let config = fast_config();
        let result = enforce_drain(&config, || 1);
        assert!(result.is_err());
    }
}
