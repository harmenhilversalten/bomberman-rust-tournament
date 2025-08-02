//! Bomb logic crate providing chain reaction and explosion calculations.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

pub mod bomb;

pub use bomb::{
    BombError, BombManager,
    chain::{BombChain, BombChainId},
    entity::{Bomb, BombId, Position},
    explosion::Explosion,
};
