#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

//! Event definitions and bus for the Bomberman project.

pub mod bus;
pub mod events;
pub mod queue;
pub mod serialization;

pub use bus::{EventBus, EventFilter, SubscriberId};
pub use events::{BombEvent, BotDecision, BotEvent, Event, GameEvent, PowerUpType, SystemEvent};
pub use queue::EventPriority;
pub use serialization::{Transition, TransitionRecorder, decoder, encoder};
