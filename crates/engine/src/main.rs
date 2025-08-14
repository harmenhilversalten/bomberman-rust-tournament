use engine::{SystemInitializer, TournamentManager, UnifiedConfig, display::GameDisplay};
use log::info;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    println!("🎮 Starting Bomberman Tournament Engine...");
    
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config/default.toml".to_string());
    println!("📁 Loading config from: {}", config_path);
    
    let mut config = UnifiedConfig::from_file(&config_path)?;
    config = config.with_env_overrides()?;
    println!("✅ Configuration loaded successfully");
    info!("Loaded configuration from {}", config_path);
    
    println!("🔧 Initializing system...");
    let mut initializer = SystemInitializer::new(config.clone());
    let handle = initializer.initialize().await?;
    println!("✅ System initialized successfully");
    info!("System initialized successfully");
    
    if handle.has_tournament() {
        println!("🏆 Running tournament mode");
        run_tournament(handle).await?;
    } else {
        println!("🎯 Running interactive game mode");
        run_interactive_game(handle, config.engine.width, config.engine.height).await?;
    }
    println!("🏁 Engine finished");
    Ok(())
}

async fn run_interactive_game(
    handle: engine::SystemHandle, 
    width: usize, 
    height: usize
) -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = handle.into_engine();
    let display = GameDisplay::new(width, height);
    
    // Initialize terminal
    display.init_terminal()?;
    
    // Get reference to the actual game grid from the engine
    let grid = engine.grid();
    
    println!("🎮 Starting interactive Bomberman game!");
    println!("Controls: SPACE=pause/resume, R=restart, Q=quit");
    println!("Press any key to start...");
    
    // Wait for user input to start
    wait_for_keypress().await?;
    
    let game_running = true;
    let mut paused = false;
    let mut tick_count = 0;
    
    // Game loop
    while game_running {
        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        break;
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        tick_count = 0;
                        paused = false;
                        info!("Game restarted");
                    }
                    KeyCode::Char(' ') => {
                        paused = !paused;
                        info!("Game {}", if paused { "paused" } else { "resumed" });
                    }
                    _ => {}
                }
            }
        }
        
        // Update game state if not paused
        if !paused {
            // Run game tick
            engine.tick().await?;
            tick_count += 1;
            
            // Update display with the actual game grid
            display.render(&grid)?;
            
            // Add delay for visibility
            tokio::time::sleep(Duration::from_millis(200)).await;
            
            // Stop after reasonable number of ticks for demo
            if tick_count >= 100 {
                info!("Demo completed after {} ticks", tick_count);
                break;
            }
        } else {
            // Still render when paused
            display.render(&grid)?;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
    
    // Restore terminal
    display.restore_terminal()?;
    println!("\n🎮 Thanks for playing Bomberman Tournament!");
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

async fn wait_for_keypress() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    Ok(())
}

