//! Representation of an agent playing the game.

use serde::{Deserialize, Serialize};

/// Current state for a single agent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentState {
    /// Unique agent identifier.
    pub id: usize,
    /// Agent position on the grid.
    pub position: (u16, u16),
    /// Remaining bombs the agent can place.
    pub bombs_left: u8,
    /// Blast radius of bombs placed by this agent.
    pub power: u8,
}

impl AgentState {
    /// Creates a new agent state at the given position.
    pub fn new(id: usize, position: (u16, u16)) -> Self {
        Self {
            id,
            position,
            bombs_left: 1,
            power: 1,
        }
    }
}
