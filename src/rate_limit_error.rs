use std::fmt;
use std::time::Duration;

/// Errors specific to rate limiting operations.
#[derive(Debug, Clone, PartialEq)]
pub enum RateLimitError {
    /// Request was denied due to rate limiting.
    Exceeded { retry_after: Duration },
    /// Configuration was invalid.
    InvalidConfig(String),
    /// Internal lock was poisoned.
    LockPoisoned,
}

impl fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RateLimitError::Exceeded { retry_after } => {
                write!(
                    f,
                    "rate limit exceeded; retry after {:.2}s",
                    retry_after.as_secs_f64()
                )
            }
            RateLimitError::InvalidConfig(msg) => {
                write!(f, "invalid rate limit config: {}", msg)
            }
            RateLimitError::LockPoisoned => {
                write!(f, "rate limiter internal lock was poisoned")
            }
        }
    }
}

impl std::error::Error for RateLimitError {}

impl From<RateLimitError> for String {
    fn from(e: RateLimitError) -> Self {
        e.to_string()
    }
}
