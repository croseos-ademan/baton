//! Orchestrates the socket handoff from old process to new process.

use crate::fd_passing::{recv_fds, send_fds};
use crate::socket::PassableSocket;
use nix::sys::socket::{socketpair, AddressFamily, SockFlag, SockType};
use std::io;
use std::os::unix::io::RawFd;

pub const BATON_FDS_ENV: &str = "BATON_FDS";
pub const BATON_SOCK_ENV: &str = "BATON_SOCK";

/// Create a socketpair for fd handoff coordination.
pub fn create_handoff_channel() -> io::Result<(RawFd, RawFd)> {
    socketpair(
        AddressFamily::Unix,
        SockType::Stream,
        None,
        SockFlag::SOCK_CLOEXEC,
    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
}

/// Parent side: send sockets to the newly spawned child.
pub fn handoff_to_child(channel_fd: RawFd, sockets: &[&PassableSocket]) -> io::Result<()> {
    let fds: Vec<RawFd> = sockets.iter().map(|s| s.fd).collect();
    send_fds(channel_fd, &fds)
}

/// Child side: receive sockets passed from the parent.
pub fn receive_from_parent(channel_fd: RawFd, count: usize) -> io::Result<Vec<RawFd>> {
    recv_fds(channel_fd, count)
}

/// Build the environment entries to inform the child of the handoff socket fd.
pub fn build_child_env(channel_fd: RawFd) -> Vec<(String, String)> {
    vec![
        (BATON_SOCK_ENV.to_string(), channel_fd.to_string()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_child_env() {
        let env = build_child_env(7);
        assert_eq!(env.len(), 1);
        assert_eq!(env[0], ("BATON_SOCK".to_string(), "7".to_string()));
    }

    #[test]
    fn test_create_handoff_channel_returns_two_fds() {
        let (a, b) = create_handoff_channel().expect("socketpair failed");
        assert!(a >= 0);
        assert!(b >= 0);
        unsafe {
            libc::close(a);
            libc::close(b);
        }
    }
}
