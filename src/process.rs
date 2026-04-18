use std::ffi::CString;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

/// Spawn a new process and optionally signal the old one to terminate.
/// Returns the PID of the newly spawned child process.
pub fn handoff(old_pid: Option<u32>, command: &str, args: &[String]) -> Result<u32, String> {
    let child_pid = spawn(command, args).map_err(|e| format!("spawn error: {}", e))?;

    if let Some(pid) = old_pid {
        signal_terminate(pid).map_err(|e| format!("signal error: {}", e))?;
    }

    Ok(child_pid)
}

fn spawn(command: &str, args: &[String]) -> io::Result<u32> {
    let mut cmd = std::process::Command::new(command);
    cmd.args(args);

    // Inherit stdin/stdout/stderr so the new process can take over the terminal
    cmd.stdin(std::process::Stdio::inherit());
    cmd.stdout(std::process::Stdio::inherit());
    cmd.stderr(std::process::Stdio::inherit());

    let child = cmd.spawn()?;
    Ok(child.id())
}

fn signal_terminate(pid: u32) -> io::Result<()> {
    // Send SIGTERM to the old process
    let ret = unsafe { libc::kill(pid as libc::pid_t, libc::SIGTERM) };
    if ret != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_valid_command() {
        let pid = spawn("true", &[]);
        assert!(pid.is_ok(), "expected spawn to succeed for 'true'");
        assert!(pid.unwrap() > 0);
    }

    #[test]
    fn test_spawn_invalid_command() {
        let result = spawn("__nonexistent_baton_cmd__", &[]);
        assert!(result.is_err(), "expected spawn to fail for nonexistent command");
    }

    #[test]
    fn test_handoff_no_old_pid() {
        let result = handoff(None, "true", &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_signal_invalid_pid() {
        // PID 0 or a very large PID should fail gracefully
        let result = signal_terminate(0xFFFFFF);
        assert!(result.is_err());
    }
}
