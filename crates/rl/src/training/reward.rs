use crate::types::{Action, Observation};

/// Record of a single transition used for reward analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct RewardRecord {
    /// Observation before taking the action.
    pub observation: Observation,
    /// Action taken by the agent.
    pub action: Action,
    /// Reward received after the action.
    pub reward: f32,
    /// Observation after the action.
    pub next_observation: Observation,
    /// Whether the episode terminated.
    pub done: bool,
}

/// Calculate reward based on observation differences.
pub fn calculate_reward(
    prev_state: &Observation,
    current_state: &Observation,
    _action: Action,
    _agent_id: usize,
) -> f32 {
    let prev_sum: f32 = prev_state.iter().sum();
    let curr_sum: f32 = current_state.iter().sum();
    curr_sum - prev_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reward_is_difference_between_states() {
        let prev = vec![0.0, 1.0];
        let curr = vec![1.0, 2.0];
        let r = calculate_reward(&prev, &curr, 0, 0);
        assert_eq!(r, 2.0);
    }
}
