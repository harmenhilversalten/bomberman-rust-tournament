//! Policy that selects actions uniformly at random.

use std::path::Path;

use rand::Rng;

use crate::error::RLError;
use crate::types::{Action, Observation, TrainingBatch};

use super::{Policy, PolicyType};

/// Simple random policy useful for testing.
#[derive(Debug)]
pub struct RandomPolicy {
    num_actions: Action,
}

impl RandomPolicy {
    /// Creates a new random policy over `num_actions` actions.
    pub fn new(num_actions: Action) -> Self {
        Self { num_actions }
    }
}

impl Policy for RandomPolicy {
    fn get_policy_type(&self) -> PolicyType {
        PolicyType::Random
    }

    fn select_action(&mut self, _observation: &Observation) -> Result<Action, RLError> {
        let mut rng = rand::rng();
        Ok(rng.random_range(0..self.num_actions))
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
        std::mem::size_of::<Self>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_within_range() {
        let mut policy = RandomPolicy::new(5);
        let obs = Observation::new(vec![0.0]);
        let action = policy.select_action(&obs).unwrap();
        assert!((0..5).contains(&action));
    }
}
