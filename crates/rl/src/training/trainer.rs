//! Basic training loop using a replay buffer.

use crate::{error::RLError, policy::Policy};

use super::ReplayBuffer;
use crate::environment::{RLEnvironment, RewardCalculator};

/// Trainer coordinating environment interaction and policy updates.
pub struct Trainer<P, R>
where
    P: Policy,
    R: RewardCalculator,
{
    env: RLEnvironment<R>,
    policy: P,
    buffer: ReplayBuffer,
}

impl<P, R> Trainer<P, R>
where
    P: Policy,
    R: RewardCalculator,
{
    /// Create a new trainer with the given components.
    pub fn new(env: RLEnvironment<R>, policy: P, buffer_capacity: usize) -> Self {
        Self {
            env,
            policy,
            buffer: ReplayBuffer::new(buffer_capacity),
        }
    }

    /// Run a number of training episodes.
    pub fn train(
        &mut self,
        episodes: u32,
        max_steps: u32,
        batch_size: usize,
    ) -> Result<(), RLError> {
        for _ in 0..episodes {
            let batch = self.env.run_episode(&mut self.policy, max_steps)?;
            for i in 0..batch.actions.len() {
                self.buffer.push(
                    batch.observations[i].clone(),
                    batch.actions[i],
                    batch.rewards[i],
                    batch.next_observations[i].clone(),
                    batch.dones[i],
                );
            }
            if self.buffer.len() >= batch_size {
                let sample = self.buffer.sample(batch_size);
                self.policy.update(&sample)?;
            }
        }
        Ok(())
    }

    /// Access internal buffer for testing.
    pub fn buffer(&self) -> &ReplayBuffer {
        &self.buffer
    }
}
