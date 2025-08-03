use thiserror::Error;

/// Errors that may occur within the bot crate.
#[derive(Debug, Error)]
pub enum BotError {
    /// Subscribing to events failed.
    #[error("Event subscription failed: {0}")]
    EventSubscription(String),
    /// The bot failed to make a decision within the timeout.
    #[error("Decision timeout after {timeout_ms}ms")]
    DecisionTimeout {
        /// Timeout in milliseconds.
        timeout_ms: u64,
    },
    /// The AI pipeline returned an error.
    #[error("AI decision failed: {0}")]
    AIDecision(String),
    /// Sending a command to the engine failed.
    #[error("Command send failed: {0}")]
    CommandSend(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_variants() {
        let e = BotError::DecisionTimeout { timeout_ms: 5 };
        assert_eq!(format!("{}", e), "Decision timeout after 5ms");
    }
}
