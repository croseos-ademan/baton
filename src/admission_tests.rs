#[cfg(test)]
mod tests {
    use crate::admission::{AdmissionController, AdmissionDecision};
    use crate::admission_config::AdmissionConfig;
    use std::time::Duration;

    fn default_config() -> AdmissionConfig {
        AdmissionConfig {
            max_load_threshold: 0.8,
            min_interval: Duration::from_millis(100),
            max_handoffs_per_window: Some(5),
        }
    }

    #[test]
    fn allows_when_conditions_met() {
        let ctrl = AdmissionController::new(default_config());
        assert_eq!(ctrl.evaluate(0.5), AdmissionDecision::Allow);
    }

    #[test]
    fn denies_when_load_too_high() {
        let ctrl = AdmissionController::new(default_config());
        let decision = ctrl.evaluate(0.95);
        match decision {
            AdmissionDecision::Deny(msg) => assert!(msg.contains("load")),
            _ => panic!("expected Deny"),
        }
    }

    #[test]
    fn denies_when_min_interval_not_elapsed() {
        let mut ctrl = AdmissionController::new(default_config());
        ctrl.record_handoff();
        // Immediately evaluate again — interval hasn't elapsed
        let decision = ctrl.evaluate(0.3);
        match decision {
            AdmissionDecision::Deny(msg) => assert!(msg.contains("interval")),
            _ => panic!("expected Deny due to interval"),
        }
    }

    #[test]
    fn allows_after_interval_elapses() {
        let config = AdmissionConfig {
            max_load_threshold: 0.9,
            min_interval: Duration::from_millis(1),
            max_handoffs_per_window: None,
        };
        let mut ctrl = AdmissionController::new(config);
        ctrl.record_handoff();
        std::thread::sleep(Duration::from_millis(10));
        assert_eq!(ctrl.evaluate(0.3), AdmissionDecision::Allow);
    }

    #[test]
    fn denies_when_quota_exhausted() {
        let mut ctrl = AdmissionController::new(AdmissionConfig {
            max_load_threshold: 0.9,
            min_interval: Duration::from_millis(0),
            max_handoffs_per_window: Some(2),
        });
        ctrl.record_handoff();
        ctrl.record_handoff();
        let decision = ctrl.evaluate(0.1);
        match decision {
            AdmissionDecision::Deny(msg) => assert!(msg.contains("quota")),
            _ => panic!("expected Deny due to quota"),
        }
    }

    #[test]
    fn reset_window_clears_count() {
        let mut ctrl = AdmissionController::new(AdmissionConfig {
            max_load_threshold: 0.9,
            min_interval: Duration::from_millis(0),
            max_handoffs_per_window: Some(1),
        });
        ctrl.record_handoff();
        assert_eq!(ctrl.handoff_count(), 1);
        ctrl.reset_window();
        assert_eq!(ctrl.handoff_count(), 0);
        assert_eq!(ctrl.evaluate(0.1), AdmissionDecision::Allow);
    }
}
