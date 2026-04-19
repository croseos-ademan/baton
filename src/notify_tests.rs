#[cfg(test)]
mod tests {
    use crate::notify::Notifier;
    use crate::notify_config::NotifyConfig;
    use crate::notify_handler::NotifyHandler;
    use std::os::unix::net::UnixDatagram;
    use tempfile::tempdir;

    fn make_server(path: &str) -> UnixDatagram {
        let sock = UnixDatagram::bind(path).expect("bind");
        sock.set_nonblocking(true).ok();
        sock
    }

    #[test]
    fn test_notifier_not_available_without_env() {
        // Ensure NOTIFY_SOCKET isn't set in this test
        std::env::remove_var("NOTIFY_SOCKET");
        let n = Notifier::new();
        assert!(!n.is_available());
    }

    #[test]
    fn test_notifier_send_ready() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("notify.sock");
        let path_str = path.to_str().unwrap();
        let server = make_server(path_str);

        let n = Notifier::from_path(path_str);
        assert!(n.is_available());
        n.ready().expect("ready");

        let mut buf = [0u8; 64];
        let len = server.recv(&mut buf).expect("recv");
        assert_eq!(&buf[..len], b"READY=1\n");
    }

    #[test]
    fn test_handler_skips_when_disabled() {
        let config = NotifyConfig::disabled();
        let handler = NotifyHandler::new(config);
        // Should not error even without a socket
        assert!(handler.on_ready().is_ok());
        assert!(handler.on_stopping().is_ok());
    }

    #[test]
    fn test_notify_config_defaults() {
        let c = NotifyConfig::new();
        assert!(c.enabled);
        assert!(c.notify_ready);
        assert!(c.notify_stopping);
        assert!(c.socket_path.is_none());
    }

    #[test]
    fn test_notify_config_builder() {
        let c = NotifyConfig::new()
            .with_socket_path("/tmp/test.sock")
            .with_notify_ready(false);
        assert_eq!(c.socket_path.as_deref(), Some("/tmp/test.sock"));
        assert!(!c.notify_ready);
    }
}
