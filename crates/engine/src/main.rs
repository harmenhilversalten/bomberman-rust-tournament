use engine::{SystemInitializer, TournamentManager, UnifiedConfig};
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    println!("Starting Bomberman engine...");
    
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config/default.toml".to_string());
    println!("Loading config from: {}", config_path);
    
    let mut config = UnifiedConfig::from_file(&config_path)?;
    config = config.with_env_overrides()?;
    println!("Configuration loaded successfully");
    info!("Loaded configuration from {}", config_path);
    
    println!("Initializing system...");
    let mut initializer = SystemInitializer::new(config);
    let handle = initializer.initialize().await?;
    println!("System initialized successfully");
    info!("System initialized successfully");
    
    if handle.has_tournament() {
        println!("Running tournament mode");
        run_tournament(handle).await?;
    } else {
        println!("Running single game mode");
        run_single_game(handle).await?;
    }
    println!("Engine finished");
    Ok(())
}

async fn run_single_game(handle: engine::SystemHandle) -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = handle.into_engine();
    info!("Starting single game with {} ticks", 10);
    for tick in 0..10 {
        info!("Running tick {}", tick);
        engine.tick().await.unwrap();
        info!("Completed tick {}", tick);
        
        // Add a small delay to make the ticks visible
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
    info!("Game completed successfully");
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
