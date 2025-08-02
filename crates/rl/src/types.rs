//! Common data types used across RL components.

/// Observation provided to policies and value estimators.
#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    /// Flat feature vector representation.
    pub features: Vec<f32>,
}

impl Observation {
    /// Creates a new observation from a vector of features.
    pub fn new(features: Vec<f32>) -> Self {
        Self { features }
    }

    /// Returns the underlying feature slice.
    pub fn as_slice(&self) -> &[f32] {
        &self.features
    }
}

/// Discrete action represented as an index.
pub type Action = i64;

/// A batch of training data.
#[derive(Debug, Clone, Default)]
pub struct TrainingBatch {
    /// Batch observations.
    pub observations: Vec<Observation>,
    /// Actions taken.
    pub actions: Vec<Action>,
    /// Rewards observed.
    pub rewards: Vec<f32>,
    /// Next state observations.
    pub next_observations: Vec<Observation>,
    /// Episode termination flags.
    pub dones: Vec<bool>,
}
