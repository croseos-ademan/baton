#[cfg(test)]
mod tests {
    use crate::shedding::{LoadShedder, SheddingDecision};
    use crate::shedding_config::SheddingConfig;

    fn make_shedder(max_in_flight: u64) -> LoadShedder {
        LoadShedder::new(SheddingConfig::new(max_in_flight))
    }

    #[test]
    fn test_allow_when_below_limit() {
        let shedder = make_shedder(10);
        assert_eq!(shedder.check(), SheddingDecision::Allow);
    }

    #[test]
    fn test_shed_when_at_limit() {
        let shedder = make_shedder(2);
        let _g1 = shedder.acquire().expect("first acquire should succeed");
        let _g2 = shedder.acquire().expect("second acquire should succeed");
        assert_eq!(
            shedder.check(),
            SheddingDecision::Shed { reason: "max in-flight exceeded" }
        );
    }

    #[test]
    fn test_acquire_returns_none_when_full() {
        let shedder = make_shedder(1);
        let _g = shedder.acquire().expect("first acquire should succeed");
        assert!(shedder.acquire().is_none());
    }

    #[test]
    fn test_in_flight_decrements_on_drop() {
        let shedder = make_shedder(5);
        {
            let _g = shedder.acquire().unwrap();
            assert_eq!(shedder.in_flight(), 1);
        }
        assert_eq!(shedder.in_flight(), 0);
    }

    #[test]
    fn test_rejected_total_increments() {
        let shedder = make_shedder(1);
        let _g = shedder.acquire().unwrap();
        let _ = shedder.acquire(); // should be rejected
        assert_eq!(shedder.rejected_total(), 1);
    }

    #[test]
    fn test_config_validation_zero_in_flight() {
        let config = SheddingConfig::new(0);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_zero_rps() {
        let config = SheddingConfig::new(100).with_max_rps(0);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_valid() {
        let config = SheddingConfig::new(100).with_max_rps(500);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_multiple_guards_track_correctly() {
        let shedder = make_shedder(10);
        let guards: Vec<_> = (0..5).filter_map(|_| shedder.acquire()).collect();
        assert_eq!(shedder.in_flight(), 5);
        drop(guards);
        assert_eq!(shedder.in_flight(), 0);
    }
}
