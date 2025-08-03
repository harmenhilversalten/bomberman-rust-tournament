//! Bomb placement strategies.

pub mod placer;
pub mod safe;
pub mod strategic;
pub mod tactical;

pub use placer::PlacementStrategy;
pub use safe::SafePlacer;
pub use strategic::StrategicPlacer;
pub use tactical::{BombPlacementStrategy, TacticalPlacement};
