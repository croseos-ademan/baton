use crate::error::BatonError;
use crate::notify::Notifier;
use crate::notify_config::NotifyConfig;

/// Drives notify messages based on config and lifecycle events.
pub struct NotifyHandler {
    notifier: Notifier,
    config: NotifyConfig,
}

impl NotifyHandler {
    pub fn new(config: NotifyConfig) -> Self {
        let notifier = match &config.socket_path {
            Some(path) => Notifier::from_path(path),
            None => Notifier::new(),
        };
        NotifyHandler { notifier, config }
    }

    pub fn on_ready(&self) -> Result<(), BatonError> {
        if self.config.enabled && self.config.notify_ready {
            if self.notifier.is_available() {
                return self.notifier.ready();
            }
        }
        Ok(())
    }

    pub fn on_stopping(&self) -> Result<(), BatonError> {
        if self.config.enabled && self.config.notify_stopping {
            if self.notifier.is_available() {
                return self.notifier.stopping();
            }
        }
        Ok(())
    }

    pub fn on_reloading(&self) -> Result<(), BatonError> {
        if self.config.enabled && self.notifier.is_available() {
            return self.notifier.reloading();
        }
        Ok(())
    }

    pub fn on_status(&self, msg: &str) -> Result<(), BatonError> {
        if self.config.enabled && self.notifier.is_available() {
            return self.notifier.status(msg);
        }
        Ok(())
    }
}
