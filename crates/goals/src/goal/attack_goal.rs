//! Attack enemy goal implementation.

use super::{Action, BotId, Goal, GoalError, GoalType};
use state::{GameState, Tile};

/// Goal to attack nearby enemies strategically.
#[derive(Debug, Clone)]
pub struct AttackEnemyGoal;

impl Goal for AttackEnemyGoal {
    fn get_goal_type(&self) -> GoalType {
        GoalType::AttackEnemy
    }

    fn get_priority(&self, state: &GameState, bot_id: BotId) -> f32 {
        if let Some(enemy_pos) = self.find_nearest_enemy(state, bot_id) {
            if let Some(bot_pos) = self.get_bot_position(state, bot_id) {
                let distance = self.manhattan_distance(bot_pos, enemy_pos);
                if distance <= 4 {
                    90.0 - (distance as f32 * 10.0) // Much higher priority for close enemies
                } else if distance <= 8 {
                    50.0 - (distance as f32 * 3.0) // Medium priority for medium distance
                } else {
                    20.0 // Low priority for distant enemies
                }
            } else {
                0.0
            }
        } else {
            0.0 // No enemies to attack
        }
    }

    fn is_achievable(&self, state: &GameState, bot_id: BotId) -> bool {
        self.find_nearest_enemy(state, bot_id).is_some()
    }

    fn get_progress(&self, state: &GameState, bot_id: BotId) -> f32 {
        if let Some(enemy_pos) = self.find_nearest_enemy(state, bot_id) {
            if let Some(bot_pos) = self.get_bot_position(state, bot_id) {
                let distance = self.manhattan_distance(bot_pos, enemy_pos);
                if distance <= 2 {
                    1.0 // Close enough to attack
                } else {
                    1.0 / (1.0 + distance as f32)
                }
            } else {
                0.0
            }
        } else {
            1.0 // No enemies = goal complete
        }
    }

    fn is_completed(&self, state: &GameState, bot_id: BotId) -> bool {
        // Goal completed when no enemies nearby or we're in attack position
        if let Some(enemy_pos) = self.find_nearest_enemy(state, bot_id) {
            if let Some(bot_pos) = self.get_bot_position(state, bot_id) {
                let distance = self.manhattan_distance(bot_pos, enemy_pos);
                distance <= 2 // Close enough to place bomb
            } else {
                false
            }
        } else {
            true // No enemies
        }
    }

    fn plan(&self, state: &GameState, bot_id: BotId) -> Result<Vec<Action>, GoalError> {
        if let Some(enemy_pos) = self.find_nearest_enemy(state, bot_id) {
            if let Some(bot_pos) = self.get_bot_position(state, bot_id) {
                let distance = self.manhattan_distance(bot_pos, enemy_pos);
                if distance <= 2 {
                    // Close enough, place bomb and prepare to escape
                    Ok(vec![Action::PlaceBomb, Action::EscapeDanger])
                } else {
                    // Move closer to enemy
                    Ok(vec![Action::MoveTowards { x: enemy_pos.0, y: enemy_pos.1 }])
                }
            } else {
                Ok(vec![Action::Wait])
            }
        } else {
            Ok(vec![Action::Wait])
        }
    }
}

impl AttackEnemyGoal {
    fn find_nearest_enemy(&self, state: &GameState, bot_id: BotId) -> Option<(u16, u16)> {
        let bot_pos = self.get_bot_position(state, bot_id)?;
        let snapshot = state.grid.snapshot();
        
        let mut nearest_enemy = None;
        let mut min_distance = u16::MAX;
        
        for agent in snapshot.agents() {
            if agent.id != bot_id {
                let distance = self.manhattan_distance(bot_pos, agent.position);
                if distance < min_distance {
                    min_distance = distance;
                    nearest_enemy = Some(agent.position);
                }
            }
        }
        
        nearest_enemy
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

/// Goal to destroy soft blocks strategically.
#[derive(Debug, Clone)]
pub struct DestroyBlocksGoal;

impl Goal for DestroyBlocksGoal {
    fn get_goal_type(&self) -> GoalType {
        GoalType::DestroyBlocks
    }

    fn get_priority(&self, state: &GameState, bot_id: BotId) -> f32 {
        if let Some(_block_pos) = self.find_nearest_destructible_block(state, bot_id) {
            60.0 // Higher priority for map control and path clearing
        } else {
            0.0 // No blocks to destroy
        }
    }

    fn is_achievable(&self, state: &GameState, bot_id: BotId) -> bool {
        self.find_nearest_destructible_block(state, bot_id).is_some()
    }

    fn get_progress(&self, state: &GameState, bot_id: BotId) -> f32 {
        if let Some(block_pos) = self.find_nearest_destructible_block(state, bot_id) {
            if let Some(bot_pos) = self.get_bot_position(state, bot_id) {
                let distance = self.manhattan_distance(bot_pos, block_pos);
                if distance <= 2 {
                    1.0 // Close enough to bomb
                } else {
                    1.0 / (1.0 + distance as f32)
                }
            } else {
                0.0
            }
        } else {
            1.0 // No blocks = goal complete
        }
    }

    fn is_completed(&self, state: &GameState, bot_id: BotId) -> bool {
        // Goal completed when no destructible blocks nearby or we've cleared them
        self.find_nearest_destructible_block(state, bot_id).is_none()
    }

    fn plan(&self, state: &GameState, bot_id: BotId) -> Result<Vec<Action>, GoalError> {
        if let Some(block_pos) = self.find_nearest_destructible_block(state, bot_id) {
            if let Some(bot_pos) = self.get_bot_position(state, bot_id) {
                let distance = self.manhattan_distance(bot_pos, block_pos);
                if distance <= 2 {
                    // Close enough, place bomb and escape
                    Ok(vec![Action::PlaceBomb, Action::EscapeDanger])
                } else {
                    // Move closer to blocks
                    Ok(vec![Action::MoveTowards { x: block_pos.0, y: block_pos.1 }])
                }
            } else {
                Ok(vec![Action::Wait])
            }
        } else {
            Ok(vec![Action::Wait])
        }
    }
}

impl DestroyBlocksGoal {
    fn find_nearest_destructible_block(&self, state: &GameState, bot_id: BotId) -> Option<(u16, u16)> {
        let bot_pos = self.get_bot_position(state, bot_id)?;
        let snapshot = state.grid.snapshot();
        let tiles = snapshot.tiles();
        
        let mut nearest_block = None;
        let mut min_distance = u16::MAX;
        
        for y in 0..state.grid.height() {
            for x in 0..state.grid.width() {
                let index = y * state.grid.width() + x;
                if index < tiles.len() && tiles[index] == Tile::SoftCrate {
                    let block_pos = (x as u16, y as u16);
                    let distance = self.manhattan_distance(bot_pos, block_pos);
                    if distance < min_distance {
                        min_distance = distance;
                        nearest_block = Some(block_pos);
                    }
                }
            }
        }
        
        nearest_block
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
