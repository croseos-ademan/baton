#[cfg(test)]
mod tests {
    use super::super::backoff::Backoff;
    use std::time::Duration;

    #[test]
    fn test_initial_duration() {
        let mut b = Backoff::new(
            Duration::from_millis(100),
            Duration::from_secs(10),
            2.0,
        );
        assert_eq!(b.next(), Duration::from_millis(100));
    }

    #[test]
    fn test_exponential_growth() {
        let mut b = Backoff::new(
            Duration::from_millis(100),
            Duration::from_secs(10),
            2.0,
        );
        assert_eq!(b.next(), Duration::from_millis(100));
        assert_eq!(b.next(), Duration::from_millis(200));
        assert_eq!(b.next(), Duration::from_millis(400));
    }

    #[test]
    fn test_max_cap() {
        let mut b = Backoff::new(
            Duration::from_millis(500),
            Duration::from_secs(1),
            4.0,
        );
        b.next();
        let second = b.next();
        assert!(second <= Duration::from_secs(1));
    }

    #[test]
    fn test_reset() {
        let mut b = Backoff::default_config();
        b.next();
        b.next();
        assert_eq!(b.attempts(), 2);
        b.reset();
        assert_eq!(b.attempts(), 0);
        assert_eq!(b.next(), Duration::from_millis(100));
    }

    #[test]
    fn test_is_maxed() {
        let mut b = Backoff::new(
            Duration::from_millis(500),
            Duration::from_secs(1),
            3.0,
        );
        assert!(!b.is_maxed());
        b.next();
        b.next();
        assert!(b.is_maxed());
    }
}
