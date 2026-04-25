use std::time::{Duration, Instant};
use crate::rollback_config::RollbackConfig;
use crate::error::BatonError;

#[derive(Debug, Clone, PartialEq)]
pub enum RollbackReason {
    HealthCheckFailed,
    StartupTimeout,
    ProcessCrashed,
    Manual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RollbackState {
    Idle,
    Triggered(RollbackReason),
    InProgress,
    Completed,
    Failed(String),
}

pub struct RollbackManager {
    config: RollbackConfig,
    state: RollbackState,
    triggered_at: Option<Instant>,
    attempt_count: u32,
}

impl RollbackManager {
    pub fn new(config: RollbackConfig) -> Self {
        RollbackManager {
            config,
            state: RollbackState::Idle,
            triggered_at: None,
            attempt_count: 0,
        }
    }

    pub fn trigger(&mut self, reason: RollbackReason) -> Result<(), BatonError> {
        if !self.config.enabled {
            return Err(BatonError::Config("Rollback is disabled".into()));
        }
        if self.attempt_count >= self.config.max_attempts {
            return Err(BatonError::Config(format!(
                "Max rollback attempts ({}) exceeded",
                self.config.max_attempts
            )));
        }
        self.state = RollbackState::Triggered(reason);
        self.triggered_at = Some(Instant::now());
        self.attempt_count += 1;
        Ok(())
    }

    pub fn begin(&mut self) -> Result<(), BatonError> {
        match &self.state {
            RollbackState::Triggered(_) => {
                self.state = RollbackState::InProgress;
                Ok(())
            }
            _ => Err(BatonError::Config("Rollback not triggered".into())),
        }
    }

    pub fn complete(&mut self) {
        self.state = RollbackState::Completed;
    }

    pub fn fail(&mut self, reason: impl Into<String>) {
        self.state = RollbackState::Failed(reason.into());
    }

    pub fn state(&self) -> &RollbackState {
        &self.state
    }

    pub fn attempt_count(&self) -> u32 {
        self.attempt_count
    }

    pub fn elapsed(&self) -> Option<Duration> {
        self.triggered_at.map(|t| t.elapsed())
    }

    pub fn is_within_window(&self) -> bool {
        match self.triggered_at {
            Some(t) => t.elapsed() <= self.config.window,
            None => false,
        }
    }
}
