#[cfg(test)]
mod tests {
    use crate::cleanup::CleanupRegistry;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    fn create_temp_file(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
        let path = dir.join(name);
        let mut f = fs::File::create(&path).unwrap();
        writeln!(f, "temp").unwrap();
        path
    }

    #[test]
    fn test_register_and_len() {
        let reg = CleanupRegistry::new();
        assert!(reg.is_empty());
        let dir = tempdir().unwrap();
        let p = create_temp_file(dir.path(), "test1.tmp");
        reg.register(&p);
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn test_run_removes_files() {
        let dir = tempdir().unwrap();
        let p1 = create_temp_file(dir.path(), "a.tmp");
        let p2 = create_temp_file(dir.path(), "b.tmp");

        assert!(p1.exists());
        assert!(p2.exists());

        let reg = CleanupRegistry::new();
        reg.register(&p1);
        reg.register(&p2);
        reg.run();

        assert!(!p1.exists());
        assert!(!p2.exists());
    }

    #[test]
    fn test_run_tolerates_missing_file() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("nonexistent.tmp");
        let reg = CleanupRegistry::new();
        reg.register(&p);
        // Should not panic
        reg.run();
    }

    #[test]
    fn test_clone_shares_registry() {
        let dir = tempdir().unwrap();
        let p = create_temp_file(dir.path(), "shared.tmp");
        let reg = CleanupRegistry::new();
        reg.register(&p);
        let reg2 = reg.clone();
        assert_eq!(reg2.len(), 1);
    }

    #[test]
    fn test_run_clears_registry() {
        let dir = tempdir().unwrap();
        let p = create_temp_file(dir.path(), "clear.tmp");
        let reg = CleanupRegistry::new();
        reg.register(&p);
        assert_eq!(reg.len(), 1);
        reg.run();
        // After run, the registry should be empty
        assert!(reg.is_empty());
    }
}
