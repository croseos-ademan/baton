#[cfg(test)]
mod tests {
    use std::env;
    use crate::env::{HandoffEnv, ENV_HANDOFF, ENV_PID_FILE, ENV_SOCKET_FD};

    fn clean() {
        env::remove_var(ENV_HANDOFF);
        env::remove_var(ENV_SOCKET_FD);
        env::remove_var(ENV_PID_FILE);
    }

    #[test]
    fn test_from_env_empty() {
        clean();
        let e = HandoffEnv::from_env();
        assert!(!e.is_handoff);
        assert!(e.socket_fd.is_none());
        assert!(e.pid_file.is_none());
    }

    #[test]
    fn test_from_env_populated() {
        clean();
        env::set_var(ENV_HANDOFF, "1");
        env::set_var(ENV_SOCKET_FD, "7");
        env::set_var(ENV_PID_FILE, "/run/app.pid");
        let e = HandoffEnv::from_env();
        assert!(e.is_handoff);
        assert_eq!(e.socket_fd, Some(7));
        assert_eq!(e.pid_file.as_deref(), Some("/run/app.pid"));
        clean();
    }

    #[test]
    fn test_to_vars_roundtrip() {
        let e = HandoffEnv {
            socket_fd: Some(5),
            pid_file: Some("/tmp/test.pid".to_string()),
            is_handoff: true,
        };
        let vars = e.to_vars();
        assert!(vars.iter().any(|(k, v)| k == ENV_HANDOFF && v == "1"));
        assert!(vars.iter().any(|(k, v)| k == ENV_SOCKET_FD && v == "5"));
        assert!(vars.iter().any(|(k, v)| k == ENV_PID_FILE && v == "/tmp/test.pid"));
    }

    #[test]
    fn test_clear() {
        env::set_var(ENV_HANDOFF, "1");
        env::set_var(ENV_SOCKET_FD, "3");
        HandoffEnv::clear();
        assert!(env::var(ENV_HANDOFF).is_err());
        assert!(env::var(ENV_SOCKET_FD).is_err());
    }
}
