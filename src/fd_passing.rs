//! File descriptor passing over Unix domain sockets using SCM_RIGHTS.

use nix::sys::socket::{recvmsg, sendmsg, ControlMessage, ControlMessageOwned, MsgFlags};
use nix::sys::uio::IoVec;
use std::io;
use std::os::unix::io::RawFd;

const DUMMY_BYTE: &[u8] = b"\x00";

/// Send file descriptors over a Unix socket.
pub fn send_fds(sock_fd: RawFd, fds: &[RawFd]) -> io::Result<()> {
    let iov = [IoVec::from_slice(DUMMY_BYTE)];
    let cmsg = [ControlMessage::ScmRights(fds)];
    sendmsg(sock_fd, &iov, &cmsg, MsgFlags::empty(), None)
        .map(|_| ())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
}

/// Receive file descriptors from a Unix socket.
/// Returns the received fds on success.
pub fn recv_fds(sock_fd: RawFd, max_fds: usize) -> io::Result<Vec<RawFd>> {
    let mut buf = [0u8; 1];
    let iov = [IoVec::from_mut_slice(&mut buf)];
    let mut cmsg_buf = nix::cmsg_space!([RawFd; 8]);

    let msg = recvmsg(sock_fd, &iov, Some(&mut cmsg_buf), MsgFlags::empty())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let mut fds = Vec::new();
    for cmsg in msg.cmsgs() {
        if let ControlMessageOwned::ScmRights(received) = cmsg {
            fds.extend_from_slice(&received[..received.len().min(max_fds)]);
        }
    }
    Ok(fds)
}
