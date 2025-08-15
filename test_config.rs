use crates::engine::config::UnifiedConfig;

fn main() {
    println!("Testing configuration loading...");
    
    match UnifiedConfig::from_file("config/default.toml") {
        Ok(config) => {
            println!("✅ Configuration loaded successfully!");
            println!("🔧 Engine: {}x{} grid", config.engine.width, config.engine.height);
            println!("🤖 Bots: {} configured", config.bots.len());
            for (i, bot) in config.bots.iter().enumerate() {
                println!("  Bot {}: {} ({})", i, bot.name, bot.ai_type);
            }
        }
        Err(e) => {
            println!("❌ Failed to load configuration: {}", e);
        }
    }
}
