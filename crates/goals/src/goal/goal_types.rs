//! Built-in goal implementations.

use super::{Action, BotId, Goal, GoalError, GoalType};
use state::GameState;

/// Goal to collect a nearby power-up.
#[derive(Debug, Clone)]
pub struct CollectPowerUpGoal;

impl Goal for CollectPowerUpGoal {
    fn get_goal_type(&self) -> GoalType {
        GoalType::CollectPowerUp
    }

    fn get_priority(&self, _state: &GameState, _bot_id: BotId) -> f32 {
        10.0
    }

    fn is_achievable(&self, _state: &GameState, _bot_id: BotId) -> bool {
        true
    }

    fn get_progress(&self, _state: &GameState, _bot_id: BotId) -> f32 {
        0.0
    }

    fn is_completed(&self, _state: &GameState, _bot_id: BotId) -> bool {
        false
    }

    fn plan(&self, _state: &GameState, _bot_id: BotId) -> Result<Vec<Action>, GoalError> {
        Ok(vec![Action::Wait])
    }
}

/// Goal to move away from danger.
#[derive(Debug, Clone)]
pub struct AvoidDangerGoal;

impl Goal for AvoidDangerGoal {
    fn get_goal_type(&self) -> GoalType {
        GoalType::AvoidDanger
    }

    fn get_priority(&self, _state: &GameState, _bot_id: BotId) -> f32 {
        5.0
    }

    fn is_achievable(&self, _state: &GameState, _bot_id: BotId) -> bool {
        true
    }

    fn get_progress(&self, _state: &GameState, _bot_id: BotId) -> f32 {
        0.0
    }

    fn is_completed(&self, _state: &GameState, _bot_id: BotId) -> bool {
        false
    }

    fn plan(&self, _state: &GameState, _bot_id: BotId) -> Result<Vec<Action>, GoalError> {
        Ok(vec![Action::Wait])
    }
}
