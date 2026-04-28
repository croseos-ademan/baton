use std::io::Write;
use crate::telemetry::TelemetryEvent;

pub trait TelemetrySink: Send + Sync {
    fn emit(&mut self, event: &TelemetryEvent);
    fn flush(&mut self) {}
}

pub struct LogSink {
    prefix: String,
}

impl LogSink {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self { prefix: prefix.into() }
    }
}

impl TelemetrySink for LogSink {
    fn emit(&mut self, event: &TelemetryEvent) {
        let duration_str = event.duration
            .map(|d| format!(" duration={}ms", d.as_millis()))
            .unwrap_or_default();
        let labels_str: String = event.labels.iter()
            .map(|(k, v)| format!(" {}={}", k, v))
            .collect();
        eprintln!("[{}] event={}{}{}", self.prefix, event.name, duration_str, labels_str);
    }
}

pub struct WriterSink<W: Write + Send + Sync> {
    writer: W,
}

impl<W: Write + Send + Sync> WriterSink<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W: Write + Send + Sync> TelemetrySink for WriterSink<W> {
    fn emit(&mut self, event: &TelemetryEvent) {
        let duration_ms = event.duration.map(|d| d.as_millis()).unwrap_or(0);
        let line = format!(
            "{{\"event\":\"{}\",\"duration_ms\":{},\"labels\":{{{}}}}}",
            event.name,
            duration_ms,
            event.labels.iter()
                .map(|(k, v)| format!("\"{}\":\"{}\"", k, v))
                .collect::<Vec<_>>()
                .join(",")
        );
        let _ = writeln!(self.writer, "{}", line);
    }

    fn flush(&mut self) {
        let _ = self.writer.flush();
    }
}
