use crate::error::BatonError;
use std::env;
use std::io::Write;
use std::os::unix::net::UnixDatagram;

/// Sends sd_notify style messages to a supervisor or parent process.
pub struct Notifier {
    socket_path: Option<String>,
}

impl Notifier {
    pub fn new() -> Self {
        let socket_path = env::var("NOTIFY_SOCKET").ok();
        Notifier { socket_path }
    }

    pub fn from_path(path: &str) -> Self {
        Notifier {
            socket_path: Some(path.to_string()),
        }
    }

    pub fn is_available(&self) -> bool {
        self.socket_path.is_some()
    }

    pub fn send(&self, msg: &str) -> Result<(), BatonError> {
        let path = self.socket_path.as_deref().ok_or_else(|| {
            BatonError::NotifyError("NOTIFY_SOCKET not set".to_string())
        })?;
        let sock = UnixDatagram::unbound()
            .map_err(|e| BatonError::NotifyError(e.to_string()))?;
        sock.send_to(msg.as_bytes(), path)
            .map_err(|e| BatonError::NotifyError(e.to_string()))?;
        Ok(())
    }

    pub fn ready(&self) -> Result<(), BatonError> {
        self.send("READY=1\n")
    }

    pub fn reloading(&self) -> Result<(), BatonError> {
        self.send("RELOADING=1\n")
    }

    pub fn stopping(&self) -> Result<(), BatonError> {
        self.send("STOPPING=1\n")
    }

    pub fn status(&self, msg: &str) -> Result<(), BatonError> {
        self.send(&format!("STATUS={}\n", msg))
    }
}

impl Default for Notifier {
    fn default() -> Self {
        Self::new()
    }
}
