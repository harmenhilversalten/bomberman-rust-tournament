use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;

use super::scheduler::TaskScheduler;
use crate::{
    bot::{BotError, BotHandle, BotManager},
    config::EngineConfig,
    simulation::{DeterminismChecker, Replay, ReplayRecorder},
    systems::System,
};
use ::bot::BotConfig;

use crossbeam::channel::Receiver;
use events::{
    bus::{EventBus, EventFilter},
    events::bot_events::BotId,
    events::{BotDecision, BotEvent, Event, GameEvent},
    queue::EventPriority,
};
use log::error;
use state::{GameGrid, components::Bomb, grid::GridDelta};
use thiserror::Error;
use tokio::sync::watch;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Game grid lock poisoned: {0}")]
    GridLockPoisoned(String),
    #[error("System execution failed: {system:?}, reason: {reason}")]
    SystemExecution { system: String, reason: String },
    #[error("Event broadcast failed: {0}")]
    EventBroadcast(String),
    #[error("Bot command processing failed: {0}")]
    BotCommandProcessing(String),
}

/// Core game engine advancing the simulation and broadcasting changes.
pub struct Engine {
    config: EngineConfig,
    grid: Arc<RwLock<GameGrid>>,
    bot_manager: BotManager,
    bots: Vec<BotHandle>,
    events: Arc<EventBus>,
    delta_tx: watch::Sender<GridDelta>,
    scheduler: TaskScheduler,
    systems: Vec<Arc<Mutex<Box<dyn System>>>>,
    replay_recorder: ReplayRecorder,
    determinism_checker: DeterminismChecker,
    bot_command_rx: Receiver<Event>,
    tick: u64,
    bot_status: HashMap<BotId, String>,
    movement_cooldowns: HashMap<BotId, std::time::Instant>, // Track movement cooldowns
}

impl Engine {
    /// Creates a new engine configured via [`EngineConfig`].
    pub fn new(config: EngineConfig) -> (Self, watch::Receiver<GridDelta>, Arc<EventBus>) {
        let grid = GameGrid::new(config.width, config.height);
        let (tx, rx) = watch::channel(GridDelta::None);
        let events = Arc::new(EventBus::new());
        let filter = EventFilter::new(|e| matches!(e, Event::Bot(_)));
        let (_id, cmd_rx) = events.subscribe_with_filter(Some(filter));
        let bot_manager = BotManager::new();
        (
            Self {
                config,
                grid: Arc::new(RwLock::new(grid)),
                delta_tx: tx,
                scheduler: TaskScheduler::new(),
                systems: Vec::new(),
                replay_recorder: ReplayRecorder::new(),
                determinism_checker: DeterminismChecker::new(),
                events: Arc::clone(&events),
                bot_manager,
                bots: Vec::new(),
                bot_command_rx: cmd_rx,
                tick: 0,
                bot_status: std::collections::HashMap::new(),
                movement_cooldowns: HashMap::new(),
            },
            rx,
            events,
        )
    }

    /// Construct an engine from provided components.
    pub fn with_components(
        config: EngineConfig,
        grid: Arc<RwLock<GameGrid>>,
        events: Arc<EventBus>,
    ) -> (Self, watch::Receiver<GridDelta>) {
        let (tx, rx) = watch::channel(GridDelta::None);
        let filter = EventFilter::new(|e| matches!(e, Event::Bot(_)));
        let (_id, cmd_rx) = events.subscribe_with_filter(Some(filter));
        let bot_manager = BotManager::new();
        (
            Self {
                config,
                grid,
                delta_tx: tx,
                scheduler: TaskScheduler::new(),
                systems: Vec::new(),
                replay_recorder: ReplayRecorder::new(),
                determinism_checker: DeterminismChecker::new(),
                events,
                bot_manager,
                bots: Vec::new(),
                bot_command_rx: cmd_rx,
                tick: 0,
                bot_status: std::collections::HashMap::new(),
                movement_cooldowns: HashMap::new(),
            },
            rx,
        )
    }

    /// Advances the game by a single tick by running all registered systems.
    pub async fn tick(&mut self) -> Result<(), EngineError> {
        self.scheduler.run().await;
        self.events.process();
        
        // Send a tick event to prompt bots to make decisions
        // This ensures bots get regular opportunities to think and act
        let tick_delta = GridDelta::None; // Use None as a "thinking prompt"
        
        // Broadcast to all subscribers (including bots)
        self.events.broadcast(Event::Grid(tick_delta.clone()));
        
        // Also send via the delta channel for any other listeners
        let _ = self.delta_tx.send(tick_delta);
        
        // Process all bot events directly from the event bus
        // This ensures we get events from ALL bots, not just from a subscription
        let mut _event_count = 0;
        
        // Process any events that might be in the subscription first
        while let Ok(Event::Bot(cmd)) = self.bot_command_rx.try_recv() {
            _event_count += 1;
            match &cmd {
                BotEvent::Status { bot_id, status } => {
                    self.bot_status.insert(*bot_id, status.clone());
                }
                BotEvent::Decision { bot_id, .. } | BotEvent::Error { bot_id, .. } => {
                    if let Err(e) = self.handle_bot_command(cmd.clone()) {
                        self.events.emit(
                            Event::Bot(BotEvent::Error {
                                bot_id: *bot_id,
                                message: e.to_string(),
                            }),
                            EventPriority::Normal,
                        );
                    }
                }
            }
        }
        
        // CRITICAL: Process the event bus to ensure ALL events are delivered
        // This is the key fix - we need to process the event bus to get events from all bots
        self.events.process();
        
        // Process bot decisions from the event bus
        // The bots send their decisions via events, not via the channel
        let mut bot_events = Vec::new();
        self.events.collect_events(&mut bot_events, |event| {
            matches!(event, Event::Bot(BotEvent::Decision { .. }))
        });
        
        for event in bot_events {
            if let Event::Bot(cmd) = event {
                match &cmd {
                    BotEvent::Status { bot_id, status } => {
                        self.bot_status.insert(*bot_id, status.clone());
                    }
                    BotEvent::Decision { bot_id, .. } | BotEvent::Error { bot_id, .. } => {
                        if let Err(e) = self.handle_bot_command(cmd.clone()) {
                            self.events.emit(
                                Event::Bot(BotEvent::Error {
                                    bot_id: *bot_id,
                                    message: e.to_string(),
                                }),
                                EventPriority::Normal,
                            );
                        }
                    }
                }
            }
        }
        

        
        let grid = self
            .grid
            .read()
            .map_err(|e| EngineError::GridLockPoisoned(e.to_string()))?;
        self.determinism_checker.record(&grid);
        drop(grid);
        self.tick += 1;
        self.events
            .broadcast(Event::Game(GameEvent::TickCompleted { tick: self.tick }));
        Ok(())
    }

    fn handle_bot_command(&mut self, cmd: BotEvent) -> Result<(), BotError> {
        match cmd {
            BotEvent::Decision { bot_id, decision } => {
                println!("Processing decision for bot {}: {:?}", bot_id, decision);
                match decision {
                    BotDecision::Wait => Ok(()),
                    BotDecision::Move(direction) => {
                        // Check movement cooldown (200ms between movements)
                        let now = std::time::Instant::now();
                        let default_time = std::time::Instant::now();
                        let last_move = self.movement_cooldowns.get(&bot_id).unwrap_or(&default_time);
                        if now.duration_since(*last_move).as_millis() < 200 {
                            println!("Bot {} is in movement cooldown", bot_id);
                            return Ok(()); // Still in cooldown
                        }
                        
                        let mut grid = self.grid.write().expect("grid lock poisoned");
                        
                        // Find the agent and calculate new position
                        let mut new_position = None;
                        if let Some(agent) = grid.agents().iter().find(|a| a.id == bot_id) {
                            let (mut x, mut y) = agent.position;
                            let old_pos = (x, y);
                            println!("Bot {} current position: ({}, {})", bot_id, x, y);
                            
                            // Calculate new position
                            match direction {
                                common::Direction::Up => y = y.saturating_sub(1),
                                common::Direction::Down => y = y.saturating_add(1).min(self.config.height as u16 - 1),
                                common::Direction::Left => x = x.saturating_sub(1),
                                common::Direction::Right => x = x.saturating_add(1).min(self.config.width as u16 - 1),
                            }
                            println!("Bot {} new position: ({}, {})", bot_id, x, y);
                            
                            // Only move if position actually changed and is valid
                            if (x, y) != old_pos && self.is_position_walkable(&grid, (x, y)) {
                                println!("Bot {} position is walkable", bot_id);
                                new_position = Some((x, y));
                            } else {
                                println!("Bot {} position ({}, {}) is not walkable", bot_id, x, y);
                                if (x, y) == old_pos {
                                    println!("  Position didn't change");
                                }
                                // Let's check what's at this position
                                if x < self.config.width as u16 && y < self.config.height as u16 {
                                    let tiles = grid.tiles();
                                    let index = (y as usize) * self.config.width + (x as usize);
                                    if index < tiles.len() {
                                        println!("  Tile at ({}, {}): {:?}", x, y, tiles[index]);
                                    }
                                }
                                // Check for other agents
                                for agent in grid.agents() {
                                    if agent.position == (x, y) {
                                        println!("  Agent {} is at position ({}, {})", agent.id, x, y);
                                    }
                                }
                            }
                        }
                        
                        // Apply the movement if valid
                        if let Some(new_pos) = new_position {
                            println!("Moving bot {} to ({}, {})", bot_id, new_pos.0, new_pos.1);
                            if let Some(agent) = grid.agents_mut().iter_mut().find(|a| a.id == bot_id) {
                                agent.position = new_pos;
                                let delta = GridDelta::MoveAgent(bot_id, new_pos);
                                self.replay_recorder.record(delta.clone());
                                let _ = self.delta_tx.send(delta.clone());
                                self.events.broadcast(Event::Grid(delta));
                                
                                // Update movement cooldown
                                self.movement_cooldowns.insert(bot_id, now);
                            }
                        }
                        Ok(())
                    }
                    BotDecision::PlaceBomb => {
                        println!("Bot {} placing bomb", bot_id);
                        let mut grid = self.grid.write().expect("grid lock poisoned");
                        if let Some(agent) = grid.agents_mut().iter_mut().find(|a| a.id == bot_id) {
                            // Check if agent has bombs left
                            if agent.bombs_left == 0 {
                                println!("Bot {} has no bombs left", bot_id);
                                drop(grid);
                                return Ok(()); // Can't place bomb, no bombs left
                            }
                        
                            let position = agent.position;
                            
                            // Decrement bombs left
                            agent.bombs_left -= 1;
                            
                            // Create bomb for the state grid (for display/tracking)
                            let state_bomb = Bomb::new(bot_id, position, 3, 1);
                            let delta = GridDelta::AddBomb(state_bomb);
                            grid.apply_delta(delta.clone());
                            drop(grid);
                            
                            self.replay_recorder.record(delta.clone());
                            let _ = self.delta_tx.send(delta.clone());
                            self.events.broadcast(Event::Grid(delta));
                            
                            // Also broadcast bomb placement event for the bomb system to handle
                            self.events.broadcast(Event::bomb(events::events::BombEvent::Placed {
                                agent_id: bot_id,
                                position,
                            }));
                        }
                        Ok(())
                    }
                    }
                }
            },
            BotEvent::Error { .. } => Ok(()),
            BotEvent::Status { bot_id, status } => {
                self.bot_status.insert(bot_id, status);
                Ok(())
            }
        }
    }

    /// Spawn a bot managed by the engine.
                        agent.bombs_left -= 1;
                        
                        // Create bomb for the state grid (for display/tracking)
                        let state_bomb = Bomb::new(bot_id, position, 3, 1);
                        let delta = GridDelta::AddBomb(state_bomb);
                        grid.apply_delta(delta.clone());
                        drop(grid);
                        
                        self.replay_recorder.record(delta.clone());
                        let _ = self.delta_tx.send(delta.clone());
                        self.events.broadcast(Event::Grid(delta));
                        
                        // Also broadcast bomb placement event for the bomb system to handle
                        self.events.broadcast(Event::bomb(events::events::BombEvent::Placed {
                            agent_id: bot_id,
                            position,
                        }));
                    }
                }
            },
            BotEvent::Error { .. } => Ok(()),
            BotEvent::Status { bot_id, status } => {
                self.bot_status.insert(bot_id, status);
                Ok(())
            }
        }
    }

    /// Spawn a bot managed by the engine.
    pub fn spawn_bot(&mut self, config: BotConfig) -> Result<BotId, BotError> {
        let handle = self
            .bot_manager
            .spawn_bot(config, Arc::clone(&self.events))?;
        let id = handle.id;
        self.bots.push(handle);
        
        // Calculate spawn position based on bot ID to avoid overlapping
        // Spread 8 bots across the larger map in a grid pattern
        // Each spawn position should have a 3x3 cleared area
        let spawn_positions = [
            (3u16, 3u16),        // Top-left
            ((self.config.width / 2) as u16, 3u16),  // Top-center
            ((self.config.width - 4) as u16, 3u16),  // Top-right
            (3u16, (self.config.height / 2) as u16), // Middle-left
            ((self.config.width - 4) as u16, (self.config.height / 2) as u16), // Middle-right
            (3u16, (self.config.height - 4) as u16), // Bottom-left
            ((self.config.width / 2) as u16, (self.config.height - 4) as u16), // Bottom-center
            ((self.config.width - 4) as u16, (self.config.height - 4) as u16), // Bottom-right
        ];
        let position = spawn_positions[id % spawn_positions.len()];
        
        // Initialize movement cooldown for this bot
        self.movement_cooldowns.insert(id, std::time::Instant::now());
        
        let agent = state::components::AgentState::new(id, position);
        let delta = GridDelta::AddAgent(agent);
        self.grid.write().expect("grid lock poisoned").apply_delta(delta.clone());
        self.replay_recorder.record(delta.clone());
        let _ = self.delta_tx.send(delta.clone());
        self.events.broadcast(Event::Grid(delta));
        println!("🎯 Engine spawned bot {} at position {:?}", id, position);
        
        Ok(id)
    }

    /// Remove a bot from the engine.
    pub fn remove_bot(&mut self, bot_id: BotId) -> Result<(), BotError> {
        if let Some(pos) = self.bots.iter().position(|b| b.id == bot_id) {
            let handle = self.bots.remove(pos);
            handle.abort();
            Ok(())
        } else {
            Err(BotError::NotFound)
        }
    }

    /// Access the shared game grid.
    pub fn grid(&self) -> Arc<RwLock<GameGrid>> {
        Arc::clone(&self.grid)
    }

    /// Access the engine configuration.
    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    /// Snapshot of current bot statuses (e.g., active goal per bot).
    pub fn bot_status(&self) -> std::collections::HashMap<usize, String> {
        self.bot_status.clone()
    }

    /// Start recording a replay.
    pub fn start_replay_recording(&mut self) {
        self.replay_recorder.start();
    }
    
    /// Check if a position is walkable (not a wall or obstacle)
    fn is_position_walkable(&self, grid: &GameGrid, pos: (u16, u16)) -> bool {
        use state::Tile;
        
        // Bounds checking
        if pos.0 >= self.config.width as u16 || pos.1 >= self.config.height as u16 {
            return false;
        }
        
        // Check if there's another agent at this position
        for agent in grid.agents() {
            if agent.position == pos {
                return false;
            }
        }
        
        // Check tile type
        let tiles = grid.tiles();
        let index = (pos.1 as usize) * self.config.width + (pos.0 as usize);
        if index < tiles.len() {
            match tiles[index] {
                Tile::Empty | Tile::PowerUp => true,
                Tile::Wall | Tile::SoftCrate | Tile::Explosion => false,
            }
        } else {
            false
        }
    }

    /// Stop recording and return the replay.
    pub fn stop_replay_recording(&mut self) -> Replay {
        self.replay_recorder.stop()
    }

    /// Access determinism hashes collected each tick.
    pub fn determinism_hashes(&self) -> &[u64] {
        self.determinism_checker.hashes()
    }

    /// Apply a replay to the current grid recording hashes.
    pub fn load_replay(&mut self, replay: &Replay) {
        let mut grid = self.grid.write().expect("grid lock poisoned");
        for delta in replay.deltas() {
            grid.apply_delta(delta.clone());
            self.determinism_checker.record(&grid);
        }
    }

    /// Add a task to the internal scheduler.
    pub fn add_task<F>(&mut self, name: &str, deps: Vec<String>, parallel: bool, task: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.scheduler.add_task(name, deps, parallel, task);
    }

    /// Register a new system with the engine.
    pub fn add_system(&mut self, system: Box<dyn System>) {
        let deps = system
            .dependencies()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let parallel = system.parallelizable();
        let name = system.name().to_string();
        let sys = Arc::new(Mutex::new(system));
        let grid = Arc::clone(&self.grid);
        let tx = self.delta_tx.clone();
        let sys_clone = Arc::clone(&sys);
        let recorder = self.replay_recorder.clone();
        let events = Arc::clone(&self.events);
        self.scheduler.add_task(name, deps, parallel, move || {
            let mut s = sys_clone.lock().expect("system lock poisoned");
            if let Some(delta) = s.run(&grid, events.as_ref()) {
                let mut g = grid.write().expect("grid lock poisoned");
                g.apply_delta(delta.clone());
                recorder.record(delta.clone());
                let _ = tx.send(delta.clone());
                events.broadcast(Event::Grid(delta));
            }
        });
        self.systems.push(sys);
    }

    /// Access the current statuses of all bots.
    pub fn bot_statuses(&self) -> &std::collections::HashMap<usize, String> {
        &self.bot_status
    }
    
    /// Check if the game has ended and return the winner if applicable.
    pub fn check_game_end(&self) -> Option<usize> {
        let grid = self.grid.read().ok()?;
        let agents = grid.agents();
        
        // Game ends when only one agent remains
        if agents.len() == 1 {
            Some(agents[0].id)
        } else if agents.is_empty() {
            // All agents eliminated (tie)
            Some(usize::MAX) // Use MAX to indicate tie
        } else {
            None // Game still ongoing
        }
    }
    
    /// Get the number of remaining agents.
    pub fn remaining_agents(&self) -> usize {
        if let Ok(grid) = self.grid.read() {
            grid.agents().len()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use events::{events::BotDecision, queue::EventPriority};
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    #[tokio::test]
    async fn tick_broadcasts_system_delta() {
        use crate::{config::EngineConfig, systems::MovementSystem};

        let config = EngineConfig {
            width: 1,
            height: 1,
            ..EngineConfig::default()
        };
        let (mut engine, mut rx, _events) = Engine::new(config);
        engine.add_system(Box::new(MovementSystem::new()));
        assert_eq!(*rx.borrow(), GridDelta::None);
        engine.tick().await.unwrap();
        assert!(matches!(
            rx.borrow_and_update().clone(),
            GridDelta::SetTile { x: 0, y: 0, .. }
        ));
    }

    #[tokio::test]
    async fn tick_runs_scheduler_tasks() {
        use crate::config::EngineConfig;
        let config = EngineConfig {
            width: 1,
            height: 1,
            ..EngineConfig::default()
        };
        let (mut engine, _rx, _events) = Engine::new(config);
        let flag = Arc::new(AtomicBool::new(false));
        let flag_clone = Arc::clone(&flag);
        engine.add_task("flag", vec![], true, move || {
            flag_clone.store(true, Ordering::SeqCst);
        });
        engine.tick().await.unwrap();
        assert!(flag.load(Ordering::SeqCst));
    }

    #[test]
    fn engine_uses_config() {
        use crate::config::EngineConfig;
        let cfg = EngineConfig {
            width: 2,
            height: 3,
            tick_rate: 30,
            ..EngineConfig::default()
        };
        let (engine, _rx, _events) = Engine::new(cfg.clone());
        assert_eq!(engine.config().tick_rate, 30);
        assert_eq!(engine.config().width, 2);
        assert_eq!(engine.config().height, 3);
    }

    #[tokio::test]
    async fn tick_emits_game_event() {
        use crate::config::EngineConfig;
        let cfg = EngineConfig {
            width: 1,
            height: 1,
            ..EngineConfig::default()
        };
        let (mut engine, _rx, events) = Engine::new(cfg);
        let (_id, rx_event) = events.subscribe();
        engine.tick().await.unwrap();
        assert_eq!(
            rx_event.try_recv().unwrap(),
            Event::Game(GameEvent::TickCompleted { tick: 1 })
        );
    }

    #[tokio::test]
    async fn tick_broadcasts_grid_event() {
        use crate::{config::EngineConfig, systems::MovementSystem};
        use events::bus::EventFilter;
        let config = EngineConfig {
            width: 1,
            height: 1,
            ..EngineConfig::default()
        };
        let (mut engine, _rx, events) = Engine::new(config);
        engine.add_system(Box::new(MovementSystem::new()));
        let filter = EventFilter::new(|e| matches!(e, Event::Grid(_)));
        let (_id, rx_event) = events.subscribe_with_filter(Some(filter));
        engine.tick().await.unwrap();
        assert!(matches!(rx_event.try_recv().unwrap(), Event::Grid(_)));
    }

    #[tokio::test]
    async fn engine_processes_bot_commands() {
        use crate::config::EngineConfig;
        let cfg = EngineConfig {
            width: 1,
            height: 1,
            ..EngineConfig::default()
        };
        let (mut engine, mut rx, events) = Engine::new(cfg);
        events.emit(
            Event::Bot(BotEvent::Decision {
                bot_id: 1,
                decision: BotDecision::PlaceBomb,
            }),
            EventPriority::Normal,
        );
        engine.tick().await.unwrap();
        assert!(matches!(
            rx.borrow_and_update().clone(),
            GridDelta::AddBomb(_)
        ));
    }

    #[tokio::test]
    async fn bomb_system_emits_event() {
        use crate::{config::EngineConfig, systems::BombSystem};
        let cfg = EngineConfig {
            width: 1,
            height: 1,
            ..EngineConfig::default()
        };
        let (mut engine, _rx, events) = Engine::new(cfg);
        engine.add_system(Box::new(BombSystem::new()));
        let (_id, rx_event) = events.subscribe();
        engine.tick().await.unwrap();
        // Ensure some event was emitted
        assert!(rx_event.try_recv().is_ok());
    }
}
