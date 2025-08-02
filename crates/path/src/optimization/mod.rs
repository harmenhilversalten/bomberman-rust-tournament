//! Path optimization algorithms.

mod simplification;
mod smoothing;

pub use simplification::simplify_path;
pub use smoothing::smooth_path;
