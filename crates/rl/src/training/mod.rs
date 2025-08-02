//! Training utilities including replay buffers and loops.

pub mod buffer;
pub mod trainer;

pub use buffer::ReplayBuffer;
pub use trainer::Trainer;
