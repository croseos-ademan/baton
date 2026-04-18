#[cfg(test)]
mod tests {
    use std::io;
    use crate::error::{BatonError, Result};

    #[test]
    fn display_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err = BatonError::from(io_err);
        assert!(err.to_string().contains("I/O error"));
        assert!(err.to_string().contains("file not found"));
    }

    #[test]
    fn display_pid_file_error() {
        let err = BatonError::PidFile("lock failed".into());
        assert_eq!(err.to_string(), "PID file error: lock failed");
    }

    #[test]
    fn display_handoff_error() {
        let err = BatonError::Handoff("no successor".into());
        assert_eq!(err.to_string(), "Handoff error: no successor");
    }

    #[test]
    fn display_ready_timeout() {
        let err = BatonError::ReadyTimeout;
        assert_eq!(err.to_string(), "Timed out waiting for process readiness");
    }

    #[test]
    fn from_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "denied");
        let err: BatonError = io_err.into();
        matches!(err, BatonError::Io(_));
    }

    #[test]
    fn result_alias_ok() {
        let r: Result<u32> = Ok(42);
        assert_eq!(r.unwrap(), 42);
    }

    #[test]
    fn result_alias_err() {
        let r: Result<u32> = Err(BatonError::Config("bad value".into()));
        assert!(r.is_err());
        assert!(r.unwrap_err().to_string().contains("bad value"));
    }

    #[test]
    fn source_present_for_io() {
        use std::error::Error;
        let io_err = io::Error::new(io::ErrorKind::Other, "oops");
        let err = BatonError::Io(io_err);
        assert!(err.source().is_some());
    }

    #[test]
    fn source_absent_for_others() {
        use std::error::Error;
        let err = BatonError::Signal("SIGTERM lost".into());
        assert!(err.source().is_none());
    }
}
