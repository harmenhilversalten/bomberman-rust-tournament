use bot::AiType;
use engine::{BotConfig, Engine, EngineConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut engine, _rx, _events) = Engine::new(EngineConfig::default());
    let bot_configs = vec![
        BotConfig::new("bot1", AiType::Heuristic),
        BotConfig::new("bot2", AiType::Reactive),
    ];

    for cfg in bot_configs {
        if let Err(e) = engine.spawn_bot(cfg) {
            eprintln!("failed to spawn bot: {}", e);
        }
    }

    for _ in 0..10 {
        engine.tick();
    }

    Ok(())
}
