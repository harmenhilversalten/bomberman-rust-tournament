//! Simple in-memory replay buffer.

use crate::types::{Action, Observation, TrainingBatch};

/// Fixed-size replay buffer storing recent transitions.
pub struct ReplayBuffer {
    capacity: usize,
    batch: TrainingBatch,
}

impl ReplayBuffer {
    /// Create a new buffer with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            batch: TrainingBatch::default(),
        }
    }

    /// Number of stored transitions.
    pub fn len(&self) -> usize {
        self.batch.actions.len()
    }

    /// Returns `true` if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Push a single transition into the buffer, evicting oldest if necessary.
    pub fn push(
        &mut self,
        obs: Observation,
        action: Action,
        reward: f32,
        next_obs: Observation,
        done: bool,
    ) {
        if self.len() >= self.capacity {
            self.batch.observations.remove(0);
            self.batch.actions.remove(0);
            self.batch.rewards.remove(0);
            self.batch.next_observations.remove(0);
            self.batch.dones.remove(0);
        }
        self.batch.observations.push(obs);
        self.batch.actions.push(action);
        self.batch.rewards.push(reward);
        self.batch.next_observations.push(next_obs);
        self.batch.dones.push(done);
    }

    /// Sample a batch of the most recent transitions.
    pub fn sample(&self, batch_size: usize) -> TrainingBatch {
        let start = self.len().saturating_sub(batch_size);
        TrainingBatch {
            observations: self.batch.observations[start..].to_vec(),
            actions: self.batch.actions[start..].to_vec(),
            rewards: self.batch.rewards[start..].to_vec(),
            next_observations: self.batch.next_observations[start..].to_vec(),
            dones: self.batch.dones[start..].to_vec(),
        }
    }
}
