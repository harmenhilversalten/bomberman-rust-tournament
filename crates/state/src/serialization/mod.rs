//! Serialization and deserialization of the game state.

use serde::{Deserialize, Serialize};

use crate::{
    components::{AgentState, Bomb},
    grid::{GameGrid, Tile},
    state::GameState,
};

/// Supported serialization formats.
#[derive(Debug, Clone, Copy)]
pub enum Format {
    /// Binary format using bincode.
    Binary,
    /// JSON text format.
    Json,
}

/// Errors that can occur during serialization or deserialization.
#[derive(Debug)]
pub enum SerializationError {
    /// Error with binary encoding/decoding.
    Binary(bincode::Error),
    /// Error with JSON encoding/decoding.
    Json(serde_json::Error),
}

impl From<bincode::Error> for SerializationError {
    fn from(err: bincode::Error) -> Self {
        SerializationError::Binary(err)
    }
}

impl From<serde_json::Error> for SerializationError {
    fn from(err: serde_json::Error) -> Self {
        SerializationError::Json(err)
    }
}

/// Internal representation of the game state for serialization.
#[derive(Serialize, Deserialize)]
pub(crate) struct SerializableState {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    bombs: Vec<Bomb>,
    agents: Vec<AgentState>,
    version: u64,
}

impl From<&GameState> for SerializableState {
    fn from(state: &GameState) -> Self {
        Self {
            width: state.grid.width(),
            height: state.grid.height(),
            tiles: state.grid.tiles().to_vec(),
            bombs: state.grid.bombs().to_vec(),
            agents: state.grid.agents().to_vec(),
            version: state.grid.version(),
        }
    }
}

impl From<SerializableState> for GameState {
    fn from(s: SerializableState) -> Self {
        let grid = GameGrid::from_parts(s.width, s.height, s.tiles, s.bombs, s.agents, s.version);
        GameState { grid }
    }
}

/// Utilities for decoding state.
pub mod decoder;
/// Utilities for encoding state.
pub mod encoder;

#[cfg(test)]
mod tests {
    use super::{Format, decoder, encoder};
    use crate::{
        components::{AgentState, Bomb},
        grid::{GridDelta, Tile},
        state::GameState,
    };

    #[test]
    fn round_trip_binary_and_json() {
        let mut state = GameState::new(2, 2);
        state.apply_delta(GridDelta::SetTile {
            x: 0,
            y: 1,
            tile: Tile::Wall,
        });
        state.apply_delta(GridDelta::AddBomb(Bomb::new(1, (0, 0), 3, 1)));
        state.apply_delta(GridDelta::AddAgent(AgentState::new(1, (1, 1))));

        for format in [Format::Binary, Format::Json] {
            let bytes = encoder::encode(&state, format).expect("encode");
            let decoded = decoder::decode(&bytes, format).expect("decode");
            assert_eq!(decoded.grid.width(), 2);
            assert_eq!(decoded.grid.height(), 2);
            assert_eq!(decoded.grid.tile(0, 1), Some(Tile::Wall));
            assert_eq!(decoded.grid.bombs().len(), 1);
            assert_eq!(decoded.grid.agents().len(), 1);
        }
    }
}
