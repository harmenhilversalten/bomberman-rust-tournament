#![forbid(unsafe_code)]
#![allow(clippy::all)]

pub mod bot;
pub mod config;
pub mod coord;
pub mod engine;
pub mod game;
pub mod map;
pub mod shrink;
pub mod simulation;
pub mod systems;

pub use config::{ConfigError, EngineConfig, GameRules};
pub use engine::{Engine, TaskScheduler};
pub use simulation::{DeterminismChecker, Replay, ReplayRecorder};
pub use systems::System;
