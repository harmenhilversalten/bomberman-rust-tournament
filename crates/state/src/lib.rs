#![deny(unsafe_code)]
#![warn(missing_docs, clippy::all)]

//! Bomberman game state crate.

pub mod components;
pub mod grid;
pub mod state;

pub use components::{AgentState, Bomb};
pub use grid::{GameGrid, Tile};
pub use state::{GameState, SnapshotView};

/// Initializes the crate and returns a greeting.
pub fn init() -> &'static str {
    "initialized"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_initializes() {
        assert_eq!(init(), "initialized");
    }
}
