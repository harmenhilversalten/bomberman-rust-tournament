use crate::goal::{AvoidDangerGoal, CollectPowerUpGoal, Goal};
use state::GameState;

/// Trait for types that can generate goals from a game snapshot.
pub trait GoalGenerator {
    /// Generate goals given the current game state snapshot.
    fn generate(&self, snapshot: &GameState) -> Vec<Box<dyn Goal>>;
}

/// Manager responsible for producing goals for the bot.
#[derive(Default)]
pub struct GoalManager;

impl GoalManager {
    /// Create a new [`GoalManager`].
    pub fn new() -> Self {
        Self
    }

    /// Generate the list of currently relevant goals.
    pub fn generate_goals(&self, snapshot: &GameState) -> Vec<Box<dyn Goal>> {
        // For now include a couple of basic goals.
        let _ = snapshot;
        vec![
            Box::new(CollectPowerUpGoal) as Box<dyn Goal>,
            Box::new(AvoidDangerGoal) as Box<dyn Goal>,
        ]
    }
}

impl GoalGenerator for GoalManager {
    fn generate(&self, snapshot: &GameState) -> Vec<Box<dyn Goal>> {
        self.generate_goals(snapshot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manager_produces_goals() {
        let manager = GoalManager::new();
        let goals = manager.generate_goals(&GameState::new(1, 1));
        assert_eq!(goals.len(), 2);
    }
}
