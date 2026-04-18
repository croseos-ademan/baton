#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::config::Config;
    use crate::config_builder::ConfigBuilder;

    #[test]
    fn default_config_has_sensible_values() {
        let cfg = Config::default();
        assert_eq!(cfg.socket_path, "/tmp/baton.sock");
        assert_eq!(cfg.ready_timeout, Duration::from_secs(30));
        assert_eq!(cfg.exit_timeout, Duration::from_secs(10));
        assert_eq!(cfg.ready_env_var, "READY");
        assert!(cfg.pass_fds);
        assert!(cfg.pid_file.is_none());
    }

    #[test]
    fn builder_overrides_defaults() {
        let cfg = ConfigBuilder::new()
            .socket_path("/run/myapp.sock")
            .pid_file("/run/myapp.pid")
            .ready_timeout_secs(60)
            .exit_timeout_secs(5)
            .ready_env_var("LISTENING")
            .pass_fds(false)
            .build();

        assert_eq!(cfg.socket_path, "/run/myapp.sock");
        assert_eq!(cfg.pid_file.as_deref(), Some("/run/myapp.pid"));
        assert_eq!(cfg.ready_timeout, Duration::from_secs(60));
        assert_eq!(cfg.exit_timeout, Duration::from_secs(5));
        assert_eq!(cfg.ready_env_var, "LISTENING");
        assert!(!cfg.pass_fds);
    }

    #[test]
    fn builder_from_env_reads_variables() {
        std::env::set_var("BATON_SOCKET", "/tmp/env_test.sock");
        std::env::set_var("BATON_READY_TIMEOUT", "45");
        std::env::set_var("BATON_PID_FILE", "/tmp/env_test.pid");

        let cfg = ConfigBuilder::new().from_env().build();

        assert_eq!(cfg.socket_path, "/tmp/env_test.sock");
        assert_eq!(cfg.ready_timeout, Duration::from_secs(45));
        assert_eq!(cfg.pid_file.as_deref(), Some("/tmp/env_test.pid"));

        std::env::remove_var("BATON_SOCKET");
        std::env::remove_var("BATON_READY_TIMEOUT");
        std::env::remove_var("BATON_PID_FILE");
    }

    #[test]
    fn explicit_builder_values_take_precedence_over_env() {
        std::env::set_var("BATON_SOCKET", "/tmp/should_be_ignored.sock");

        let cfg = ConfigBuilder::new()
            .socket_path("/tmp/explicit.sock")
            .from_env()
            .build();

        assert_eq!(cfg.socket_path, "/tmp/explicit.sock");

        std::env::remove_var("BATON_SOCKET");
    }
}
