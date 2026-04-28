//! Fence module: ensures only one handoff is in progress at a time.
//! A fence acts as a distributed-style guard using an atomic file + lock
//! to prevent concurrent process handoffs from racing each other.

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::error::BatonError;

/// A handoff fence that prevents concurrent handoff operations.
pub struct Fence {
    path: PathBuf,
    created_at: u64,
}

impl Fence {
    /// Attempt to acquire the fence. Returns `Err` if already held.
    pub fn acquire(dir: &Path) -> Result<Self, BatonError> {
        let path = dir.join(".baton_fence");
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs();

        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| match e.kind() {
                io::ErrorKind::AlreadyExists => {
                    BatonError::FenceAlreadyHeld(path.display().to_string())
                }
                _ => BatonError::Io(e),
            })?;

        writeln!(file, "{}", now).map_err(BatonError::Io)?;

        Ok(Fence { path, created_at: now })
    }

    /// Return when the fence was acquired (Unix seconds).
    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    /// Explicitly release the fence.
    pub fn release(self) -> Result<(), BatonError> {
        fs::remove_file(&self.path).map_err(BatonError::Io)
    }

    /// Check whether a stale fence exists older than `max_age`.
    pub fn is_stale(dir: &Path, max_age: Duration) -> bool {
        let path = dir.join(".baton_fence");
        match fs::read_to_string(&path) {
            Ok(contents) => {
                let ts: u64 = contents.trim().parse().unwrap_or(0);
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or(Duration::ZERO)
                    .as_secs();
                now.saturating_sub(ts) > max_age.as_secs()
            }
            Err(_) => false,
        }
    }

    /// Remove a stale fence unconditionally (use with care).
    pub fn force_release(dir: &Path) -> Result<(), BatonError> {
        let path = dir.join(".baton_fence");
        fs::remove_file(&path).map_err(BatonError::Io)
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
