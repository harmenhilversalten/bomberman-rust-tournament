//! Simplified Torch-like value estimator.

use std::path::Path;

use ndarray::{Array1, Array2};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::error::RLError;
use crate::types::{Observation, TrainingBatch};

use super::ValueEstimator;

#[derive(Serialize, Deserialize)]
struct LinearModel {
    weights: Array2<f32>,
    bias: Array1<f32>,
}

/// Simple linear value function.
pub struct TorchValueEstimator {
    model: LinearModel,
}

impl TorchValueEstimator {
    /// Creates a new estimator for observations of `input_dim`.
    pub fn new(input_dim: usize) -> Self {
        let mut rng = rand::rng();
        let weights = Array2::from_shape_fn((input_dim, 1), |_| rng.random());
        let bias = Array1::from_shape_fn(1, |_| rng.random());
        Self {
            model: LinearModel { weights, bias },
        }
    }

    fn forward(&self, obs: &Observation) -> f32 {
        let x = Array1::from(obs.features.clone());
        (x.dot(&self.model.weights) + &self.model.bias)[0]
    }
}

impl ValueEstimator for TorchValueEstimator {
    fn get_value(&self, observation: &Observation) -> Result<f32, RLError> {
        Ok(self.forward(observation))
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_load_roundtrip() {
        let estimator = TorchValueEstimator::new(2);
        let obs = Observation::new(vec![0.1, -0.3]);
        let value = estimator.get_value(&obs).unwrap();

        let path = std::env::temp_dir().join("torch_value_estimator.bin");
        estimator.save(&path).unwrap();

        let mut loaded = TorchValueEstimator::new(2);
        loaded.load(&path).unwrap();
        let new_value = loaded.get_value(&obs).unwrap();

        assert_eq!(value, new_value);
        let _ = std::fs::remove_file(path);
    }
}
