use crate::goal::{AvoidDangerGoal, CollectPowerUpGoal, AttackEnemyGoal, DestroyBlocksGoal, Goal};
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
        // Generate all available goals for intelligent planning
        let _ = snapshot;
        vec![
            Box::new(AttackEnemyGoal) as Box<dyn Goal>,  // Highest priority - aggressive play
            Box::new(DestroyBlocksGoal) as Box<dyn Goal>, // High priority - map control
            Box::new(AvoidDangerGoal) as Box<dyn Goal>,   // Medium priority - survival
            Box::new(CollectPowerUpGoal) as Box<dyn Goal>, // Lower priority - power progression
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
