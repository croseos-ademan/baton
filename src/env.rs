//! Environment variable passing between old and new process instances.
//!
//! When performing a handoff, certain environment variables are used to
//! communicate state (e.g., inherited file descriptors) from the parent
//! to the child process.

use std::env;

/// Key used to pass the listening socket FD number to the child.
pub const ENV_SOCKET_FD: &str = "BATON_SOCKET_FD";

/// Key used to signal the child that it is being launched as part of a handoff.
pub const ENV_HANDOFF: &str = "BATON_HANDOFF";

/// Key used to pass the PID file path to the child.
pub const ENV_PID_FILE: &str = "BATON_PID_FILE";

/// Encapsulates the environment state injected during a handoff.
#[derive(Debug, Clone)]
pub struct HandoffEnv {
    pub socket_fd: Option<i32>,
    pub pid_file: Option<String>,
    pub is_handoff: bool,
}

impl HandoffEnv {
    /// Read handoff environment from the current process environment.
    pub fn from_env() -> Self {
        let socket_fd = env::var(ENV_SOCKET_FD)
            .ok()
            .and_then(|v| v.parse::<i32>().ok());
        let pid_file = env::var(ENV_PID_FILE).ok();
        let is_handoff = env::var(ENV_HANDOFF).map(|v| v == "1").unwrap_or(false);
        HandoffEnv { socket_fd, pid_file, is_handoff }
    }

    /// Build a list of (key, value) pairs to inject into a child process.
    pub fn to_vars(&self) -> Vec<(String, String)> {
        let mut vars = Vec::new();
        vars.push((ENV_HANDOFF.to_string(), "1".to_string()));
        if let Some(fd) = self.socket_fd {
            vars.push((ENV_SOCKET_FD.to_string(), fd.to_string()));
        }
        if let Some(ref path) = self.pid_file {
            vars.push((ENV_PID_FILE.to_string(), path.clone()));
        }
        vars
    }

    /// Apply these vars to the current process environment.
    pub fn apply(&self) {
        for (k, v) in self.to_vars() {
            env::set_var(k, v);
        }
    }

    /// Remove handoff-related vars from the current process environment.
    pub fn clear() {
        env::remove_var(ENV_SOCKET_FD);
        env::remove_var(ENV_HANDOFF);
        env::remove_var(ENV_PID_FILE);
    }
}
