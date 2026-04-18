//! Unified error type for baton.

use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum BatonError {
    Io(std::io::Error),
    /// A lock file is already held by another process.
    LockHeld(PathBuf),
    /// PID file already exists with a running process.
    PidFileExists(PathBuf),
    /// Handoff timed out waiting for child readiness.
    HandoffTimeout,
    /// Child process exited unexpectedly.
    ChildExited(i32),
    /// Signal delivery failed.
    SignalFailed(i32),
    /// Environment variable error.
    EnvError(String),
    /// Generic configuration error.
    Config(String),
    /// Other error with message.
    Other(String),
}

pub type Result<T> = std::result::Result<T, BatonError>;

impl fmt::Display for BatonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BatonError::Io(e) => write!(f, "I/O error: {}", e),
            BatonError::LockHeld(p) => write!(f, "lock already held: {}", p.display()),
            BatonError::PidFileExists(p) => write!(f, "PID file exists: {}", p.display()),
            BatonError::HandoffTimeout => write!(f, "handoff timed out"),
            BatonError::ChildExited(code) => write!(f, "child exited with code {}", code),
            BatonError::SignalFailed(sig) => write!(f, "failed to send signal {}", sig),
            BatonError::EnvError(msg) => write!(f, "environment error: {}", msg),
            BatonError::Config(msg) => write!(f, "configuration error: {}", msg),
            BatonError::Other(msg) => write!(f, "{}", msg),
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

impl From<std::io::Error> for BatonError {
    fn from(e: std::io::Error) -> Self {
        BatonError::Io(e)
    }
}
