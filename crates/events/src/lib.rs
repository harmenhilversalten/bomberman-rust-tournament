#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

//! Event definitions and bus for the Bomberman project.

pub mod bus;
pub mod events;
pub mod queue;

pub use bus::{EventBus, EventFilter, SubscriberId};
pub use events::{BotDecision, BotEvent, Event, GameEvent, SystemEvent};
pub use queue::EventPriority;
