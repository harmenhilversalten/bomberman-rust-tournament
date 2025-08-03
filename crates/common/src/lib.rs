#![forbid(unsafe_code)]

pub mod diagnostics;
pub mod error;
pub mod logging;

pub use error::{BombermanError, Result};
