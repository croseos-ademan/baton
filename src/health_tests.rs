#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::health::{wait_healthy, HealthCheck};

    #[test]
    fn delay_check_succeeds() {
        let result = wait_healthy(&HealthCheck::Delay(Duration::from_millis(10)), Duration::from_secs(1));
        assert!(result.is_ok());
    }

    #[test]
    fn tcp_check_times_out_on_closed_port() {
        // Port 1 is almost certainly closed.
        let result = wait_healthy(
            &HealthCheck::TcpPort(1),
            Duration::from_millis(250),
        );
        assert!(result.is_err());
        let msg = format!("{:?}", result.unwrap_err());
        assert!(msg.contains("timed out") || msg.contains("Timeout"));
    }

    #[test]
    fn tcp_check_succeeds_on_open_port() {
        use std::net::TcpListener;
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        // Accept in background so the connect can complete.
        std::thread::spawn(move || { let _ = listener.accept(); });
        let result = wait_healthy(&HealthCheck::TcpPort(port), Duration::from_secs(2));
        assert!(result.is_ok());
    }

    #[test]
    fn ready_fd_check_is_noop() {
        let result = wait_healthy(&HealthCheck::ReadyFd, Duration::from_millis(10));
        assert!(result.is_ok());
    }
}
