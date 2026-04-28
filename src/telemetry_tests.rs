#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::telemetry::{Telemetry, TelemetryEvent};
    use crate::telemetry_config::TelemetryConfig;
    use crate::telemetry_sink::{LogSink, WriterSink, TelemetrySink};
    use crate::error::BatonError;

    fn make_telemetry(enabled: bool) -> Telemetry {
        let config = TelemetryConfig::builder()
            .enabled(enabled)
            .max_events(4)
            .namespace("test")
            .build();
        Telemetry::new(config)
    }

    #[test]
    fn test_record_event() {
        let mut t = make_telemetry(true);
        t.record(TelemetryEvent::new("handoff.start"));
        assert_eq!(t.events().len(), 1);
        assert_eq!(t.events()[0].name, "handoff.start");
    }

    #[test]
    fn test_disabled_does_not_record() {
        let mut t = make_telemetry(false);
        t.record(TelemetryEvent::new("handoff.start"));
        assert_eq!(t.events().len(), 0);
    }

    #[test]
    fn test_max_events_evicts_oldest() {
        let mut t = make_telemetry(true);
        for i in 0..6 {
            t.record(TelemetryEvent::new(format!("event.{}", i)));
        }
        assert_eq!(t.events().len(), 4);
        assert_eq!(t.events()[0].name, "event.2");
    }

    #[test]
    fn test_flush_clears_events() {
        let mut t = make_telemetry(true);
        t.record(TelemetryEvent::new("e1"));
        t.record(TelemetryEvent::new("e2"));
        let flushed = t.flush();
        assert_eq!(flushed.len(), 2);
        assert_eq!(t.events().len(), 0);
    }

    #[test]
    fn test_span_records_ok_status() {
        let mut t = make_telemetry(true);
        let _ = t.span("db.query", || Ok::<_, BatonError>(42));
        assert_eq!(t.events().len(), 1);
        assert_eq!(t.events()[0].labels.get("status").map(String::as_str), Some("ok"));
        assert!(t.events()[0].duration.is_some());
    }

    #[test]
    fn test_span_records_error_status() {
        let mut t = make_telemetry(true);
        let _ = t.span("db.query", || Err::<i32, _>(BatonError::Timeout));
        assert_eq!(t.events()[0].labels.get("status").map(String::as_str), Some("error"));
    }

    #[test]
    fn test_writer_sink_json_output() {
        let buf: Vec<u8> = Vec::new();
        let mut sink = WriterSink::new(buf);
        let event = TelemetryEvent::new("handoff.complete")
            .with_label("pid", "1234")
            .with_duration(Duration::from_millis(42));
        sink.emit(&event);
    }

    #[test]
    fn test_event_with_labels() {
        let event = TelemetryEvent::new("restart")
            .with_label("reason", "signal")
            .with_label("version", "2");
        assert_eq!(event.labels.get("reason").map(String::as_str), Some("signal"));
        assert_eq!(event.labels.get("version").map(String::as_str), Some("2"));
    }
}
