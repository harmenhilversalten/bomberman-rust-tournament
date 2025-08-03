//! Policy backed by a Torch neural network.
use std::{path::Path, sync::Mutex};

use tch::{Device, Tensor, nn, nn::Module};

use crate::{
    error::RLError,
    types::{Action, Observation, TrainingBatch},
};

use super::{Policy, PolicyType};

/// Simple feed-forward neural network policy using `tch`.
pub struct TorchPolicy {
    vs: nn::VarStore,
    net: Mutex<nn::Sequential>,
}

impl TorchPolicy {
    /// Create a new policy with the given dimensions.
    pub fn new(input_dim: i64, output_dim: i64) -> Self {
        let vs = nn::VarStore::new(Device::Cpu);
        let net = nn::seq().add(nn::linear(
            &vs.root() / "layer1",
            input_dim,
            output_dim,
            Default::default(),
        ));
        Self {
            vs,
            net: Mutex::new(net),
        }
    }

    /// Load a policy from the specified file returning the initialized instance.
    pub fn load(path: &Path, input_dim: i64, output_dim: i64) -> Result<Self, RLError> {
        let mut policy = Self::new(input_dim, output_dim);
        Policy::load(&mut policy, path)?;
        Ok(policy)
    }
}

impl Policy for TorchPolicy {
    fn get_policy_type(&self) -> PolicyType {
        PolicyType::Torch
    }

    fn select_action(&mut self, observation: &Observation) -> Result<Action, RLError> {
        let input = Tensor::of_slice(&observation[..]).unsqueeze(0);
        let output = self.net.lock().unwrap().forward(&input);
        Ok(output.argmax(-1, false).int64_value(&[0]))
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

    fn get_memory_usage(&self) -> usize {
        self.vs.variables().values().map(|t| t.numel() * 4).sum()
    }
}
