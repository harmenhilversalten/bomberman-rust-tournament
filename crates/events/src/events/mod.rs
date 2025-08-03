//! Event type definitions.

pub mod bomb_events;
pub mod bot_events;
pub mod game_events;
pub mod system_events;

use state::grid::GridDelta;

pub use bomb_events::{BombEvent, PowerUpType};
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
    /// State change event.
    Grid(GridDelta),
    /// Bomb-related event.
    Bomb(BombEvent),
}

impl Event {
    /// Convenience constructor for bomb events.
    pub fn bomb(event: BombEvent) -> Self {
        Event::Bomb(event)
    }
}
