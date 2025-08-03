//! Test utilities and mocks used across integration tests.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

use std::time::Duration;

/// Module containing simple mock types.
pub mod mocks {
    use events::events::Event;
    use std::sync::Mutex;

    /// Mock event bus that stores broadcast events for inspection.
    #[derive(Default)]
    pub struct MockEventBus {
        events: Mutex<Vec<Event>>,
    }

    impl MockEventBus {
        /// Create a new empty mock bus.
        pub fn new() -> Self {
            Self::default()
        }

        /// Record a broadcast event.
        pub fn broadcast(&self, event: Event) {
            self.events.lock().expect("lock").push(event);
        }

        /// Retrieve all recorded events.
        pub fn events(&self) -> Vec<Event> {
            self.events.lock().expect("lock").clone()
        }

        /// Clear all stored events.
        pub fn clear(&self) {
            self.events.lock().expect("lock").clear();
        }
    }
}

/// Assertion helpers used in tests.
pub mod assertions {
    use std::time::Duration;

    /// Assert that a duration is within an expected bound.
    pub fn assert_performance_within_bounds(duration: Duration, max: Duration) {
        assert!(
            duration <= max,
            "Performance exceeded bounds: {:?} > {:?}",
            duration,
            max
        );
    }
}

/// Initializes the crate and returns a greeting.
pub fn init() -> &'static str {
    "initialized"
}

#[cfg(test)]
mod tests {
    use super::*;
    use events::events::GameEvent;

    #[test]
    fn init_returns_initialized() {
        assert_eq!(init(), "initialized");
    }

    #[test]
    fn mock_event_bus_records_events() {
        use events::events::Event;
        use mocks::MockEventBus;

        let bus = MockEventBus::new();
        bus.broadcast(Event::Game(GameEvent::TickCompleted { tick: 1 }));
        assert_eq!(bus.events().len(), 1);
        bus.clear();
        assert!(bus.events().is_empty());
    }
}
