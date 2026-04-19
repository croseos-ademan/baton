/// Configuration for the notify subsystem.
#[derive(Debug, Clone)]
pub struct NotifyConfig {
    /// Whether to send sd_notify messages when available.
    pub enabled: bool,
    /// Override the NOTIFY_SOCKET path (useful for testing).
    pub socket_path: Option<String>,
    /// Send READY=1 after child signals readiness.
    pub notify_ready: bool,
    /// Send STOPPING=1 before initiating handoff.
    pub notify_stopping: bool,
}

impl NotifyConfig {
    pub fn new() -> Self {
        NotifyConfig {
            enabled: true,
            socket_path: None,
            notify_ready: true,
            notify_stopping: true,
        }
    }

    pub fn disabled() -> Self {
        NotifyConfig {
            enabled: false,
            socket_path: None,
            notify_ready: false,
            notify_stopping: false,
        }
    }

    pub fn with_socket_path(mut self, path: &str) -> Self {
        self.socket_path = Some(path.to_string());
        self
    }

    pub fn with_notify_ready(mut self, v: bool) -> Self {
        self.notify_ready = v;
        self
    }

    pub fn with_notify_stopping(mut self, v: bool) -> Self {
        self.notify_stopping = v;
        self
    }
}

impl Default for NotifyConfig {
    fn default() -> Self {
        Self::new()
    }
}
