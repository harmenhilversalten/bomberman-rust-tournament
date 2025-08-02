#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

//! Event definitions and bus for the Bomberman project.

pub mod bus;
pub mod events;

pub use bus::{EventBus, SubscriberId};
pub use events::{BotEvent, Event, GameEvent, SystemEvent};
