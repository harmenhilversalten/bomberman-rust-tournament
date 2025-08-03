use engine::{SystemInitializer, TournamentManager, UnifiedConfig};
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config/default.toml".to_string());
    let mut config = UnifiedConfig::from_file(&config_path)?;
    config = config.with_env_overrides()?;
    info!("Loaded configuration from {}", config_path);
    let mut initializer = SystemInitializer::new(config);
    let handle = initializer.initialize().await?;
    info!("System initialized successfully");
    if handle.has_tournament() {
        run_tournament(handle).await?;
    } else {
        run_single_game(handle).await?;
    }
    Ok(())
}

async fn run_single_game(handle: engine::SystemHandle) -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = handle.into_engine();
    for _ in 0..10 {
        engine.tick().unwrap();
    }
    Ok(())
}

async fn run_tournament(
    system_handle: engine::SystemHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Duration;

    let tournament_config = system_handle
        .tournament_config()
        .cloned()
        .ok_or("Tournament configuration not found")?;

    let mut tournament_manager = TournamentManager::new(tournament_config.clone(), system_handle);

    tournament_manager.start_registration().await?;
    info!("Tournament registration opened");

    tokio::time::sleep(Duration::from_secs(
        tournament_config.registration_timeout_seconds,
    ))
    .await;

    tournament_manager.start_tournament().await?;
    info!("Tournament started");

    while tournament_manager.has_next_round() {
        let results = tournament_manager.run_next_round().await?;
        info!("Completed round with {} games", results.len());
    }

    let final_results = tournament_manager.finalize_tournament().await?;
    info!("Tournament completed");
    display_tournament_results(&final_results);

    Ok(())
}

fn display_tournament_results(results: &engine::tournament::TournamentResults) {
    for (bot_id, score, rank) in &results.rankings {
        info!("Bot {} rank {} with {} wins", bot_id, rank, score.wins);
    }
}
