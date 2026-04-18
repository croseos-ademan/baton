use std::fmt;
use std::io;

/// Central error type for baton operations.
#[derive(Debug)]
pub enum BatonError {
    /// I/O error from the OS.
    Io(io::Error),
    /// A PID file operation failed.
    PidFile(String),
    /// Socket setup or passing failed.
    Socket(String),
    /// Process handoff failed.
    Handoff(String),
    /// Signal handling error.
    Signal(String),
    /// Configuration error.
    Config(String),
    /// Child process error.
    Child(String),
    /// Timeout waiting for readiness.
    ReadyTimeout,
}

impl fmt::Display for BatonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BatonError::Io(e) => write!(f, "I/O error: {}", e),
            BatonError::PidFile(msg) => write!(f, "PID file error: {}", msg),
            BatonError::Socket(msg) => write!(f, "Socket error: {}", msg),
            BatonError::Handoff(msg) => write!(f, "Handoff error: {}", msg),
            BatonError::Signal(msg) => write!(f, "Signal error: {}", msg),
            BatonError::Config(msg) => write!(f, "Config error: {}", msg),
            BatonError::Child(msg) => write!(f, "Child error: {}", msg),
            BatonError::ReadyTimeout => write!(f, "Timed out waiting for process readiness"),
        }
    }
}

impl std::error::Error for BatonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BatonError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for BatonError {
    fn from(e: io::Error) -> Self {
        BatonError::Io(e)
    }
}

/// Convenience alias used throughout baton.
pub type Result<T> = std::result::Result<T, BatonError>;
