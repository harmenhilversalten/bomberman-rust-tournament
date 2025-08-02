//! Bomb logic crate providing chain reaction and explosion calculations.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

pub mod analysis;
pub mod bomb;
pub mod placement;
pub mod power;
pub mod timing;

pub use bomb::{
    BombError, BombManager,
    chain::{BombChain, BombChainId},
    entity::{Bomb, BombId, Position},
    explosion::Explosion,
};

pub use analysis::{danger_tiles, is_safe, opportunity_tiles, safe_tiles};
pub use placement::{PlacementStrategy, SafePlacer, StrategicPlacer};
pub use power::{Direction, affected_tiles, kick_bomb};
pub use timing::{BombTimer, RemoteDetonator};
