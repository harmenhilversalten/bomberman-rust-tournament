//! Simple gym-like environment for Bomberman training.

use crate::{
    error::RLError,
    policy::Policy,
    types::{Action, Observation, TrainingBatch},
};

use super::{ActionSpace, ObservationSpace, RewardCalculator};

/// Minimal environment representing a 1D line with a goal position.
pub struct RLEnvironment<R: RewardCalculator> {
    position: i32,
    goal: i32,
    observation_space: ObservationSpace,
    action_space: ActionSpace,
    reward_calculator: R,
    episode_length: u32,
    current_step: u32,
}

impl<R: RewardCalculator> RLEnvironment<R> {
    /// Create a new environment with the specified goal and episode length.
    pub fn new(goal: i32, episode_length: u32, reward_calculator: R) -> Self {
        Self {
            position: 0,
            goal,
            observation_space: ObservationSpace { size: 1 },
            action_space: ActionSpace { actions: 2 },
            reward_calculator,
            episode_length,
            current_step: 0,
        }
    }

    /// Reset the environment to the starting state returning the initial observation.
    pub fn reset(&mut self) -> Observation {
        self.position = 0;
        self.current_step = 0;
        self.get_observation()
    }

    /// Take a step in the environment using the provided action.
    pub fn step(&mut self, action: Action) -> Result<(Observation, f32, bool), RLError> {
        match action {
            0 => self.position -= 1,
            1 => self.position += 1,
            _ => {}
        }
        if self.position < 0 {
            self.position = 0;
        }
        if self.position > self.goal {
            self.position = self.goal;
        }
        self.current_step += 1;
        let done = self.position == self.goal || self.current_step >= self.episode_length;
        let reward = self
            .reward_calculator
            .calculate(self.position, self.goal, done);
        Ok((self.get_observation(), reward, done))
    }

    /// Return the observation for the current state.
    fn get_observation(&self) -> Observation {
        vec![self.position as f32 / self.goal as f32]
    }

    /// Run an episode using the provided policy collecting a batch of transitions.
    pub fn run_episode<P: Policy>(
        &mut self,
        policy: &mut P,
        max_steps: u32,
    ) -> Result<TrainingBatch, RLError> {
        let mut batch = TrainingBatch::default();
        let mut obs = self.reset();
        for _ in 0..max_steps {
            let action = policy.select_action(&obs)?;
            let (next_obs, reward, done) = self.step(action)?;
            batch.observations.push(obs.clone());
            batch.actions.push(action);
            batch.rewards.push(reward);
            batch.next_observations.push(next_obs.clone());
            batch.dones.push(done);
            obs = next_obs;
            if done {
                break;
            }
        }
        Ok(batch)
    }

    /// Access the observation space.
    pub fn observation_space(&self) -> ObservationSpace {
        self.observation_space
    }

    /// Access the action space.
    pub fn action_space(&self) -> ActionSpace {
        self.action_space
    }
}
