//! Core bot functionality.

/// Configuration options for bots.
pub mod config;
/// Decision-making trait used by the kernel.
pub mod decision;
/// Bot kernel and run loop.
pub mod kernel;
/// Runtime statistics tracked for each bot.
pub mod state;

pub use config::BotConfig;
pub use decision::DecisionMaker;
pub use kernel::{Bot, BotHandle};
pub use state::BotState;
