#[cfg(test)]
mod tests {
    use crate::lock::LockFile;
    use tempfile::tempdir;

    #[test]
    fn test_acquire_lock_creates_file() {
        let dir = tempdir().unwrap();
        let lock_path = dir.path().join("test.lock");
        let lock = LockFile::acquire(&lock_path).unwrap();
        assert!(lock_path.exists());
        assert_eq!(lock.path(), lock_path);
    }

    #[test]
    fn test_acquire_lock_exclusive() {
        let dir = tempdir().unwrap();
        let lock_path = dir.path().join("exclusive.lock");
        let _lock1 = LockFile::acquire(&lock_path).unwrap();
        let result = LockFile::acquire(&lock_path);
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("lock") || msg.contains("held") || msg.contains("exclusive"));
    }

    #[test]
    fn test_lock_released_on_drop() {
        let dir = tempdir().unwrap();
        let lock_path = dir.path().join("drop.lock");
        {
            let _lock = LockFile::acquire(&lock_path).unwrap();
        }
        // After drop, lock file should be removed and re-acquisition should succeed.
        let result = LockFile::acquire(&lock_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lock_file_removed_on_drop() {
        let dir = tempdir().unwrap();
        let lock_path = dir.path().join("cleanup.lock");
        {
            let _lock = LockFile::acquire(&lock_path).unwrap();
            assert!(lock_path.exists());
        }
        assert!(!lock_path.exists());
    }
}
