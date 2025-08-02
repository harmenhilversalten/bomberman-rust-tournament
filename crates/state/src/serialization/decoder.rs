use crate::state::GameState;

use super::{Format, SerializableState, SerializationError};

/// Decode bytes into a game state using the specified format.
pub fn decode(bytes: &[u8], format: Format) -> Result<GameState, SerializationError> {
    let data: SerializableState = match format {
        Format::Binary => bincode::deserialize(bytes).map_err(SerializationError::Binary)?,
        Format::Json => serde_json::from_slice(bytes).map_err(SerializationError::Json)?,
    };
    Ok(GameState::from(data))
}
