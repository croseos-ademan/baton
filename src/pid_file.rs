use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Manages a PID file for the running process.
pub struct PidFile {
    path: PathBuf,
}

impl PidFile {
    /// Create and write a PID file at the given path.
    pub fn create<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let pid = std::process::id();
        let mut file = File::create(&path)?;
        writeln!(file, "{}", pid)?;
        Ok(PidFile { path })
    }

    /// Read the PID stored in an existing PID file.
    pub fn read<P: AsRef<Path>>(path: P) -> io::Result<u32> {
        let contents = fs::read_to_string(path)?;
        contents
            .trim()
            .parse::<u32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Return the path of this PID file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Remove the PID file explicitly.
    pub fn remove(&self) -> io::Result<()> {
        fs::remove_file(&self.path)
    }
}

impl Drop for PidFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
