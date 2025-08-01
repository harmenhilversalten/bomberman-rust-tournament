//! Game state management modules.

/// Entity identifier definitions.
pub mod entity;
/// Wrapper around the game grid.
pub mod game_state;
/// Snapshot support (placeholder).
pub mod snapshot;

pub use game_state::GameState;
