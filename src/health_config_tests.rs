#[cfg(test)]
mod tests {
    use crate::health_config::HealthCheckSpec;
    use crate::health::HealthCheck;
    use std::time::Duration;

    #[test]
    fn resolve_ready_fd() {
        let spec = HealthCheckSpec::new("ready_fd");
        let (check, timeout) = spec.resolve().unwrap();
        assert!(matches!(check, HealthCheck::ReadyFd));
        assert_eq!(timeout, Duration::from_secs(30));
    }

    #[test]
    fn resolve_tcp() {
        let spec = HealthCheckSpec::new("tcp").with_value("8080").with_timeout(10);
        let (check, timeout) = spec.resolve().unwrap();
        assert!(matches!(check, HealthCheck::TcpPort(8080)));
        assert_eq!(timeout, Duration::from_secs(10));
    }

    #[test]
    fn resolve_delay() {
        let spec = HealthCheckSpec::new("delay").with_value("500");
        let (check, _) = spec.resolve().unwrap();
        assert!(matches!(check, HealthCheck::Delay(d) if d == Duration::from_millis(500)));
    }

    #[test]
    fn resolve_invalid_kind() {
        let spec = HealthCheckSpec::new("magic");
        assert!(spec.resolve().is_err());
    }

    #[test]
    fn resolve_tcp_bad_port() {
        let spec = HealthCheckSpec::new("tcp").with_value("notaport");
        assert!(spec.resolve().is_err());
    }

    #[test]
    fn resolve_delay_bad_value() {
        let spec = HealthCheckSpec::new("delay").with_value("abc");
        assert!(spec.resolve().is_err());
    }
}
