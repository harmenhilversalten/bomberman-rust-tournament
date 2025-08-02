#![forbid(unsafe_code)]
#![allow(clippy::all)]

pub mod bot;
pub mod coord;
pub mod engine;
pub mod game;
pub mod map;
pub mod shrink;
pub mod systems;

pub use engine::{Engine, TaskScheduler};
pub use systems::System;
