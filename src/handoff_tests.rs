#[cfg(test)]
mod integration {
    use crate::fd_passing::{recv_fds, send_fds};
    use crate::handoff::create_handoff_channel;
    use nix::unistd::{close, fork, ForkResult};
    use std::os::unix::io::RawFd;

    fn open_dummy_fd() -> RawFd {
        let fd = unsafe { libc::memfd_create(b"test\0".as_ptr() as *const libc::c_char, 0) };
        assert!(fd >= 0, "memfd_create failed");
        fd
    }

    #[test]
    fn test_fd_roundtrip_via_socketpair() {
        let (parent_fd, child_fd) = create_handoff_channel().unwrap();
        let dummy = open_dummy_fd();

        match unsafe { fork() }.expect("fork failed") {
            ForkResult::Parent { child } => {
                close(child_fd).ok();
                send_fds(parent_fd, &[dummy]).expect("send_fds failed");
                close(parent_fd).ok();
                close(dummy).ok();
                nix::sys::wait::waitpid(child, None).ok();
            }
            ForkResult::Child => {
                close(parent_fd).ok();
                close(dummy).ok();
                let received = recv_fds(child_fd, 1).expect("recv_fds failed");
                assert_eq!(received.len(), 1);
                close(child_fd).ok();
                std::process::exit(0);
            }
        }
    }
}
