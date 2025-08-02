//! Bomb logic crate providing chain reaction and explosion calculations.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

pub mod bomb;
pub mod placement;
pub mod timing;

pub use bomb::{
    BombError, BombManager,
    chain::{BombChain, BombChainId},
    entity::{Bomb, BombId, Position},
    explosion::Explosion,
};

pub use placement::{PlacementStrategy, SafePlacer, StrategicPlacer};
pub use timing::{BombTimer, RemoteDetonator};
