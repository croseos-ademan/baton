use std::fmt;

#[derive(Debug, PartialEq)]
pub enum QuorumError {
    /// The internal mutex was poisoned.
    LockPoisoned,
    /// A duplicate vote was cast (when strict mode is enabled).
    DuplicateVote(String),
    /// Quorum was not reached within the allowed window.
    Timeout,
}

impl fmt::Display for QuorumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuorumError::LockPoisoned => write!(f, "quorum lock poisoned"),
            QuorumError::DuplicateVote(p) => write!(f, "duplicate vote from participant: {}", p),
            QuorumError::Timeout => write!(f, "quorum not reached before timeout"),
        }
    }
}

impl std::error::Error for QuorumError {}
