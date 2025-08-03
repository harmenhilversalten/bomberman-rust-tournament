use crate::goal::{BotId, Goal};
use influence::map::{InfluenceData, Position};
use state::GameState;

/// Trait evaluating state properties used during scoring.
pub trait StateEvaluator {
    /// Evaluate the given snapshot returning a scalar score.
    fn evaluate(&self, snapshot: &GameState) -> f32;
}

/// Scorer combining heuristics and influence data.
#[derive(Default)]
pub struct GoalScorer;

impl GoalScorer {
    /// Create a new [`GoalScorer`].
    pub fn new() -> Self {
        Self
    }

    /// Score a goal based on the current snapshot and influence information.
    pub fn score_goal(
        &self,
        goal: &dyn Goal,
        snapshot: &GameState,
        influence: &InfluenceData,
        bot_id: BotId,
    ) -> f32 {
        // At this stage we simply subtract local danger from the priority.
        let base = goal.get_priority(snapshot, bot_id);
        let danger = influence.get_danger_at(Position::new(0, 0));
        base - danger
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::goal::CollectPowerUpGoal;
    use influence::map::InfluenceMap;

    #[test]
    fn scoring_returns_positive_value() {
        let scorer = GoalScorer::new();
        let state = GameState::new(1, 1);
        let mut map = InfluenceMap::new(1, 1);
        let _ = map.update(&state);
        let data = map.data();
        let score = scorer.score_goal(&CollectPowerUpGoal, &state, &data, 0);
        assert!(score > 0.0);
    }
}
