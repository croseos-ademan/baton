#[cfg(test)]
mod tests {
    use super::super::ready::*;
    use std::env;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_from_env_none() {
        env::remove_var("NOTIFY_SOCKET");
        env::remove_var("BATON_READY_FILE");
        let notifier = ReadyNotifier::from_env();
        assert!(matches!(notifier, ReadyNotifier::None));
    }

    #[test]
    fn test_from_env_ready_file() {
        env::remove_var("NOTIFY_SOCKET");
        env::set_var("BATON_READY_FILE", "/tmp/baton_test_ready");
        let notifier = ReadyNotifier::from_env();
        assert!(matches!(notifier, ReadyNotifier::File(_)));
        env::remove_var("BATON_READY_FILE");
    }

    #[test]
    fn test_notify_ready_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("ready");
        let notifier = ReadyNotifier::File(path.to_str().unwrap().to_string());
        notifier.notify_ready().unwrap();
        assert!(path.exists());
        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "ready\n");
    }

    #[test]
    fn test_notify_stopping_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("ready");
        fs::write(&path, b"ready\n").unwrap();
        let notifier = ReadyNotifier::File(path.to_str().unwrap().to_string());
        notifier.notify_stopping().unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn test_notify_none_is_ok() {
        let notifier = ReadyNotifier::None;
        assert!(notifier.notify_ready().is_ok());
        assert!(notifier.notify_stopping().is_ok());
    }
}
