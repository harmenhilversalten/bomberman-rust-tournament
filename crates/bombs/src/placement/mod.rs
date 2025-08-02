//! Bomb placement strategies.

pub mod placer;
pub mod safe;
pub mod strategic;

pub use placer::PlacementStrategy;
pub use safe::SafePlacer;
pub use strategic::StrategicPlacer;
