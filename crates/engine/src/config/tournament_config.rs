use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentConfig {
    pub name: String,
    pub format: TournamentFormat,
    pub max_concurrent_games: usize,
    pub game_timeout_seconds: u64,
    pub scoring_system: ScoringSystem,
    pub registration_timeout_seconds: u64,
    pub allow_remote_bots: bool,
    pub persist_results: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TournamentFormat {
    RoundRobin { total_rounds: u32 },
    SingleElimination { bracket_size: u32 },
    Swiss { rounds: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScoringSystem {
    WinLoss {
        win_points: u32,
        loss_points: u32,
    },
    Survival {
        time_multiplier: f32,
    },
    Destruction {
        crate_points: u32,
        enemy_points: u32,
    },
    Hybrid {
        weights: HashMap<String, f32>,
    },
}
