//! Utilities for analyzing bomb danger and opportunities.

pub mod danger;
pub mod opportunity;

pub use danger::{danger_tiles, is_safe};
pub use opportunity::{opportunity_tiles, safe_tiles};
