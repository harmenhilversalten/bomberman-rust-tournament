use std::time::Duration;

/// Configuration options for a [`Bot`].
#[derive(Debug, Clone)]
pub struct BotConfig {
    /// Unique identifier for the bot.
    pub id: u32,
    /// Maximum allowed time for making a single decision.
    pub decision_timeout: Duration,
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            id: 0,
            decision_timeout: Duration::from_millis(2),
        }
    }
}
