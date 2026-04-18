//! Spawning child processes with inherited file descriptors and handoff environment.

use std::{
    io,
    os::unix::process::CommandExt,
    process::Command,
};
use crate::env::HandoffEnv;

/// Configuration for spawning a child process during handoff.
pub struct ChildConfig<'a> {
    pub argv: &'a [String],
    pub env: &'a HandoffEnv,
}

/// Spawn the child process, injecting handoff environment variables.
/// The caller is responsible for ensuring inherited FDs are not CLOEXEC.
pub fn spawn_child(cfg: &ChildConfig) -> io::Result<u32> {
    if cfg.argv.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "argv is empty"));
    }

    let mut cmd = Command::new(&cfg.argv[0]);
    if cfg.argv.len() > 1 {
        cmd.args(&cfg.argv[1..]);
    }

    for (k, v) in cfg.env.to_vars() {
        cmd.env(k, v);
    }

    // Ensure the child starts in the same working directory.
    if let Ok(cwd) = std::env::current_dir() {
        cmd.current_dir(cwd);
    }

    let child = cmd.spawn()?;
    Ok(child.id())
}

/// Build a `Command` pre-configured for handoff without spawning it.
/// Useful for testing or deferred execution.
pub fn build_command(cfg: &ChildConfig) -> io::Result<Command> {
    if cfg.argv.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "argv is empty"));
    }
    let mut cmd = Command::new(&cfg.argv[0]);
    if cfg.argv.len() > 1 {
        cmd.args(&cfg.argv[1..]);
    }
    for (k, v) in cfg.env.to_vars() {
        cmd.env(k, v);
    }
    Ok(cmd)
}
