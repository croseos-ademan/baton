#[cfg(test)]
mod tests {
    use crate::metrics::Metrics;
    use crate::metrics_config::{MetricsConfig, MetricsFormat};
    use crate::metrics_reporter::MetricsReporter;
    use crate::metrics_writer::MetricsWriter;
    use std::time::Duration;

    #[test]
    fn test_record_handoff_success() {
        let m = Metrics::new();
        m.record_handoff(Duration::from_millis(42), true);
        let s = m.snapshot();
        assert_eq!(s.handoffs_total, 1);
        assert_eq!(s.handoff_failures, 0);
        assert_eq!(s.last_handoff_duration_ms, 42);
    }

    #[test]
    fn test_record_handoff_failure() {
        let m = Metrics::new();
        m.record_handoff(Duration::from_millis(10), false);
        let s = m.snapshot();
        assert_eq!(s.handoffs_total, 1);
        assert_eq!(s.handoff_failures, 1);
    }

    #[test]
    fn test_record_health_check() {
        let m = Metrics::new();
        m.record_health_check(true);
        m.record_health_check(false);
        let s = m.snapshot();
        assert_eq!(s.health_checks_total, 2);
        assert_eq!(s.health_check_failures, 1);
    }

    #[test]
    fn test_reporter_format_text() {
        let m = Metrics::new();
        m.record_restart();
        let r = MetricsReporter::new(m);
        let text = r.format_text();
        assert!(text.contains("restarts_total=1"));
    }

    #[test]
    fn test_reporter_format_json() {
        let m = Metrics::new();
        let r = MetricsReporter::new(m);
        let json = r.format_json();
        assert!(json.starts_with('{'));
        assert!(json.contains("\"restarts_total\":0"));
    }

    #[test]
    fn test_writer_disabled_skips_write() {
        let m = Metrics::new();
        let cfg = MetricsConfig::new();
        let w = MetricsWriter::new(cfg, m);
        assert!(w.write().is_ok());
    }

    #[test]
    fn test_writer_json_format() {
        let m = Metrics::new();
        let cfg = MetricsConfig::new().with_enabled(true).with_format(MetricsFormat::Json);
        let w = MetricsWriter::new(cfg, m);
        assert!(w.snapshot_json().contains("handoffs_total"));
    }
}
