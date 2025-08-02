//! Event type definitions.

pub mod bot_events;
pub mod game_events;
pub mod system_events;

pub use bot_events::{BotDecision, BotEvent};
pub use game_events::GameEvent;
pub use system_events::SystemEvent;

/// Wrapper enum combining all event categories.
use serde::{Deserialize, Serialize};

/// Wrapper enum combining all event categories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Event {
    /// Game-related event.
    Game(GameEvent),
    /// Bot-specific event.
    Bot(BotEvent),
    /// Internal system event.
    System(SystemEvent),
}
