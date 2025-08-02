//! Goal evaluation helpers.

use std::collections::HashMap;

use state::GameState;

use crate::goal::{self, BotId, Goal, GoalType};

/// Evaluates a goal using its priority and configured weights.
pub fn evaluate_goal(
    goal: &dyn Goal,
    state: &GameState,
    bot_id: BotId,
    weights: &HashMap<GoalType, f32>,
) -> f32 {
    let base = goal.get_priority(state, bot_id);
    let weight = weights.get(&goal.get_goal_type()).copied().unwrap_or(1.0);
    goal::priority::weighted_priority(base, weight)
}
