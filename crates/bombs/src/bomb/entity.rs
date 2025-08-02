//! Bomb structure and identifiers.

use serde::{Deserialize, Serialize};

/// Grid position represented by (x, y).
pub type Position = (u16, u16);

/// Unique identifier for a bomb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BombId(pub u32);

/// Bomb instance with properties relevant for chain reactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bomb {
    /// Identifier for this bomb.
    pub id: BombId,
    /// Identifier of the owning agent.
    pub owner: usize,
    /// Position on the grid.
    pub position: Position,
    /// Ticks until this bomb explodes.
    pub timer: u8,
    /// Blast radius of the bomb.
    pub power: u8,
    /// Whether the blast pierces through obstacles.
    pub pierce: bool,
    /// Whether the bomb can be detonated remotely.
    pub remote: bool,
    /// Whether the bomb can be kicked by agents.
    pub kickable: bool,
    /// Optional chain identifier this bomb belongs to.
    pub chain_id: Option<crate::bomb::chain::BombChainId>,
}

impl Bomb {
    /// Creates a new bomb with default flags.
    pub fn new(id: BombId, owner: usize, position: Position, timer: u8, power: u8) -> Self {
        Self {
            id,
            owner,
            position,
            timer,
            power,
            pierce: false,
            remote: false,
            kickable: false,
            chain_id: None,
        }
    }
}
