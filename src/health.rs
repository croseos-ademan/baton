//! Health check support for baton.
//!
//! Polls a child process to determine if it is ready to accept traffic.

use std::time::{Duration, Instant};
use crate::error::{BatonError, Result};

/// Strategy used to determine child health.
#[derive(Debug, Clone)]
pub enum HealthCheck {
    /// Wait for the child to write a newline to its stdout (SD_NOTIFY style).
    ReadyFd,
    /// Poll a TCP port until it accepts a connection.
    TcpPort(u16),
    /// Simply wait a fixed duration.
    Delay(Duration),
}

/// Poll until the health check passes or the deadline is exceeded.
pub fn wait_healthy(check: &HealthCheck, timeout: Duration) -> Result<()> {
    let deadline = Instant::now() + timeout;
    match check {
        HealthCheck::ReadyFd => {
            // Handled separately via ready.rs; nothing to poll here.
            Ok(())
        }
        HealthCheck::TcpPort(port) => poll_tcp(*port, deadline),
        HealthCheck::Delay(d) => {
            std::thread::sleep(*d);
            Ok(())
        }
    }
}

fn poll_tcp(port: u16, deadline: Instant) -> Result<()> {
    use std::net::TcpStream;
    let addr = format!("127.0.0.1:{}", port);
    loop {
        if TcpStream::connect(&addr).is_ok() {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(BatonError::Timeout(format!(
                "health check on port {} timed out",
                port
            )));
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
