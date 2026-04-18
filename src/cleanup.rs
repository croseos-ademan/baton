//! Cleanup handler for removing temporary resources on exit.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone)]
pub struct CleanupRegistry {
    paths: Arc<Mutex<Vec<PathBuf>>>,
}

impl CleanupRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a path to be removed on cleanup.
    pub fn register<P: AsRef<Path>>(&self, path: P) {
        let mut paths = self.paths.lock().unwrap();
        paths.push(path.as_ref().to_path_buf());
    }

    /// Remove all registered paths, logging errors but continuing.
    pub fn run(&self) {
        let paths = self.paths.lock().unwrap();
        for path in paths.iter() {
            if path.exists() {
                if let Err(e) = std::fs::remove_file(path) {
                    eprintln!("baton: cleanup failed for {}: {}", path.display(), e);
                }
            }
        }
    }

    /// Return the number of registered paths.
    pub fn len(&self) -> usize {
        self.paths.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Drop for CleanupRegistry {
    fn drop(&mut self) {
        // Only run cleanup if this is the last Arc reference.
        if Arc::strong_count(&self.paths) == 1 {
            self.run();
        }
    }
}
