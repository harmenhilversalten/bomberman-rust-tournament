use crate::state::GameState;

use super::{Format, SerializableState, SerializationError};

/// Encode the provided game state into the selected format.
pub fn encode(state: &GameState, format: Format) -> Result<Vec<u8>, SerializationError> {
    let data = SerializableState::from(state);
    match format {
        Format::Binary => bincode::serialize(&data).map_err(SerializationError::Binary),
        Format::Json => serde_json::to_vec(&data).map_err(SerializationError::Json),
    }
}
