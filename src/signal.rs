use nix::sys::signal::{self, Signal, SigHandler};
use nix::unistd::Pid;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct SignalHandler {
    pub shutdown_requested: Arc<AtomicBool>,
    pub reload_requested: Arc<AtomicBool>,
}

impl SignalHandler {
    pub fn new() -> Self {
        SignalHandler {
            shutdown_requested: Arc::new(AtomicBool::new(false)),
            reload_requested: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn setup(&self) -> nix::Result<()> {
        unsafe {
            signal::signal(Signal::SIGTERM, SigHandler::Handler(handle_sigterm))?;
            signal::signal(Signal::SIGHUP, SigHandler::Handler(handle_sighup))?;
            signal::signal(Signal::SIGINT, SigHandler::Handler(handle_sigterm))?;
        }
        Ok(())
    }

    pub fn shutdown_requested(&self) -> bool {
        self.shutdown_requested.load(Ordering::SeqCst)
    }

    pub fn reload_requested(&self) -> bool {
        self.reload_requested.load(Ordering::SeqCst)
    }

    pub fn clear_reload(&self) {
        self.reload_requested.store(false, Ordering::SeqCst);
    }
}

static SHUTDOWN_FLAG: AtomicBool = AtomicBool::new(false);
static RELOAD_FLAG: AtomicBool = AtomicBool::new(false);

extern "C" fn handle_sigterm(_: libc::c_int) {
    SHUTDOWN_FLAG.store(true, Ordering::SeqCst);
}

extern "C" fn handle_sighup(_: libc::c_int) {
    RELOAD_FLAG.store(true, Ordering::SeqCst);
}

pub fn send_signal(pid: u32, signal: Signal) -> nix::Result<()> {
    signal::kill(Pid::from_raw(pid as i32), signal)
}

pub fn is_shutdown_requested() -> bool {
    SHUTDOWN_FLAG.load(Ordering::SeqCst)
}

pub fn is_reload_requested() -> bool {
    RELOAD_FLAG.load(Ordering::SeqCst)
}
