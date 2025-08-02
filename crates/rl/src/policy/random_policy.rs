//! Random policy useful for testing and baselines.
use std::path::Path;

use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::{
    error::RLError,
    types::{Action, Observation, TrainingBatch},
};

use super::{Policy, PolicyType};

/// Policy that selects actions uniformly at random.
pub struct RandomPolicy {
    rng: StdRng,
    actions: i64,
}

impl RandomPolicy {
    /// Create a new [`RandomPolicy`].
    pub fn new(actions: i64, seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_entropy(),
        };
        Self { rng, actions }
    }
}

impl Policy for RandomPolicy {
    fn get_policy_type(&self) -> PolicyType {
        PolicyType::Random
    }

    fn select_action(&mut self, _observation: &Observation) -> Result<Action, RLError> {
        Ok(self.rng.gen_range(0..self.actions))
    }

    fn update(&mut self, _batch: &TrainingBatch) -> Result<(), RLError> {
        Ok(())
    }

    fn save(&self, _path: &Path) -> Result<(), RLError> {
        Ok(())
    }

    fn load(&mut self, _path: &Path) -> Result<(), RLError> {
        Ok(())
    }

    fn get_memory_usage(&self) -> usize {
        0
    }
}
