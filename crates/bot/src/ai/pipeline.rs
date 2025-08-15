use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use goals::{GoalManager, GoalScorer, GoalPlanner, PlanningStrategy, Action};
use influence::map::InfluenceMap;
use path::{Pathfinder, Point};
use state::{GameState, grid::GridDelta, Tile, AgentState, Bomb};

use crate::bot::decision::DecisionMaker;
use events::events::BotDecision;

/// Pipeline coordinating goal generation, pathfinding and influence queries.
pub struct AIDecisionPipeline {
    goal_manager: Arc<GoalManager>,
    planner: GoalPlanner,
    pathfinder: Arc<Mutex<Pathfinder>>,
    influence_map: Arc<Mutex<InfluenceMap>>,
    scorer: GoalScorer,
    bot_id: Option<usize>,
    current_position: Option<(u16, u16)>,
    grid_width: usize,
    grid_height: usize,
    tiles: Vec<Tile>,
    agents: HashMap<usize, AgentState>,
    bombs: Vec<Bomb>,
    last_bomb_time: std::time::Instant,
    last_move_time: std::time::Instant,
    tick_counter: u64,
}

impl AIDecisionPipeline {
    pub fn new(
        goal_manager: Arc<GoalManager>,
        pathfinder: Arc<Mutex<Pathfinder>>,
        influence_map: Arc<Mutex<InfluenceMap>>,
    ) -> Self {
        let grid_width = 41;  // Updated to match config
        let grid_height = 37; // Updated to match config
        
        Self {
            goal_manager,
            pathfinder,
            influence_map,
            scorer: GoalScorer::new(),
            planner: GoalPlanner::new(PlanningStrategy::HighestScore),
            bot_id: None,
            current_position: None,
            grid_width,
            grid_height,
            tiles: vec![Tile::Empty; grid_width * grid_height],
            agents: HashMap::new(),
            bombs: Vec::new(),
            last_bomb_time: std::time::Instant::now(),
            last_move_time: std::time::Instant::now(),
            tick_counter: 0,
        }
    }

    /// Process a grid delta to update internal state
    pub fn process_delta(&mut self, delta: &GridDelta) {
        match delta {
            GridDelta::SetTile { x, y, tile } => {
                let index = y * self.grid_width + x;
                if index < self.tiles.len() {
                    self.tiles[index] = *tile;
                    
                    // If this is an explosion tile, remove any bombs at this position
                    if matches!(tile, Tile::Explosion) {
                        self.remove_bomb_at_position((*x as u16, *y as u16));
                    }
                }
            }
            GridDelta::AddAgent(agent) => {
                self.agents.insert(agent.id, agent.clone());
                if let Some(bot_id) = self.bot_id {
                    if agent.id == bot_id {
                        self.current_position = Some(agent.position);
                    }
                }
            }
            GridDelta::MoveAgent(agent_id, new_pos) => {
                if let Some(agent) = self.agents.get_mut(agent_id) {
                    agent.position = *new_pos;
                }
                if let Some(bot_id) = self.bot_id {
                    if *agent_id == bot_id {
                        self.current_position = Some(*new_pos);
                    }
                }
            }
            GridDelta::RemoveAgent(agent_id) => {
                self.agents.remove(agent_id);
                if let Some(bot_id) = self.bot_id {
                    if *agent_id == bot_id {
                        self.current_position = None;
                    }
                }
            }
            GridDelta::AddBomb(bomb) => {
                self.bombs.push(bomb.clone());
                // Update influence map with new bomb
                self.update_influence_map_with_bombs();
            }
            GridDelta::None => {}
        }
    }

    /// Update influence map with current bombs
    fn update_influence_map_with_bombs(&mut self) {
        if let Ok(mut influence_guard) = self.influence_map.lock() {
            // Clear existing danger sources by creating a new map
            let mut new_map = influence::map::InfluenceMap::new(
                influence_guard.width(),
                influence_guard.height(),
            );
            
            // Add danger sources for all active bombs
            for bomb in &self.bombs {
                let danger_source = influence::core::DangerSource {
                    x: bomb.position.0,
                    y: bomb.position.1,
                    strength: 1.0,
                    range: bomb.power as u16,
                };
                new_map.add_danger_source(danger_source);
            }
            
            // Update the map
            let _ = new_map.update(&self.build_game_state());
            
            // Replace the old map with the new one
            *influence_guard = new_map;
        }
    }

    /// Remove bomb at specific position
    fn remove_bomb_at_position(&mut self, position: (u16, u16)) {
        self.bombs.retain(|bomb| bomb.position != position);
        // Update influence map after removing bomb
        self.update_influence_map_with_bombs();
    }

    /// Build game state from internal representation
    fn build_game_state(&self) -> GameState {
        GameState::new(self.grid_width, self.grid_height)
    }

    /// Check if position is walkable
    fn is_position_walkable(&self, pos: (u16, u16)) -> bool {
        if pos.0 >= self.grid_width as u16 || pos.1 >= self.grid_height as u16 {
            return false;
        }
        
        let index = pos.1 as usize * self.grid_width + pos.0 as usize;
        if index >= self.tiles.len() {
            return false;
        }
        
        matches!(self.tiles[index], Tile::Empty | Tile::Explosion)
    }

    /// Check if position is safe from bombs
    fn is_position_safe(&self, pos: (u16, u16)) -> bool {
        // First check if position is walkable
        if !self.is_position_walkable(pos) {
            return false;
        }
        
        // Check if position is safe from bomb explosions using influence map
        if let Ok(influence_guard) = self.influence_map.lock() {
            let danger_value = influence_guard.danger_at(pos.0, pos.1).unwrap_or(0.0);
            if danger_value > 0.0 {
                return false; // Position is in danger zone
            }
        }
        
        // Also check direct bomb proximity as backup
        for bomb in &self.bombs {
            let distance = self.manhattan_distance(pos, bomb.position);
            if distance <= bomb.power.into() && bomb.timer <= 2 {
                return false; // Too close to exploding bomb
            }
        }
        
        true
    }

    /// Check if bot is currently in danger
    fn is_in_danger(&self, pos: (u16, u16)) -> bool {
        // Check influence map for danger
        if let Ok(influence_guard) = self.influence_map.lock() {
            let danger_value = influence_guard.danger_at(pos.0, pos.1).unwrap_or(0.0);
            if danger_value > 0.0 {
                return true; // Position is in danger zone
            }
        }
        
        // Also check for bombs that might explode soon as backup
        for bomb in &self.bombs {
            let distance = ((pos.0 as i32 - bomb.position.0 as i32).abs() + 
                          (pos.1 as i32 - bomb.position.1 as i32).abs()) as u16;
            
            // If we're within bomb range and it might explode soon
            if distance <= bomb.power.into() && bomb.timer <= 2 {
                return true;
            }
        }
        false
    }

    /// Find escape direction when in danger
    fn escape_danger(&self, current_pos: (u16, u16)) -> BotDecision {
        use common::Direction;
        
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        let mut safe_directions = Vec::new();
        let mut best_direction = None;
        let mut lowest_danger = f32::INFINITY;
        
        // Find all safe directions and the one with lowest danger
        for direction in &directions {
            if let Some(new_pos) = self.calculate_new_position(current_pos, *direction) {
                if self.is_position_walkable(new_pos) {
                    // Check danger using influence map
                    if let Ok(influence_guard) = self.influence_map.lock() {
                        let danger_value = influence_guard.danger_at(new_pos.0, new_pos.1).unwrap_or(0.0);
                        if danger_value <= 0.0 {
                            safe_directions.push(*direction);
                            if danger_value < lowest_danger {
                                lowest_danger = danger_value;
                                best_direction = Some(*direction);
                            }
                        }
                    } else {
                        // Fallback to direct safety check
                        if self.is_position_safe(new_pos) {
                            safe_directions.push(*direction);
                        }
                    }
                }
            }
        }
        
        // If we found a direction with lower danger, use it
        if let Some(direction) = best_direction {
            return BotDecision::Move(direction);
        }
        
        // If we have any safe directions, use the first one
        if let Some(direction) = safe_directions.first() {
            return BotDecision::Move(*direction);
        }
        
        // If no safe direction found, try to move away from the nearest bomb
        if let Some(nearest_bomb) = self.find_nearest_bomb(current_pos) {
            return self.move_away_from_position(current_pos, nearest_bomb.position);
        }
        
        BotDecision::Wait
    }

    /// Calculate new position based on direction
    fn calculate_new_position(&self, pos: (u16, u16), direction: common::Direction) -> Option<(u16, u16)> {
        match direction {
            common::Direction::Up => {
                if pos.1 > 0 { Some((pos.0, pos.1 - 1)) } else { None }
            }
            common::Direction::Down => {
                if pos.1 < self.grid_height as u16 - 1 { Some((pos.0, pos.1 + 1)) } else { None }
            }
            common::Direction::Left => {
                if pos.0 > 0 { Some((pos.0 - 1, pos.1)) } else { None }
            }
            common::Direction::Right => {
                if pos.0 < self.grid_width as u16 - 1 { Some((pos.0 + 1, pos.1)) } else { None }
            }
        }
    }

    /// Find nearest bomb
    fn find_nearest_bomb(&self, pos: (u16, u16)) -> Option<&Bomb> {
        self.bombs.iter().min_by_key(|bomb| {
            self.manhattan_distance(pos, bomb.position)
        })
    }

    /// Move away from a position
    fn move_away_from_position(&self, current_pos: (u16, u16), target_pos: (u16, u16)) -> BotDecision {
        use common::Direction;
        
        let dx = current_pos.0 as i32 - target_pos.0 as i32;
        let dy = current_pos.1 as i32 - target_pos.1 as i32;
        
        // Choose direction with larger difference
        if dx.abs() > dy.abs() {
            if dx > 0 && current_pos.0 < self.grid_width as u16 - 1 {
                BotDecision::Move(Direction::Right)
            } else if dx < 0 && current_pos.0 > 0 {
                BotDecision::Move(Direction::Left)
            } else if dy > 0 && current_pos.1 < self.grid_height as u16 - 1 {
                BotDecision::Move(Direction::Down)
            } else if dy < 0 && current_pos.1 > 0 {
                BotDecision::Move(Direction::Up)
            } else {
                BotDecision::Wait
            }
        } else {
            if dy > 0 && current_pos.1 < self.grid_height as u16 - 1 {
                BotDecision::Move(Direction::Down)
            } else if dy < 0 && current_pos.1 > 0 {
                BotDecision::Move(Direction::Up)
            } else if dx > 0 && current_pos.0 < self.grid_width as u16 - 1 {
                BotDecision::Move(Direction::Right)
            } else if dx < 0 && current_pos.0 > 0 {
                BotDecision::Move(Direction::Left)
            } else {
                BotDecision::Wait
            }
        }
    }

    /// Calculate Manhattan distance between two positions
    fn manhattan_distance(&self, pos1: (u16, u16), pos2: (u16, u16)) -> u16 {
        ((pos1.0 as i32 - pos2.0 as i32).abs() + (pos1.1 as i32 - pos2.1 as i32).abs()) as u16
    }

    /// Make goal-based decision using the goal system
    fn make_goal_based_decision(&mut self, game_state: &GameState, bot_id: usize) -> BotDecision {
        // Generate goals for current situation
        let goals = self.goal_manager.generate_goals(game_state);
        
        // For now, use the first goal directly
        if !goals.is_empty() {
            let goal = &goals[0];
            // Activate the goal and get action
            if let Ok(()) = self.planner.activate_goal(goal.clone(), game_state, bot_id, self.tick_counter) {
                if let Ok(actions) = self.planner.execute_active_goal(game_state, bot_id) {
                    if !actions.is_empty() {
                        return self.convert_action_to_decision(&actions[0], game_state, bot_id);
                    }
                }
            }
        }
        
        BotDecision::Wait
    }

    /// Convert goal Action to BotDecision
    fn convert_action_to_decision(&mut self, action: &Action, _game_state: &GameState, _bot_id: usize) -> BotDecision {
        match action {
            Action::Wait => BotDecision::Wait,
            Action::Move(direction) => {
                // Update movement cooldown when we decide to move
                self.last_move_time = std::time::Instant::now();
                BotDecision::Move(*direction)
            },
            Action::PlaceBomb => {
                self.last_bomb_time = std::time::Instant::now();
                BotDecision::PlaceBomb
            },
            Action::MoveTowards { x, y } => {
                // Use pathfinding to determine direction
                if let Some(current_pos) = self.current_position {
                    let target = Point::new(*x as i32, *y as i32);
                    let start = Point::new(current_pos.0 as i32, current_pos.1 as i32);
                    
                    if let Ok(mut pathfinder_guard) = self.pathfinder.lock() {
                        // Create a simple influence data for pathfinding
                        let mut influence_map = influence::map::InfluenceMap::new(self.grid_width as u16, self.grid_height as u16);
                        let _ = influence_map.update(&self.build_game_state());
                        let influence_data = influence_map.data();
                        if let Some(path) = pathfinder_guard.find_path(start, target, &influence_data) {
                            // Get the first step from the path
                            if let Some(first_step) = path.nodes.first() {
                                let direction = self.direction_from_points(current_pos, (first_step.position.x as u16, first_step.position.y as u16));
                                self.last_move_time = std::time::Instant::now();
                                return BotDecision::Move(direction);
                            }
                        }
                    }
                }
                BotDecision::Wait
            },
            Action::EscapeDanger => {
                if let Some(current_pos) = self.current_position {
                    return self.escape_danger(current_pos);
                }
                BotDecision::Wait
            }
        }
    }

    /// Determine direction between two points
    fn direction_from_points(&self, from: (u16, u16), to: (u16, u16)) -> common::Direction {
        if to.1 < from.1 {
            common::Direction::Up
        } else if to.1 > from.1 {
            common::Direction::Down
        } else if to.0 < from.0 {
            common::Direction::Left
        } else {
            common::Direction::Right
        }
    }

    /// Simple fallback random movement when goal system doesn't provide decisions
    fn fallback_random_decision(&mut self) -> BotDecision {
        use common::Direction;
        use rand::seq::SliceRandom;
        use rand::{thread_rng, Rng};
        
        // Check movement cooldown - allow movement more frequently
        if self.last_move_time.elapsed().as_millis() < 100 { // 100ms = 3-4 ticks at 30fps
            return BotDecision::Wait;
        }
        
        let mut rng = thread_rng();
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        
        // More aggressive movement: 70% chance to move, 20% chance to wait, 10% chance to place bomb
        let action_choice: f32 = rng.gen();
        
        if action_choice < 0.7 {
            // Move in random direction
            if let Some(direction) = directions.choose(&mut rng) {
                if let Some(current_pos) = self.current_position {
                    if let Some(new_pos) = self.calculate_new_position(current_pos, *direction) {
                        if self.is_position_safe(new_pos) {
                            self.last_move_time = std::time::Instant::now();
                            return BotDecision::Move(*direction);
                        }
                    }
                }
            }
        } else if action_choice < 0.9 {
            // Place bomb if we haven't recently
            if self.last_bomb_time.elapsed().as_millis() > 500 {
                self.last_bomb_time = std::time::Instant::now();
                return BotDecision::PlaceBomb;
            }
        }
        
        BotDecision::Wait
    }
}

impl DecisionMaker<GridDelta, BotDecision> for AIDecisionPipeline {
    fn decide(&mut self, delta: GridDelta) -> BotDecision {
        // Update internal state
        self.tick_counter += 1;
        
        // Process the delta to update our internal state
        self.process_delta(&delta);
        
        // Get bot ID from the delta if we don't have one yet
        if self.bot_id.is_none() {
            if let GridDelta::AddAgent(ref agent) = delta {
                self.bot_id = Some(agent.id);
                self.current_position = Some(agent.position);
            }
        }
        
        let bot_id = match self.bot_id {
            Some(id) => id,
            None => return BotDecision::Wait,
        };
        
        // Build game state from internal representation
        let game_state = self.build_game_state();
        
        // Use goal-based planning for intelligent decisions
        let decision = self.make_goal_based_decision(&game_state, bot_id);
        
        // Fallback: if goal system returns Wait, try simple random movement
        let final_decision = if matches!(decision, BotDecision::Wait) {
            let fallback = self.fallback_random_decision();
            // Debug: Log when fallback is used
            if matches!(fallback, BotDecision::Move(_)) {
                println!("🤖 Bot {} using fallback movement: {:?}", bot_id, fallback);
            }
            fallback
        } else {
            // Debug: Log when goal-based decision is used
            if matches!(decision, BotDecision::Move(_)) {
                println!("🎯 Bot {} using goal-based movement: {:?}", bot_id, decision);
            }
            decision
        };
        
        final_decision
    }

    fn status(&self) -> Option<String> {
        if let Some(active) = &self.planner.active_goal {
            Some(format!("{:?}", active.goal.get_goal_type()))
        } else {
            None
        }
    }
}
