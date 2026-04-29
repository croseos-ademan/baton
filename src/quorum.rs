use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use crate::quorum_config::QuorumConfig;
use crate::quorum_error::QuorumError;

/// Tracks votes/acknowledgements from multiple participants to reach a quorum
/// before allowing a handoff or coordinated action to proceed.
#[derive(Debug)]
pub struct Quorum {
    config: QuorumConfig,
    votes: Arc<Mutex<HashSet<String>>>,
}

impl Quorum {
    pub fn new(config: QuorumConfig) -> Self {
        Self {
            config,
            votes: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Record a vote from a named participant. Returns true if quorum is now reached.
    pub fn vote(&self, participant: impl Into<String>) -> Result<bool, QuorumError> {
        let mut votes = self.votes.lock().map_err(|_| QuorumError::LockPoisoned)?;
        votes.insert(participant.into());
        Ok(votes.len() >= self.config.threshold)
    }

    /// Check whether quorum has been reached without adding a vote.
    pub fn is_reached(&self) -> Result<bool, QuorumError> {
        let votes = self.votes.lock().map_err(|_| QuorumError::LockPoisoned)?;
        Ok(votes.len() >= self.config.threshold)
    }

    /// Return current vote count.
    pub fn vote_count(&self) -> Result<usize, QuorumError> {
        let votes = self.votes.lock().map_err(|_| QuorumError::LockPoisoned)?;
        Ok(votes.len())
    }

    /// Reset all votes, allowing a new round.
    pub fn reset(&self) -> Result<(), QuorumError> {
        let mut votes = self.votes.lock().map_err(|_| QuorumError::LockPoisoned)?;
        votes.clear();
        Ok(())
    }

    /// Return a snapshot of current voters.
    pub fn voters(&self) -> Result<Vec<String>, QuorumError> {
        let votes = self.votes.lock().map_err(|_| QuorumError::LockPoisoned)?;
        let mut list: Vec<String> = votes.iter().cloned().collect();
        list.sort();
        Ok(list)
    }
}
