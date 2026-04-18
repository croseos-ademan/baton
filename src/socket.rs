use std::os::unix::io::RawFd;
use nix::sys::socket::{getsockopt, sockopt::ReusePort};
use std::io;

/// Represents a socket that can be passed between processes.
#[derive(Debug)]
pub struct PassableSocket {
    pub fd: RawFd,
    pub port: u16,
    pub addr: String,
}

impl PassableSocket {
    pub fn new(fd: RawFd, port: u16, addr: &str) -> Self {
        Self {
            fd,
            port,
            addr: addr.to_string(),
        }
    }

    /// Verify the socket is still valid and has SO_REUSEPORT set.
    pub fn validate(&self) -> io::Result<bool> {
        match getsockopt(self.fd, ReusePort) {
            Ok(val) => Ok(val),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
        }
    }

    /// Return the file descriptor as a string for passing via env.
    pub fn fd_env_value(&self) -> String {
        self.fd.to_string()
    }
}

/// Collect all socket fds from the environment variable set by the parent.
pub fn sockets_from_env(env_key: &str) -> Vec<RawFd> {
    std::env::var(env_key)
        .unwrap_or_default()
        .split(',')
        .filter_map(|s| s.trim().parse::<RawFd>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passable_socket_fd_env_value() {
        let sock = PassableSocket::new(5, 8080, "127.0.0.1");
        assert_eq!(sock.fd_env_value(), "5");
    }

    #[test]
    fn test_sockets_from_env_empty() {
        std::env::remove_var("BATON_FDS");
        let fds = sockets_from_env("BATON_FDS");
        assert!(fds.is_empty());
    }

    #[test]
    fn test_sockets_from_env_multiple() {
        std::env::set_var("BATON_FDS", "3,4,5");
        let fds = sockets_from_env("BATON_FDS");
        assert_eq!(fds, vec![3, 4, 5]);
        std::env::remove_var("BATON_FDS");
    }
}
