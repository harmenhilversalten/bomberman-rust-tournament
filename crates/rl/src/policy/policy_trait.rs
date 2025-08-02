//! Policy trait and related definitions.
use std::path::Path;

use crate::{
    error::RLError,
    types::{Action, Observation, TrainingBatch},
};

/// Types of policies supported by the crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyType {
    /// Policy backed by a Torch model.
    Torch,
    /// Policy selecting actions at random.
    Random,
}

/// Behaviour mapping observations to actions.
pub trait Policy: Send + Sync {
    /// Returns the policy type.
    fn get_policy_type(&self) -> PolicyType;
    /// Select an action given an observation.
    fn select_action(&mut self, observation: &Observation) -> Result<Action, RLError>;
    /// Update the policy using a batch of training data.
    fn update(&mut self, batch: &TrainingBatch) -> Result<(), RLError>;
    /// Persist the policy to the specified path.
    fn save(&self, path: &Path) -> Result<(), RLError>;
    /// Load the policy from the specified path.
    fn load(&mut self, path: &Path) -> Result<(), RLError>;
    /// Estimated memory usage in bytes.
    fn get_memory_usage(&self) -> usize;
}
