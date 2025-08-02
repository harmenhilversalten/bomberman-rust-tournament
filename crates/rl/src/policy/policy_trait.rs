//! Core policy trait definition.

use std::path::Path;

use crate::error::RLError;
use crate::types::{Action, Observation, TrainingBatch};

/// Available policy implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyType {
    /// Policy backed by a Torch model.
    Torch,
    /// Policy choosing actions at random.
    Random,
}

/// Trait implemented by all action selection policies.
pub trait Policy: Send {
    /// Returns the type of policy.
    fn get_policy_type(&self) -> PolicyType;
    /// Selects an action for the given observation.
    fn select_action(&mut self, observation: &Observation) -> Result<Action, RLError>;
    /// Updates the policy with a batch of experience.
    fn update(&mut self, batch: &TrainingBatch) -> Result<(), RLError>;
    /// Saves the policy to a file.
    fn save(&self, path: &Path) -> Result<(), RLError>;
    /// Loads the policy from a file.
    fn load(&mut self, path: &Path) -> Result<(), RLError>;
    /// Returns an estimate of memory usage in bytes.
    fn get_memory_usage(&self) -> usize;
}
