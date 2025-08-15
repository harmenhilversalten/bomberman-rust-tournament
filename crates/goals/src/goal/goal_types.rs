//! Built-in goal implementations.

use super::{Action, BotId, Goal, GoalError, GoalType};
use state::{GameState, Tile};


/// Goal to collect a nearby power-up.
#[derive(Debug, Clone)]
pub struct CollectPowerUpGoal;

impl Goal for CollectPowerUpGoal {
    fn get_goal_type(&self) -> GoalType {
        GoalType::CollectPowerUp
    }

    fn get_priority(&self, state: &GameState, bot_id: BotId) -> f32 {
        // Higher priority if power-ups are nearby
        if let Some(_powerup_pos) = self.find_nearest_powerup(state, bot_id) {
            50.0 // High priority for collecting power-ups
        } else {
            0.0 // No power-ups available
        }
    }

    fn is_achievable(&self, state: &GameState, bot_id: BotId) -> bool {
        // Achievable if there are any power-ups on the map
        self.find_nearest_powerup(state, bot_id).is_some()
    }

    fn get_progress(&self, state: &GameState, bot_id: BotId) -> f32 {
        // Progress based on distance to nearest power-up
        if let Some(powerup_pos) = self.find_nearest_powerup(state, bot_id) {
            if let Some(bot_pos) = self.get_bot_position(state, bot_id) {
                let distance = self.manhattan_distance(bot_pos, powerup_pos);
                1.0 / (1.0 + distance as f32) // Closer = higher progress
            } else {
                0.0
            }
        } else {
            1.0 // No power-ups means goal is "complete"
        }
    }

    fn is_completed(&self, state: &GameState, bot_id: BotId) -> bool {
        // Goal is completed when there are no more power-ups or bot collected one
        self.find_nearest_powerup(state, bot_id).is_none()
    }

    fn plan(&self, state: &GameState, bot_id: BotId) -> Result<Vec<Action>, GoalError> {
        if let Some(powerup_pos) = self.find_nearest_powerup(state, bot_id) {
            Ok(vec![Action::MoveTowards { x: powerup_pos.0, y: powerup_pos.1 }])
        } else {
        Ok(vec![Action::Wait])
        }
    }
}

impl CollectPowerUpGoal {
    fn find_nearest_powerup(&self, state: &GameState, bot_id: BotId) -> Option<(u16, u16)> {
        let bot_pos = self.get_bot_position(state, bot_id)?;
        let snapshot = state.grid.snapshot();
        let tiles = snapshot.tiles();
        
        let mut nearest_powerup = None;
        let mut min_distance = u16::MAX;
        
        for y in 0..state.grid.height() {
            for x in 0..state.grid.width() {
                let index = y * state.grid.width() + x;
                if index < tiles.len() && tiles[index] == Tile::PowerUp {
                    let powerup_pos = (x as u16, y as u16);
                    let distance = self.manhattan_distance(bot_pos, powerup_pos);
                    if distance < min_distance {
                        min_distance = distance;
                        nearest_powerup = Some(powerup_pos);
                    }
                }
            }
        }
        
        nearest_powerup
    }
    
    fn get_bot_position(&self, state: &GameState, bot_id: BotId) -> Option<(u16, u16)> {
        let snapshot = state.grid.snapshot();
        snapshot.agents().iter()
            .find(|agent| agent.id == bot_id)
            .map(|agent| agent.position)
    }
    
    fn manhattan_distance(&self, pos1: (u16, u16), pos2: (u16, u16)) -> u16 {
        ((pos1.0 as i32 - pos2.0 as i32).abs() + (pos1.1 as i32 - pos2.1 as i32).abs()) as u16
    }
}

/// Goal to move away from danger.
#[derive(Debug, Clone)]
pub struct AvoidDangerGoal;

impl Goal for AvoidDangerGoal {
    fn get_goal_type(&self) -> GoalType {
        GoalType::AvoidDanger
    }

    fn get_priority(&self, state: &GameState, bot_id: BotId) -> f32 {
        // Very high priority if in immediate danger
        if self.is_in_immediate_danger(state, bot_id) {
            100.0 // Highest priority - survival
        } else if self.is_near_danger(state, bot_id) {
            75.0 // High priority - preventive escape
        } else {
            0.0 // No danger, no priority
        }
    }

    fn is_achievable(&self, state: &GameState, bot_id: BotId) -> bool {
        // Always achievable if we can move
        self.can_move_somewhere(state, bot_id)
    }

    fn get_progress(&self, state: &GameState, bot_id: BotId) -> f32 {
        // Progress based on distance from danger
        if self.is_in_immediate_danger(state, bot_id) {
            0.0 // No progress if still in danger
        } else if self.is_near_danger(state, bot_id) {
            0.5 // Partial progress if still near danger
        } else {
            1.0 // Full progress if safe
        }
    }

    fn is_completed(&self, state: &GameState, bot_id: BotId) -> bool {
        // Completed when not in danger
        !self.is_in_immediate_danger(state, bot_id) && !self.is_near_danger(state, bot_id)
    }

    fn plan(&self, state: &GameState, bot_id: BotId) -> Result<Vec<Action>, GoalError> {
        if self.is_in_immediate_danger(state, bot_id) {
            Ok(vec![Action::EscapeDanger])
        } else if self.is_near_danger(state, bot_id) {
            // Find a safer position
            if let Some(safe_pos) = self.find_safest_position(state, bot_id) {
                Ok(vec![Action::MoveTowards { x: safe_pos.0, y: safe_pos.1 }])
            } else {
                Ok(vec![Action::EscapeDanger])
            }
        } else {
            Ok(vec![Action::Wait])
        }
    }
}

impl AvoidDangerGoal {
    fn is_in_immediate_danger(&self, state: &GameState, bot_id: BotId) -> bool {
        if let Some(bot_pos) = self.get_bot_position(state, bot_id) {
            let snapshot = state.grid.snapshot();
            for bomb in snapshot.bombs() {
                let distance = self.manhattan_distance(bot_pos, bomb.position);
                if distance <= bomb.power.into() && bomb.timer <= 2 {
                    return true;
                }
            }
        }
        false
    }
    
    fn is_near_danger(&self, state: &GameState, bot_id: BotId) -> bool {
        if let Some(bot_pos) = self.get_bot_position(state, bot_id) {
            let snapshot = state.grid.snapshot();
            for bomb in snapshot.bombs() {
                let distance = self.manhattan_distance(bot_pos, bomb.position);
                if distance <= (bomb.power as u16) + 1 {
                    return true;
                }
            }
        }
        false
    }
    
    fn can_move_somewhere(&self, state: &GameState, bot_id: BotId) -> bool {
        if let Some(bot_pos) = self.get_bot_position(state, bot_id) {
            let directions = [
                (0i32, -1i32), (0, 1), (-1, 0), (1, 0) // Up, Down, Left, Right
            ];
            
            for (dx, dy) in &directions {
                let new_x = bot_pos.0 as i32 + dx;
                let new_y = bot_pos.1 as i32 + dy;
                
                if new_x >= 0 && new_y >= 0 {
                    let new_pos = (new_x as u16, new_y as u16);
                    if self.is_position_walkable(state, new_pos) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn find_safest_position(&self, state: &GameState, bot_id: BotId) -> Option<(u16, u16)> {
        if let Some(bot_pos) = self.get_bot_position(state, bot_id) {
            let mut safest_pos = None;
            let mut max_safety_score = f32::NEG_INFINITY;
            
            // Check positions within a reasonable radius
            for dy in -3i32..=3 {
                for dx in -3i32..=3 {
                    let new_x = bot_pos.0 as i32 + dx;
                    let new_y = bot_pos.1 as i32 + dy;
                    
                    if new_x >= 0 && new_y >= 0 {
                        let check_pos = (new_x as u16, new_y as u16);
                        if self.is_position_walkable(state, check_pos) {
                            let safety_score = self.calculate_safety_score(state, check_pos);
                            if safety_score > max_safety_score {
                                max_safety_score = safety_score;
                                safest_pos = Some(check_pos);
                            }
                        }
                    }
                }
            }
            
            safest_pos
        } else {
            None
        }
    }
    
    fn calculate_safety_score(&self, state: &GameState, pos: (u16, u16)) -> f32 {
        let snapshot = state.grid.snapshot();
        let mut score = 100.0; // Base safety score
        
        // Penalize based on distance to bombs
        for bomb in snapshot.bombs() {
            let distance = self.manhattan_distance(pos, bomb.position);
            if distance <= bomb.power.into() {
                score -= 50.0; // Dangerous position
            } else {
                score += (distance as f32) * 2.0; // Further from bombs is safer
            }
        }
        
        score
    }
    
    fn is_position_walkable(&self, state: &GameState, pos: (u16, u16)) -> bool {
        let snapshot = state.grid.snapshot();
        let tiles = snapshot.tiles();
        
        if pos.0 >= state.grid.width() as u16 || pos.1 >= state.grid.height() as u16 {
            return false;
        }
        
        let index = (pos.1 as usize) * state.grid.width() + (pos.0 as usize);
        if index >= tiles.len() {
            return false;
        }
        
        match tiles[index] {
            Tile::Empty | Tile::PowerUp => true,
            Tile::Wall | Tile::SoftCrate | Tile::Explosion => false,
        }
    }
    
    fn get_bot_position(&self, state: &GameState, bot_id: BotId) -> Option<(u16, u16)> {
        let snapshot = state.grid.snapshot();
        snapshot.agents().iter()
            .find(|agent| agent.id == bot_id)
            .map(|agent| agent.position)
    }
    
    fn manhattan_distance(&self, pos1: (u16, u16), pos2: (u16, u16)) -> u16 {
        ((pos1.0 as i32 - pos2.0 as i32).abs() + (pos1.1 as i32 - pos2.1 as i32).abs()) as u16
    }
}
