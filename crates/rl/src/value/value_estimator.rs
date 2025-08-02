//! Trait for estimating state values.
use std::path::Path;

use crate::{
    error::RLError,
    types::{Observation, TrainingBatch},
};

/// Evaluates the value of observations.
pub trait ValueEstimator: Send + Sync {
    /// Return the scalar value for the given observation.
    fn get_value(&self, observation: &Observation) -> Result<f32, RLError>;
    /// Update the estimator using a batch of transitions.
    fn update(&mut self, batch: &TrainingBatch) -> Result<(), RLError>;
    /// Save the estimator to disk.
    fn save(&self, path: &Path) -> Result<(), RLError>;
    /// Load the estimator from disk.
    fn load(&mut self, path: &Path) -> Result<(), RLError>;
}
