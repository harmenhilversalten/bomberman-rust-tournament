//! Simplified Torch-like policy using linear algebra.

use std::path::Path;

use ndarray::{Array1, Array2};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::error::RLError;
use crate::types::{Action, Observation, TrainingBatch};

use super::{Policy, PolicyType};

#[derive(Serialize, Deserialize)]
struct LinearModel {
    weights: Array2<f32>,
    bias: Array1<f32>,
}

/// Policy backed by a single linear layer.
pub struct TorchPolicy {
    model: LinearModel,
}

impl TorchPolicy {
    /// Creates a new linear policy.
    pub fn new(input_dim: usize, num_actions: usize) -> Self {
        let mut rng = rand::rng();
        let weights = Array2::from_shape_fn((input_dim, num_actions), |_| rng.random());
        let bias = Array1::from_shape_fn(num_actions, |_| rng.random());
        Self {
            model: LinearModel { weights, bias },
        }
    }

    fn forward(&self, obs: &Observation) -> Array1<f32> {
        let x = Array1::from(obs.features.clone());
        x.dot(&self.model.weights) + &self.model.bias
    }
}

impl Policy for TorchPolicy {
    fn get_policy_type(&self) -> PolicyType {
        PolicyType::Torch
    }

    fn select_action(&mut self, observation: &Observation) -> Result<Action, RLError> {
        let logits = self.forward(observation);
        let (idx, _) = logits
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .ok_or_else(|| RLError::Model("empty logits".into()))?;
        Ok(idx as Action)
    }

    fn update(&mut self, _batch: &TrainingBatch) -> Result<(), RLError> {
        Ok(())
    }

    fn save(&self, path: &Path) -> Result<(), RLError> {
        let bytes = bincode::serialize(&self.model).map_err(|e| RLError::Model(e.to_string()))?;
        std::fs::write(path, bytes)?;
        Ok(())
    }

    fn load(&mut self, path: &Path) -> Result<(), RLError> {
        let bytes = std::fs::read(path)?;
        self.model = bincode::deserialize(&bytes).map_err(|e| RLError::Model(e.to_string()))?;
        Ok(())
    }

    fn get_memory_usage(&self) -> usize {
        self.model.weights.len() * std::mem::size_of::<f32>()
            + self.model.bias.len() * std::mem::size_of::<f32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_load_roundtrip() {
        let mut policy = TorchPolicy::new(2, 2);
        let obs = Observation::new(vec![1.0, -1.0]);
        let action = policy.select_action(&obs).unwrap();

        let path = std::env::temp_dir().join("torch_policy_save_load.bin");
        policy.save(&path).unwrap();

        let mut loaded = TorchPolicy::new(2, 2);
        loaded.load(&path).unwrap();
        let new_action = loaded.select_action(&obs).unwrap();

        assert_eq!(action, new_action);
        let _ = std::fs::remove_file(path);
    }
}
