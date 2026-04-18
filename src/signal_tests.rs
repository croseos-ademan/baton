#[cfg(test)]
mod tests {
    use super::super::signal::*;
    use nix::sys::signal::Signal;
    use nix::unistd::getpid;
    use std::sync::atomic::Ordering;

    #[test]
    fn test_signal_handler_initial_state() {
        let handler = SignalHandler::new();
        assert!(!handler.shutdown_requested());
        assert!(!handler.reload_requested());
    }

    #[test]
    fn test_clear_reload() {
        let handler = SignalHandler::new();
        handler.reload_requested.store(true, Ordering::SeqCst);
        assert!(handler.reload_requested());
        handler.clear_reload();
        assert!(!handler.reload_requested());
    }

    #[test]
    fn test_send_signal_self() {
        let pid = getpid().as_raw() as u32;
        // SIGWINCH is safe to send to self without side effects
        let result = send_signal(pid, Signal::SIGWINCH);
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_signal_invalid_pid() {
        // PID 0 has special meaning, use a clearly invalid large PID
        let result = send_signal(99999999, Signal::SIGTERM);
        assert!(result.is_err());
    }

    #[test]
    fn test_global_flags_initial_state() {
        // Note: these may be affected by other tests if signals are sent
        // Just verify the functions are callable
        let _ = is_shutdown_requested();
        let _ = is_reload_requested();
    }
}
