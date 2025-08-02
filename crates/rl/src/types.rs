//! Common data types used across RL modules.

/// Observation vector fed into policies and value estimators.
pub type Observation = Vec<f32>;

/// Discrete action returned by a policy.
pub type Action = i64;

/// Batch of transitions for training.
#[derive(Default, Debug, Clone)]
pub struct TrainingBatch {
    /// Observations at time t.
    pub observations: Vec<Observation>,
    /// Actions taken at time t.
    pub actions: Vec<Action>,
    /// Rewards received after each action.
    pub rewards: Vec<f32>,
    /// Observations at time t+1.
    pub next_observations: Vec<Observation>,
    /// Episode termination flags.
    pub dones: Vec<bool>,
}
