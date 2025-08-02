#![allow(clippy::module_inception)]
//! Goal execution helpers.

/// Goal execution logic.
pub mod executor;
/// Progress monitoring utilities.
pub mod monitor;

pub use executor::GoalExecutor;
pub use monitor::ProgressMonitor;
