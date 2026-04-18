use std::env;
use std::fs;
use std::io::Write;
use std::os::unix::net::UnixStream;

/// Notify readiness via sd_notify protocol or a custom socket path.
pub enum ReadyNotifier {
    SdNotify(String),
    File(String),
    None,
}

impl ReadyNotifier {
    pub fn from_env() -> Self {
        if let Ok(path) = env::var("NOTIFY_SOCKET") {
            return ReadyNotifier::SdNotify(path);
        }
        if let Ok(path) = env::var("BATON_READY_FILE") {
            return ReadyNotifier::File(path);
        }
        ReadyNotifier::None
    }

    pub fn notify_ready(&self) -> std::io::Result<()> {
        match self {
            ReadyNotifier::SdNotify(path) => {
                let mut stream = UnixStream::connect(path)?;
                stream.write_all(b"READY=1\n")?;
                Ok(())
            }
            ReadyNotifier::File(path) => {
                fs::write(path, b"ready\n")?;
                Ok(())
            }
            ReadyNotifier::None => Ok(()),
        }
    }

    pub fn notify_stopping(&self) -> std::io::Result<()> {
        match self {
            ReadyNotifier::SdNotify(path) => {
                let mut stream = UnixStream::connect(path)?;
                stream.write_all(b"STOPPING=1\n")?;
                Ok(())
            }
            ReadyNotifier::File(path) => {
                let _ = fs::remove_file(path);
                Ok(())
            }
            ReadyNotifier::None => Ok(()),
        }
    }
}
