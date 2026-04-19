/// Extension to BatonError for notify-specific variants.
/// This file documents the expected error variant added to error.rs.
///
/// In error.rs, add:
///   NotifyError(String),
/// with display:
///   BatonError::NotifyError(msg) => write!(f, "notify error: {}", msg),

/// Standalone helper for logging notify failures without propagating.
pub fn log_notify_error(context: &str, err: &crate::error::BatonError) {
    eprintln!("[baton] notify warning ({}): {}", context, err);
}

/// Returns true if the error is a notify error that can be safely ignored.
pub fn is_ignorable_notify_error(err: &crate::error::BatonError) -> bool {
    matches!(err, crate::error::BatonError::NotifyError(_))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::BatonError;

    #[test]
    fn test_is_ignorable() {
        let e = BatonError::NotifyError("no socket".to_string());
        assert!(is_ignorable_notify_error(&e));
    }

    #[test]
    fn test_not_ignorable_other_error() {
        let e = BatonError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "some io error",
        ));
        assert!(!is_ignorable_notify_error(&e));
    }
}
