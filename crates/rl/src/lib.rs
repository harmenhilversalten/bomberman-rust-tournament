//! Reinforcement learning utilities for Bomberman agents.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

/// Common error type for the RL crate.
pub mod error;
/// Policy implementations.
pub mod policy;
/// Core data types used by policies and value estimators.
pub mod types;
/// Value estimation implementations.
pub mod value;

#[cfg(test)]
mod tests {
    use crate::policy::{Policy, RandomPolicy, TorchPolicy};
    use crate::types::Observation;

    #[test]
    fn random_policy_returns_within_range() {
        let mut policy = RandomPolicy::new(3);
        let obs = Observation::new(vec![0.0, 1.0]);
        let action = policy.select_action(&obs).unwrap();
        assert!((0..3).contains(&action));
    }

    #[test]
    fn torch_policy_save_and_load_roundtrip() {
        let mut policy = TorchPolicy::new(2, 3);
        let obs = Observation::new(vec![0.5, -0.2]);
        let before = policy.select_action(&obs).unwrap();

        let path = std::env::temp_dir().join("torch_policy_roundtrip.ot");
        policy.save(&path).unwrap();

        let mut loaded = TorchPolicy::new(2, 3);
        loaded.load(&path).unwrap();
        let after = loaded.select_action(&obs).unwrap();
        assert_eq!(before, after);
        let _ = std::fs::remove_file(path);
    }
}
