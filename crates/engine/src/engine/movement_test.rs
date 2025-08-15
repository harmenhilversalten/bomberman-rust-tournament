//! Regression test for bot movement position updates.

use std::sync::{Arc, RwLock};
use events::bus::EventBus;
use events::events::{BotDecision, BotEvent, Event};
use events::queue::EventPriority;
use state::GameGrid;
use super::Engine;
use crate::config::EngineConfig;
use common::Direction;

#[tokio::test]
async fn test_bot_movement_updates_position() {
    // Setup engine with minimal config
    let config = EngineConfig {
        width: 10,
        height: 10,
        ..EngineConfig::default()
    };
    let (mut engine, _delta_rx, events) = Engine::new(config);
    
    // Spawn a bot at position (3,3) - this should be a clear spawn area
    let bot_config = bot::BotConfig::new("test_bot", bot::ai::AiType::Heuristic);
    let bot_id = engine.spawn_bot(bot_config).expect("Failed to spawn bot");
    
    // Wait for cooldown to expire - bots get a cooldown when spawned
    std::thread::sleep(std::time::Duration::from_millis(250));
    
    // Get initial position
    let initial_position = {
        let grid = engine.grid();
        let grid_lock = grid.read().unwrap();
        let snapshot = grid_lock.snapshot();
        snapshot.agents().iter().find(|a| a.id == bot_id).unwrap().position
    };
    

    
    // Send a movement command directly via events
    events.emit(
        Event::Bot(BotEvent::Decision {
            bot_id,
            decision: BotDecision::Move(Direction::Right),
        }),
        EventPriority::Normal,
    );
    
    // Process the tick to handle the movement
    engine.tick().await.expect("Failed to process tick");
    
    // Check that position was updated in the snapshot
    let final_position = {
        let grid = engine.grid();
        let grid_lock = grid.read().unwrap();
        let snapshot = grid_lock.snapshot();
        snapshot.agents().iter().find(|a| a.id == bot_id).unwrap().position
    };
    

    
    // Assert that the position changed
    assert_ne!(initial_position, final_position, 
               "Bot position should have changed from {:?} to {:?}", 
               initial_position, final_position);
    
    // Assert specific movement (right should increase x coordinate)
    assert_eq!(final_position.0, initial_position.0 + 1, 
               "Moving right should increase x coordinate from {} to {}", 
               initial_position.0, final_position.0);
    assert_eq!(final_position.1, initial_position.1, 
               "Moving right should not change y coordinate");
}

#[tokio::test]
async fn test_multiple_movements() {
    // Setup engine
    let config = EngineConfig {
        width: 10,
        height: 10,
        ..EngineConfig::default()
    };
    let (mut engine, _delta_rx, events) = Engine::new(config);
    
    // Spawn a bot
    let bot_config = bot::BotConfig::new("test_bot_multi", bot::ai::AiType::Heuristic);
    let bot_id = engine.spawn_bot(bot_config).expect("Failed to spawn bot");
    
    // Get initial position
    let mut current_position = {
        let grid = engine.grid();
        let grid_lock = grid.read().unwrap();
        let snapshot = grid_lock.snapshot();
        snapshot.agents().iter().find(|a| a.id == bot_id).unwrap().position
    };
    
    // Test multiple movements
    let movements = [Direction::Right, Direction::Down, Direction::Left, Direction::Up];
    
    for direction in movements {
        // Send movement command
        events.emit(
            Event::Bot(BotEvent::Decision {
                bot_id,
                decision: BotDecision::Move(direction),
            }),
            EventPriority::Normal,
        );
        
        // Process tick
        engine.tick().await.expect("Failed to process tick");
        
        // Check position updated
        let new_position = {
            let grid = engine.grid();
            let grid_lock = grid.read().unwrap();
            let snapshot = grid_lock.snapshot();
            snapshot.agents().iter().find(|a| a.id == bot_id).unwrap().position
        };
        
        println!("After moving {:?}: {:?} -> {:?}", direction, current_position, new_position);
        
        // Verify position changed (unless blocked)
        // Position should change unless we hit a boundary or obstacle
        if direction == Direction::Right && current_position.0 < 9 {
            assert_eq!(new_position.0, current_position.0 + 1);
        } else if direction == Direction::Left && current_position.0 > 0 {
            assert_eq!(new_position.0, current_position.0 - 1);
        } else if direction == Direction::Down && current_position.1 < 9 {
            assert_eq!(new_position.1, current_position.1 + 1);
        } else if direction == Direction::Up && current_position.1 > 0 {
            assert_eq!(new_position.1, current_position.1 - 1);
        }
        
        current_position = new_position;
    }
}
