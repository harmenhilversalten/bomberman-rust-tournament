//! Bot-specific events.

/// Identifier for a bot instance.
pub type BotId = usize;

use common::Direction;
use serde::{Deserialize, Serialize};

/// Decisions that a bot might produce.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BotDecision {
    /// Bot chose to wait.
    Wait,
    /// Bot decided to move.
    Move(Direction),
    /// Bot decided to place a bomb.
    PlaceBomb,
}

/// Events emitted by or for bots.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BotEvent {
    /// A decision was made by a bot.
    Decision {
        /// Identifier of the bot.
        bot_id: BotId,
        /// Decision made.
        decision: BotDecision,
    },
    /// Status update from a bot (e.g., current goal label).
    Status {
        /// Identifier of the bot.
        bot_id: BotId,
        /// Human-readable status text (e.g., current goal).
        status: String,
    },
    /// An error occurred for a bot.
    Error {
        /// Identifier of the bot.
        bot_id: BotId,
        /// Error message.
        message: String,
    },
}
