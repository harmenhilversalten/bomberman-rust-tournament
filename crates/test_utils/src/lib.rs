//! Test utilities and mock objects for the Bomberman project.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

/// Collection of simple mock types used across tests.
pub mod mocks {
    use std::sync::{Arc, Mutex};

    use bot::{Action, BotState};
    use events::events::Event;
    use state::GameGrid;

    /// In-memory event bus capturing emitted events.
    #[derive(Default, Clone)]
    pub struct MockEventBus {
        events: Arc<Mutex<Vec<Event>>>,
    }

    impl MockEventBus {
        /// Create a new empty mock bus.
        pub fn new() -> Self {
            Self {
                events: Arc::default(),
            }
        }

        /// Record an event that would have been broadcast.
        pub fn broadcast(&self, event: Event) {
            self.events.lock().unwrap().push(event);
        }

        /// Retrieve all recorded events.
        pub fn get_events(&self) -> Vec<Event> {
            self.events.lock().unwrap().clone()
        }

        /// Clear the recorded events.
        pub fn clear(&self) {
            self.events.lock().unwrap().clear();
        }
    }

    /// Simplified game grid used for tests.
    pub struct MockGameGrid {
        grid: GameGrid,
    }

    impl MockGameGrid {
        /// Create a basic grid with the given dimensions filled with empty tiles.
        pub fn new(width: usize, height: usize) -> Self {
            Self {
                grid: GameGrid::new(width, height),
            }
        }

        /// Access the underlying [`GameGrid`].
        pub fn inner(&self) -> &GameGrid {
            &self.grid
        }
    }

    /// Mock bot returning pre-defined decisions.
    pub struct MockBot {
        decisions: Vec<Action>,
        state: BotState,
    }

    impl MockBot {
        /// Create a mock bot that will return the supplied decisions in reverse order.
        pub fn with_predefined_decisions(decisions: Vec<Action>) -> Self {
            Self {
                decisions,
                state: BotState::default(),
            }
        }

        /// Pop the next decision from the predefined list.
        pub async fn make_decision(&mut self) -> Option<Action> {
            self.decisions.pop()
        }

        /// Access the internal [`BotState`].
        pub fn state(&self) -> &BotState {
            &self.state
        }
    }
}

/// Helpers for generating common game scenarios.
pub mod scenarios {
    use state::GameGrid;

    /// Predefined scenario types for tests.
    pub enum TestScenario {
        /// Empty grid with no entities.
        EmptyGrid,
        /// Minimal grid containing two bots.
        TwoBots,
    }

    /// Create a [`GameGrid`] based on a [`TestScenario`].
    pub fn create_test_scenario(scenario: TestScenario) -> GameGrid {
        match scenario {
            TestScenario::EmptyGrid => GameGrid::new(1, 1),
            TestScenario::TwoBots => GameGrid::new(2, 2),
        }
    }
}

/// Assertion helpers for tests.
pub mod assertions {
    use std::time::Duration;

    use bot::BotState;

    /// Assert that a bot survived for at least `duration` ticks.
    pub fn assert_bot_survived(bot_state: &BotState, duration: u32) {
        assert!(bot_state.decisions() as u32 >= duration);
    }

    /// Assert that all `achieved` goals are contained in `goals`.
    pub fn assert_goals_achieved<T: PartialEq + std::fmt::Debug>(goals: &[T], achieved: &[T]) {
        for goal in achieved {
            assert!(goals.contains(goal), "goal {:?} not achieved", goal);
        }
    }

    /// Assert that an operation completed within the allowed duration.
    pub fn assert_performance_within_bounds(duration: Duration, max_duration: Duration) {
        assert!(
            duration <= max_duration,
            "Performance exceeded bounds: {:?} > {:?}",
            duration,
            max_duration
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
    use events::events::{Event, GameEvent};
    use std::time::Duration;

    #[test]
    fn init_returns_initialized() {
        assert_eq!(init(), "initialized");
    }

    #[test]
    fn mock_event_bus_records_events() {
        let bus = mocks::MockEventBus::new();
        bus.broadcast(Event::Game(GameEvent::TickCompleted { tick: 1 }));
        assert_eq!(bus.get_events().len(), 1);
    }

    #[test]
    fn assertions_detect_performance_bounds() {
        assertions::assert_performance_within_bounds(
            Duration::from_millis(1),
            Duration::from_millis(10),
        );
    }
}
