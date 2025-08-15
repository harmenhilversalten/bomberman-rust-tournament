use state::GameState;
use thiserror::Error;

/// Identifier for a bot instance.
pub type BotId = events::events::bot_events::BotId;

/// Actions that goals can plan and execute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Do nothing this tick.
    Wait,
    /// Move in a specific direction.
    Move(common::Direction),
    /// Place a bomb at current position.
    PlaceBomb,
    /// Move towards a specific position.
    MoveTowards { 
        /// Target x coordinate.
        x: u16, 
        /// Target y coordinate.
        y: u16 
    },
    /// Escape from danger area.
    EscapeDanger,
}

/// High-level goal categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GoalType {
    /// Collect a power-up from the board.
    CollectPowerUp,
    /// Move to avoid danger such as explosions.
    AvoidDanger,
    /// Attack nearby enemies.
    AttackEnemy,
    /// Destroy soft blocks to clear paths or find power-ups.
    DestroyBlocks,
}

/// Errors that can occur during goal planning.
#[derive(Debug, Error)]
pub enum GoalError {
    /// Planning failed for some reason.
    #[error("planning failed: {0}")]
    Planning(String),
}

/// Trait implemented by all goals.
pub trait Goal: Send + Sync + GoalClone {
    /// Type of the goal.
    fn get_goal_type(&self) -> GoalType;
    /// Priority value used during scoring.
    fn get_priority(&self, state: &GameState, bot_id: BotId) -> f32;
    /// Whether the goal can currently be achieved.
    fn is_achievable(&self, state: &GameState, bot_id: BotId) -> bool;
    /// Progress towards completion in range [0,1].
    fn get_progress(&self, state: &GameState, bot_id: BotId) -> f32;
    /// Whether the goal has been completed.
    fn is_completed(&self, state: &GameState, bot_id: BotId) -> bool;
    /// Produce a plan to reach the goal.
    fn plan(&self, state: &GameState, bot_id: BotId) -> Result<Vec<Action>, GoalError>;
}

/// Helper trait to enable cloning boxed goals.
pub trait GoalClone {
    /// Clone the boxed goal.
    fn clone_box(&self) -> Box<dyn Goal>;
}

impl<T> GoalClone for T
where
    T: 'static + Goal + Clone,
{
    fn clone_box(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Goal> {
    fn clone(&self) -> Box<dyn Goal> {
        self.clone_box()
    }
}
