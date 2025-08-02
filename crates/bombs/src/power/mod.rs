//! Bomb power effects and kicking mechanics.

pub mod kick;
pub mod power_calc;

pub use kick::{Direction, kick_bomb};
pub use power_calc::affected_tiles;
