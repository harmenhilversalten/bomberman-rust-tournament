//! Bomb-related events.

use serde::{Deserialize, Serialize};

/// Identifier for an agent.
pub type AgentId = usize;

/// Grid position for events.
pub type Position = (u16, u16);

/// Possible power-up types affecting bombs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PowerUpType {
    /// Increases the number of bombs an agent can carry.
    BombCount,
    /// Extends the blast radius of bombs.
    BlastRadius,
}

/// Events related to bomb mechanics.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BombEvent {
    /// A bomb was placed by an agent.
    Placed {
        /// Identifier of the agent placing the bomb.
        agent_id: AgentId,
        /// Grid position where the bomb was placed.
        position: Position,
    },
    /// A bomb exploded at a position with a given radius.
    Exploded {
        /// Center position of the explosion.
        position: Position,
        /// Blast radius of the explosion.
        radius: u32,
    },
    /// A chain reaction occurred affecting multiple positions.
    ChainReaction {
        /// Positions impacted during the chain reaction.
        positions: Vec<Position>,
    },
    /// An agent collected a power-up affecting bombs.
    PowerUpCollected {
        /// Identifier of the agent collecting the power-up.
        agent_id: AgentId,
        /// Type of power-up collected.
        power_type: PowerUpType,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bomb_event_serializes() {
        let ev = BombEvent::Placed {
            agent_id: 1,
            position: (0, 0),
        };
        let json = serde_json::to_string(&ev).unwrap();
        assert!(json.contains("Placed"));
    }
}
