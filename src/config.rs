use std::time::Duration;

/// Configuration for baton process handoff
#[derive(Debug, Clone)]
pub struct Config {
    /// Path to the PID file
    pub pid_file: Option<String>,
    /// Path to the Unix socket used for FD passing
    pub socket_path: String,
    /// Timeout waiting for child to become ready
    pub ready_timeout: Duration,
    /// Timeout waiting for old process to exit
    pub exit_timeout: Duration,
    /// Environment variable child sets to signal readiness
    pub ready_env_var: String,
    /// Whether to pass all file descriptors to child
    pub pass_fds: bool,
}

impl Config {
    pub fn new(socket_path: impl Into<String>) -> Self {
        Config {
            pid_file: None,
            socket_path: socket_path.into(),
            ready_timeout: Duration::from_secs(30),
            exit_timeout: Duration::from_secs(10),
            ready_env_var: "READY".to_string(),
            pass_fds: true,
        }
    }

    pub fn with_pid_file(mut self, path: impl Into<String>) -> Self {
        self.pid_file = Some(path.into());
        self
    }

    pub fn with_ready_timeout(mut self, timeout: Duration) -> Self {
        self.ready_timeout = timeout;
        self
    }

    pub fn with_exit_timeout(mut self, timeout: Duration) -> Self {
        self.exit_timeout = timeout;
        self
    }

    pub fn with_ready_env_var(mut self, var: impl Into<String>) -> Self {
        self.ready_env_var = var.into();
        self
    }

    pub fn with_pass_fds(mut self, pass_fds: bool) -> Self {
        self.pass_fds = pass_fds;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new("/tmp/baton.sock")
    }
}
