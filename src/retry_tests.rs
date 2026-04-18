#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use crate::backoff::Backoff;
    use crate::retry::{Retry, RetryConfig};

    fn fast_config(max: u32) -> RetryConfig {
        RetryConfig::new(
            max,
            Backoff::new(Duration::from_millis(1), Duration::from_millis(10), 1.0),
        )
    }

    #[test]
    fn test_success_on_first_attempt() {
        let mut retry = Retry::new(fast_config(3));
        let result: Result<i32, &str> = retry.attempt(|_| Ok(42));
        assert_eq!(result.unwrap(), 42);
        assert_eq!(retry.attempts(), 1);
    }

    #[test]
    fn test_retries_on_failure() {
        let counter = Arc::new(AtomicU32::new(0));
        let c = counter.clone();
        let mut retry = Retry::new(fast_config(3));
        let result: Result<i32, &str> = retry.attempt(|_| {
            let n = c.fetch_add(1, Ordering::SeqCst);
            if n < 2 { Err("fail") } else { Ok(99) }
        });
        assert_eq!(result.unwrap(), 99);
        assert_eq!(retry.attempts(), 3);
    }

    #[test]
    fn test_exhausts_attempts() {
        let mut retry = Retry::new(fast_config(2));
        let result: Result<i32, &str> = retry.attempt(|_| Err("always fails"));
        assert!(result.is_err());
        assert!(retry.exhausted());
        assert_eq!(retry.attempts(), 2);
    }

    #[test]
    fn test_default_config() {
        let cfg = RetryConfig::default();
        assert_eq!(cfg.max_attempts, 3);
    }
}
