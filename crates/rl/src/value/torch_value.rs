//! Torch-based value estimator implementation.
use std::{path::Path, sync::Mutex};

use tch::{Device, Tensor, nn, nn::Module};

use crate::{
    error::RLError,
    types::{Observation, TrainingBatch},
};

use super::ValueEstimator;

/// Simple linear neural network value estimator.
pub struct TorchValueEstimator {
    vs: nn::VarStore,
    net: Mutex<nn::Sequential>,
}

impl TorchValueEstimator {
    /// Create a new estimator with the given input dimension.
    pub fn new(input_dim: i64) -> Self {
        let vs = nn::VarStore::new(Device::Cpu);
        let net = nn::seq().add(nn::linear(
            &vs.root() / "layer1",
            input_dim,
            1,
            Default::default(),
        ));
        Self {
            vs,
            net: Mutex::new(net),
        }
    }
}

impl ValueEstimator for TorchValueEstimator {
    fn get_value(&self, observation: &Observation) -> Result<f32, RLError> {
        let input = Tensor::of_slice(&observation[..]).unsqueeze(0);
        Ok(f32::from(
            self.net.lock().unwrap().forward(&input).squeeze(),
        ))
    }

    fn update(&mut self, _batch: &TrainingBatch) -> Result<(), RLError> {
        Ok(())
    }

    fn save(&self, path: &Path) -> Result<(), RLError> {
        self.vs.save(path).map_err(RLError::from)
    }

    fn load(&mut self, path: &Path) -> Result<(), RLError> {
        self.vs.load(path).map_err(RLError::from)
    }
}
