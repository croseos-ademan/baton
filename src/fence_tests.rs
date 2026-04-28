//! Tests for the fence module.

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tempfile::TempDir;

    use crate::fence::Fence;
    use crate::fence_config::FenceConfig;

    fn tmp() -> TempDir {
        tempfile::tempdir().expect("tempdir")
    }

    #[test]
    fn acquire_and_release() {
        let dir = tmp();
        let fence = Fence::acquire(dir.path()).expect("acquire");
        assert!(dir.path().join(".baton_fence").exists());
        fence.release().expect("release");
        assert!(!dir.path().join(".baton_fence").exists());
    }

    #[test]
    fn double_acquire_fails() {
        let dir = tmp();
        let _f1 = Fence::acquire(dir.path()).expect("first acquire");
        let result = Fence::acquire(dir.path());
        assert!(result.is_err(), "second acquire should fail");
    }

    #[test]
    fn drop_releases_fence() {
        let dir = tmp();
        {
            let _f = Fence::acquire(dir.path()).expect("acquire");
            assert!(dir.path().join(".baton_fence").exists());
        }
        assert!(!dir.path().join(".baton_fence").exists());
    }

    #[test]
    fn stale_detection() {
        let dir = tmp();
        // Write a fence with timestamp 0 (epoch) — always stale
        std::fs::write(dir.path().join(".baton_fence"), "0\n").unwrap();
        assert!(Fence::is_stale(dir.path(), Duration::from_secs(1)));
    }

    #[test]
    fn fresh_fence_not_stale() {
        let dir = tmp();
        let _f = Fence::acquire(dir.path()).expect("acquire");
        assert!(!Fence::is_stale(dir.path(), Duration::from_secs(3600)));
    }

    #[test]
    fn force_release_removes_file() {
        let dir = tmp();
        std::fs::write(dir.path().join(".baton_fence"), "0\n").unwrap();
        Fence::force_release(dir.path()).expect("force release");
        assert!(!dir.path().join(".baton_fence").exists());
    }

    #[test]
    fn fence_config_validate_ok() {
        let dir = tmp();
        let cfg = FenceConfig::new(dir.path().to_path_buf());
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn fence_config_validate_missing_dir() {
        let cfg = FenceConfig::new("/nonexistent/path/xyz".into());
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn fence_config_validate_zero_stale() {
        let dir = tmp();
        let cfg = FenceConfig::new(dir.path().to_path_buf())
            .stale_after(Duration::ZERO);
        assert!(cfg.validate().is_err());
    }
}
