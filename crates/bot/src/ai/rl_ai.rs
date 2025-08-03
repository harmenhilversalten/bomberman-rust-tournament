use std::sync::{Arc, Mutex};

use crate::bot::decision::DecisionMaker;
use events::events::BotDecision;
use rl::{Policy, Value};
use state::grid::GridDelta;

/// Reinforcement learning based AI implementation.
#[allow(missing_docs)]
pub struct RLAI {
    pub policy: Arc<Mutex<dyn Policy>>,
    pub value_network: Option<Arc<dyn Value>>,
    pub exploration_rate: f32,
}

impl RLAI {
    /// Create a new [`RLAI`] instance.
    pub fn new(
        policy: Arc<Mutex<dyn Policy>>,
        value_network: Option<Arc<dyn Value>>,
        exploration_rate: f32,
    ) -> Self {
        Self {
            policy,
            value_network,
            exploration_rate,
        }
    }

    /// Convert a [`GridDelta`] into a flat observation vector.
    fn generate_observation(&self, snapshot: &GridDelta) -> Vec<f32> {
        match snapshot {
            GridDelta::None => vec![0.0],
            GridDelta::SetTile { .. } => vec![1.0],
            GridDelta::AddBomb(_) => vec![2.0],
            GridDelta::AddAgent(_) => vec![3.0],
        }
    }
}

impl DecisionMaker<GridDelta, BotDecision> for RLAI {
    fn decide(&mut self, snapshot: GridDelta) -> BotDecision {
        let obs = self.generate_observation(&snapshot);
        let mut policy = self.policy.lock().unwrap();
        let action = policy.select_action(&obs).unwrap_or(0);
        match action {
            1 => BotDecision::PlaceBomb,
            _ => BotDecision::Wait,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rl::{
        Policy, PolicyType,
        error::RLError,
        types::{Observation, TrainingBatch},
    };

    struct StubPolicy;
    impl Policy for StubPolicy {
        fn get_policy_type(&self) -> PolicyType {
            PolicyType::Random
        }
        fn select_action(&mut self, _observation: &Observation) -> Result<i64, RLError> {
            Ok(1)
        }
        fn update(&mut self, _batch: &TrainingBatch) -> Result<(), RLError> {
            Ok(())
        }
        fn save(&self, _path: &std::path::Path) -> Result<(), RLError> {
            Ok(())
        }
        fn load(&mut self, _path: &std::path::Path) -> Result<(), RLError> {
            Ok(())
        }
        fn get_memory_usage(&self) -> usize {
            0
        }
    }

    impl RLAI {
        #[allow(missing_docs)]
        pub fn test_new() -> Self {
            let policy = Arc::new(Mutex::new(StubPolicy)) as Arc<Mutex<dyn Policy>>;
            Self::new(policy, None, 0.0)
        }
    }

    #[test]
    fn rl_ai_decides_place_bomb() {
        let mut ai = RLAI::test_new();
        let decision = ai.decide(GridDelta::None);
        assert_eq!(decision, BotDecision::PlaceBomb);
    }
}
