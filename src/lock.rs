//! Advisory file locking for preventing concurrent baton instances.

use crate::error::{BatonError, Result};
use std::fs::{File, OpenOptions};
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};

pub struct LockFile {
    path: PathBuf,
    _file: File,
}

impl LockFile {
    /// Acquire an exclusive advisory lock on the given path.
    /// Returns an error if the lock is already held.
    pub fn acquire(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .map_err(|e| BatonError::Io(e))?;

        let fd = file.as_raw_fd();
        let ret = unsafe {
            libc::flock(fd, libc::LOCK_EX | libc::LOCK_NB)
        };

        if ret != 0 {
            let err = std::io::Error::last_os_error();
            if err.raw_os_error() == Some(libc::EWOULDBLOCK) {
                return Err(BatonError::LockHeld(path));
            }
            return Err(BatonError::Io(err));
        }

        Ok(Self { path, _file: file })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for LockFile {
    fn drop(&mut self) {
        // Lock is released automatically when file is closed.
        // Best-effort removal of the lock file.
        let _ = std::fs::remove_file(&self.path);
    }
}
