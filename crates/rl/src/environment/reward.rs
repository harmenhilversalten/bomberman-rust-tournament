//! Reward calculation utilities.

/// Trait for computing rewards given the environment state.
pub trait RewardCalculator {
    /// Calculate the reward for the current position.
    fn calculate(&self, position: i32, goal: i32, done: bool) -> f32;
}

/// Simple reward: small negative per step, +1 on reaching goal.
#[derive(Default)]
pub struct SimpleReward;

impl RewardCalculator for SimpleReward {
    fn calculate(&self, position: i32, goal: i32, done: bool) -> f32 {
        if done && position == goal { 1.0 } else { -0.01 }
    }
}
