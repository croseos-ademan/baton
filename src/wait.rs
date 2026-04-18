use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq)]
pub enum WaitResult {
    Exited(i32),
    Signaled(i32),
    Timeout,
    StillRunning,
}

pub fn wait_for_exit(pid: u32, timeout: Duration) -> WaitResult {
    let deadline = Instant::now() + timeout;
    let nix_pid = Pid::from_raw(pid as i32);

    loop {
        match waitpid(nix_pid, Some(WaitPidFlag::WNOHANG)) {
            Ok(WaitStatus::Exited(_, code)) => return WaitResult::Exited(code),
            Ok(WaitStatus::Signaled(_, sig, _)) => return WaitResult::Signaled(sig as i32),
            Ok(WaitStatus::StillAlive) => {
                if Instant::now() >= deadline {
                    return WaitResult::Timeout;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Ok(_) => continue,
            Err(_) => return WaitResult::Exited(0),
        }
    }
}

pub fn process_exists(pid: u32) -> bool {
    let path = format!("/proc/{}/status", pid);
    std::path::Path::new(&path).exists()
}

pub fn wait_until_gone(pid: u32, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if !process_exists(pid) {
            return true;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    !process_exists(pid)
}
