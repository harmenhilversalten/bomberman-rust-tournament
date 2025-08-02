//! Bomb timing utilities including countdown timers and remote detonation.

pub mod remote;
pub mod timer;

pub use remote::RemoteDetonator;
pub use timer::BombTimer;
