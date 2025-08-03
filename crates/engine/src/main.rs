use engine::{SystemInitializer, UnifiedConfig};
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
        engine.tick();
    }
    Ok(())
}

async fn run_tournament(handle: engine::SystemHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Placeholder tournament execution; reuse single game for now
    run_single_game(handle).await
}
