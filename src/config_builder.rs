use std::time::Duration;
use crate::config::Config;

/// Builder for constructing a [`Config`] from environment variables and CLI-style args.
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    socket_path: Option<String>,
    pid_file: Option<String>,
    ready_timeout_secs: Option<u64>,
    exit_timeout_secs: Option<u64>,
    ready_env_var: Option<String>,
    pass_fds: Option<bool>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn socket_path(mut self, v: impl Into<String>) -> Self {
        self.socket_path = Some(v.into());
        self
    }

    pub fn pid_file(mut self, v: impl Into<String>) -> Self {
        self.pid_file = Some(v.into());
        self
    }

    pub fn ready_timeout_secs(mut self, secs: u64) -> Self {
        self.ready_timeout_secs = Some(secs);
        self
    }

    pub fn exit_timeout_secs(mut self, secs: u64) -> Self {
        self.exit_timeout_secs = Some(secs);
        self
    }

    pub fn ready_env_var(mut self, v: impl Into<String>) -> Self {
        self.ready_env_var = Some(v.into());
        self
    }

    pub fn pass_fds(mut self, v: bool) -> Self {
        self.pass_fds = Some(v);
        self
    }

    /// Populate unset fields from environment variables.
    pub fn from_env(mut self) -> Self {
        if self.socket_path.is_none() {
            self.socket_path = std::env::var("BATON_SOCKET").ok();
        }
        if self.pid_file.is_none() {
            self.pid_file = std::env::var("BATON_PID_FILE").ok();
        }
        if self.ready_timeout_secs.is_none() {
            self.ready_timeout_secs = std::env::var("BATON_READY_TIMEOUT")
                .ok()
                .and_then(|v| v.parse().ok());
        }
        if self.exit_timeout_secs.is_none() {
            self.exit_timeout_secs = std::env::var("BATON_EXIT_TIMEOUT")
                .ok()
                .and_then(|v| v.parse().ok());
        }
        if self.ready_env_var.is_none() {
            self.ready_env_var = std::env::var("BATON_READY_VAR").ok();
        }
        self
    }

    pub fn build(self) -> Config {
        let socket_path = self.socket_path.unwrap_or_else(|| "/tmp/baton.sock".to_string());
        let mut cfg = Config::new(socket_path);
        if let Some(p) = self.pid_file {
            cfg = cfg.with_pid_file(p);
        }
        if let Some(s) = self.ready_timeout_secs {
            cfg = cfg.with_ready_timeout(Duration::from_secs(s));
        }
        if let Some(s) = self.exit_timeout_secs {
            cfg = cfg.with_exit_timeout(Duration::from_secs(s));
        }
        if let Some(v) = self.ready_env_var {
            cfg = cfg.with_ready_env_var(v);
        }
        if let Some(v) = self.pass_fds {
            cfg = cfg.with_pass_fds(v);
        }
        cfg
    }
}
