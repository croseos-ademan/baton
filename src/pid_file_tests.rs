#[cfg(test)]
mod tests {
    use crate::pid_file::PidFile;
    use std::fs;

    fn tmp_path(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(name)
    }

    #[test]
    fn test_create_writes_current_pid() {
        let path = tmp_path("baton_test_pid_create.pid");
        let pid_file = PidFile::create(&path).expect("create pid file");
        let written = PidFile::read(&path).expect("read pid file");
        assert_eq!(written, std::process::id());
        drop(pid_file);
    }

    #[test]
    fn test_drop_removes_file() {
        let path = tmp_path("baton_test_pid_drop.pid");
        {
            let _pid_file = PidFile::create(&path).expect("create pid file");
            assert!(path.exists(), "pid file should exist while held");
        }
        assert!(!path.exists(), "pid file should be removed after drop");
    }

    #[test]
    fn test_explicit_remove() {
        let path = tmp_path("baton_test_pid_remove.pid");
        let pid_file = PidFile::create(&path).expect("create pid file");
        pid_file.remove().expect("explicit remove");
        assert!(!path.exists());
        // Prevent double-remove panic in drop
        std::mem::forget(pid_file);
    }

    #[test]
    fn test_read_missing_file_errors() {
        let path = tmp_path("baton_test_pid_missing.pid");
        let _ = fs::remove_file(&path);
        assert!(PidFile::read(&path).is_err());
    }
}
