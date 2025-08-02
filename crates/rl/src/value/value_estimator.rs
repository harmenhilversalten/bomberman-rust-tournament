//! Trait for value estimators.

use std::path::Path;

use crate::error::RLError;
use crate::types::{Observation, TrainingBatch};

/// Estimate state or state-action values.
pub trait ValueEstimator: Send {
    /// Returns the value estimate for an observation.
    fn get_value(&self, observation: &Observation) -> Result<f32, RLError>;
    /// Updates the estimator with a batch of data.
    fn update(&mut self, batch: &TrainingBatch) -> Result<(), RLError>;
    /// Saves the estimator to disk.
    fn save(&self, path: &Path) -> Result<(), RLError>;
    /// Loads the estimator from disk.
    fn load(&mut self, path: &Path) -> Result<(), RLError>;
}
