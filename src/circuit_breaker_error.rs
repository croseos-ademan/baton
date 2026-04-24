use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CircuitBreakerError {
    CircuitOpen,
    CallFailed(String),
    InvalidConfig(String),
}

impl fmt::Display for CircuitBreakerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CircuitBreakerError::CircuitOpen => {
                write!(f, "circuit breaker is open: calls are being rejected")
            }
            CircuitBreakerError::CallFailed(msg) => {
                write!(f, "circuit breaker call failed: {}", msg)
            }
            CircuitBreakerError::InvalidConfig(msg) => {
                write!(f, "invalid circuit breaker config: {}", msg)
            }
        }
    }
}

impl std::error::Error for CircuitBreakerError {}

impl From<String> for CircuitBreakerError {
    fn from(s: String) -> Self {
        CircuitBreakerError::CallFailed(s)
    }
}
