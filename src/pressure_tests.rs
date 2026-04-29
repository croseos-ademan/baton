#[cfg(test)]
mod tests {
    use super::super::pressure::{PressureConfig, PressureHandle};
    use std::time::Duration;

    fn default_handle() -> PressureHandle {
        PressureHandle::new(PressureConfig {
            queue_limit: 5,
            hold_duration: Duration::from_millis(100),
            poll_interval: Duration::from_millis(10),
        })
    }

    #[test]
    fn starts_inactive() {
        let h = default_handle();
        assert!(!h.is_active());
        assert_eq!(h.depth(), 0);
    }

    #[test]
    fn activates_at_limit() {
        let h = default_handle();
        for _ in 0..4 {
            h.enqueue();
            assert!(!h.is_active(), "should not be active below limit");
        }
        h.enqueue(); // depth == 5 == queue_limit
        assert!(h.is_active());
    }

    #[test]
    fn stays_active_during_hold_even_if_depth_drops() {
        let h = default_handle();
        for _ in 0..5 {
            h.enqueue();
        }
        assert!(h.is_active());
        h.dequeue(); // depth back to 4, but hold not elapsed
        assert!(h.is_active(), "should stay active within hold window");
    }

    #[test]
    fn clears_after_hold_duration() {
        let h = default_handle();
        for _ in 0..5 {
            h.enqueue();
        }
        for _ in 0..5 {
            h.dequeue();
        }
        std::thread::sleep(Duration::from_millis(150));
        assert!(!h.is_active(), "should clear after hold duration");
    }

    #[test]
    fn release_clears_immediately() {
        let h = default_handle();
        for _ in 0..5 {
            h.enqueue();
        }
        assert!(h.is_active());
        h.release();
        assert!(!h.is_active());
        assert_eq!(h.depth(), 0);
    }

    #[test]
    fn depth_tracks_enqueue_dequeue() {
        let h = default_handle();
        h.enqueue();
        h.enqueue();
        assert_eq!(h.depth(), 2);
        h.dequeue();
        assert_eq!(h.depth(), 1);
    }

    #[test]
    fn reactivates_after_release_and_new_load() {
        let h = default_handle();
        for _ in 0..5 {
            h.enqueue();
        }
        h.release();
        assert!(!h.is_active());
        for _ in 0..5 {
            h.enqueue();
        }
        assert!(h.is_active());
    }
}
