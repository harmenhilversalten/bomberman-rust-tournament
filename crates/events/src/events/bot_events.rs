//! Bot-specific events.

/// Identifier for a bot instance.
pub type BotId = usize;

/// Decisions that a bot might produce.
#[derive(Debug, Clone, PartialEq)]
pub enum BotDecision {
    /// Bot chose to wait.
    Wait,
    /// Bot decided to place a bomb.
    PlaceBomb,
}

/// Events emitted by or for bots.
#[derive(Debug, Clone, PartialEq)]
pub enum BotEvent {
    /// A decision was made by a bot.
    Decision {
        /// Identifier of the bot.
        bot_id: BotId,
        /// Decision made.
        decision: BotDecision,
    },
    /// An error occurred for a bot.
    Error {
        /// Identifier of the bot.
        bot_id: BotId,
        /// Error message.
        message: String,
    },
}
