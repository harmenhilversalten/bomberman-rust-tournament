//! Reinforcement learning utilities including policies and value estimators.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

/// Error types for RL operations.
pub mod error;
/// Policy implementations.
pub mod policy;
/// Common data structures.
pub mod types;
/// Value estimator implementations.
pub mod value;

pub use error::RLError;
pub use policy::{Policy, PolicyType, RandomPolicy, TorchPolicy};
pub use types::{Action, Observation, TrainingBatch};
pub use value::{TorchValueEstimator, ValueEstimator};

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn random_policy_returns_action_in_range() {
        let mut policy = RandomPolicy::new(5, Some(42));
        let action = policy.select_action(&vec![0.0, 0.0]).unwrap();
        assert!((0..5).contains(&action));
    }

    #[test]
    fn torch_policy_can_save_load_and_infer() {
        let mut policy = TorchPolicy::new(4, 2);
        let dir = tempdir().unwrap();
        let path = dir.path().join("policy.ot");
        policy.save(&path).unwrap();

        let mut loaded = TorchPolicy::new(4, 2);
        loaded.load(&path).unwrap();

        let obs = vec![1.0, 2.0, 3.0, 4.0];
        let a1 = policy.select_action(&obs).unwrap();
        let a2 = loaded.select_action(&obs).unwrap();
        assert_eq!(a1, a2);
    }

    #[test]
    fn torch_value_estimator_save_load_consistent() {
        let estimator = TorchValueEstimator::new(4);
        let dir = tempdir().unwrap();
        let path = dir.path().join("value.ot");
        estimator.save(&path).unwrap();

        let mut loaded = TorchValueEstimator::new(4);
        loaded.load(&path).unwrap();

        let obs = vec![0.1, 0.2, 0.3, 0.4];
        let v1 = estimator.get_value(&obs).unwrap();
        let v2 = loaded.get_value(&obs).unwrap();
        assert!((v1 - v2).abs() < f32::EPSILON);
    }
}
