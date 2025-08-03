use thiserror::Error;

/// Errors that can occur when using the [`EventBus`](crate::EventBus).
#[derive(Debug, Error)]
pub enum EventBusError {
    /// Too many subscribers are registered.
    #[error("Subscription limit exceeded: {current}/{max}")]
    SubscriptionLimit {
        /// Number of current subscribers.
        current: usize,
        /// Maximum allowed subscribers.
        max: usize,
    },
    /// Serializing an event failed.
    #[error("Event serialization failed: {0}")]
    Serialization(String),
    /// Deserializing an event failed.
    #[error("Event deserialization failed: {0}")]
    Deserialization(String),
    /// The broadcast queue is full.
    #[error("Broadcast queue full: {current}/{max}")]
    BroadcastQueueFull {
        /// Current queue size.
        current: usize,
        /// Maximum queue capacity.
        max: usize,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_variant() {
        let e = EventBusError::BroadcastQueueFull {
            current: 10,
            max: 5,
        };
        assert!(format!("{}", e).contains("10/5"));
    }
}
