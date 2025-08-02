use serde::{Deserialize, Serialize};

/// Rules governing gameplay.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameRules {
    /// Maximum number of players allowed.
    pub max_players: u8,
    /// Number of ticks before a bomb explodes.
    pub bomb_timer: u32,
    /// Starting lives for each player.
    pub starting_lives: u8,
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            max_players: 4,
            bomb_timer: 3,
            starting_lives: 3,
        }
    }
}
