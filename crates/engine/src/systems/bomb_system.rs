use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use bombs::{BombManager, BombId, Bomb as BombsCrateBomb};
use events::{
    bus::EventBus,
    events::{BombEvent, Event},
};
use state::grid::{GameGrid, GridDelta, Tile};

use super::System;

/// Manages bombs using logic from the `bombs` crate.
/// This system acts as a bridge between the engine and the bombs crate,
/// ensuring all bomb logic resides in the bombs crate as intended.
pub struct BombSystem {
    bomb_manager: BombManager,
    explosion_timers: HashMap<(usize, usize), u8>, // Position -> ticks remaining
}

impl BombSystem {
    /// Create a new [`BombSystem`].
    pub fn new() -> Self {
        Self {
            bomb_manager: BombManager::new(),
            explosion_timers: HashMap::new(),
        }
    }

    /// Updates the explosion timers for all currently exploding bombs.
    /// This is necessary to manage the animation of explosions.
    fn update_explosion_timers(&mut self, grid: &Arc<RwLock<GameGrid>>) {
        let mut grid_lock = grid.write().unwrap();
        let mut to_remove = Vec::new();

        for (pos, timer) in self.explosion_timers.iter_mut() {
            if *timer == 0 {
                // This explosion has finished its animation
                let delta = GridDelta::SetTile {
                    x: pos.0 as usize,
                    y: pos.1 as usize,
                    tile: Tile::Empty,
                };
                grid_lock.apply_delta(delta);
                to_remove.push(*pos);
            } else {
                // Decrement timer for all explosions
                *timer -= 1;
            }
        }

        for pos in to_remove {
            self.explosion_timers.remove(&pos);
        }
        drop(grid_lock);
    }
}

impl System for BombSystem {
    fn name(&self) -> &str {
        "bomb"
    }

    fn run(&mut self, grid: &Arc<RwLock<GameGrid>>, events: &EventBus) -> Option<GridDelta> {
        // First, update explosion timers and clear expired ones
        self.update_explosion_timers(grid);
        
        // Listen for bomb placement events
        // TODO: Implement proper event subscription mechanism
        
        // For now, sync grid bombs to bomb manager and process timers
        let exploding_bombs = {
            let mut grid_lock = grid.write().unwrap();
            let mut exploding = Vec::new();
            
            // Tick all bombs and collect those that should explode
            for bomb in grid_lock.bombs_mut() {
                bomb.tick();
                if bomb.is_exploding() {
                    exploding.push(bomb.clone());
                }
            }
            
            // Don't remove exploding bombs yet - we need them for explosion calculation
            exploding
        };
        
        if exploding_bombs.is_empty() {
            return None;
        }
        
        // Use bombs crate to calculate explosions
        let grid_lock = grid.read().unwrap();
        let grid_size = (grid_lock.width() as u16, grid_lock.height() as u16);
        
        // Build walls set for explosion calculation
        let mut obstacles = std::collections::HashSet::new();
        for y in 0..grid_lock.height() {
            for x in 0..grid_lock.width() {
                if let Some(tile) = grid_lock.tile(x, y) {
                    use state::Tile;
                    match tile {
                        Tile::Wall | Tile::SoftCrate => {
                            obstacles.insert((x as u16, y as u16));
                        }
                        _ => {}
                    }
                }
            }
        }
        drop(grid_lock);
        
        // Calculate explosions using the bombs crate
        let mut all_affected_positions = Vec::new();
        let mut bombs_to_remove = Vec::new();
        
        for bomb in &exploding_bombs {
            // Convert state::Bomb to bombs crate format and add to manager for calculation
            let bomb_id = BombId(bomb.owner as u32);
            let bombs_crate_bomb = BombsCrateBomb::new(
                bomb_id,
                bomb.owner,
                bomb.position,
                0, // Timer is 0 since it's exploding
                bomb.power,
            );
            self.bomb_manager.add_bomb(bombs_crate_bomb);
            
            // Calculate explosion using bombs crate
            match self.bomb_manager.calculate_explosion(bomb_id, grid_size, &obstacles) {
                Ok(explosion) => {
                    // Broadcast explosion event
                    events.broadcast(Event::bomb(BombEvent::Exploded {
                        position: bomb.position,
                        radius: bomb.power as u32,
                    }));
                    
                    all_affected_positions.extend(explosion.affected_cells);
                    bombs_to_remove.push(bomb.position);
                }
                Err(e) => {
                    eprintln!("Explosion calculation failed: {:?}", e);
                }
            }
        }
        
        // Apply explosion effects to the grid
        if !all_affected_positions.is_empty() {
            let mut grid_lock = grid.write().unwrap();
            
            // Track which agents had bombs explode to restore their bomb count
            let mut agents_to_restore_bombs = std::collections::HashSet::new();
            
            for pos in &all_affected_positions {
                // Create explosion tile
                let delta = GridDelta::SetTile {
                    x: pos.0 as usize,
                    y: pos.1 as usize,
                    tile: Tile::Explosion,
                };
                grid_lock.apply_delta(delta);
                
                // Set explosion timer (3 ticks for animation)
                self.explosion_timers.insert((pos.0 as usize, pos.1 as usize), 3);
                
                // Destroy soft crates
                if let Some(state::Tile::SoftCrate) = grid_lock.tile(pos.0 as usize, pos.1 as usize) {
                    let delta = GridDelta::SetTile {
                        x: pos.0 as usize,
                        y: pos.1 as usize,
                        tile: state::Tile::Empty,
                    };
                    grid_lock.apply_delta(delta);
                }
                
                // Remove agents hit by explosion
                let mut agents_to_remove = Vec::new();
                for (i, agent) in grid_lock.agents().iter().enumerate() {
                    if agent.position.0 as usize == pos.0 as usize && agent.position.1 as usize == pos.1 as usize {
                        agents_to_remove.push(i);
                    }
                }
                
                // Remove agents in reverse order to maintain indices
                for &index in agents_to_remove.iter().rev() {
                    if let Some(agent) = grid_lock.agents().get(index) {
                        let delta = GridDelta::RemoveAgent(agent.id);
                        grid_lock.apply_delta(delta);
                    }
                }
                
                // Remove bombs at explosion positions and track owners for restoration
                for bomb_pos in bombs_to_remove.iter() {
                    // Find and remove bombs at this position
                    let mut i = 0;
                    while i < grid_lock.bombs().len() {
                        if grid_lock.bombs()[i].position == *bomb_pos {
                            let bomb_to_remove = grid_lock.bombs_mut().remove(i);
                            // Track this bomb's owner to restore their bomb count
                            // Only restore if this bomb is actually exploding (timer = 0)
                            if bomb_to_remove.timer == 0 {
                                agents_to_restore_bombs.insert(bomb_to_remove.owner);
                            }
                            // Don't increment i since we removed an element
                        } else {
                            i += 1;
                        }
                    }
                }
            }
            
            // Restore bomb counts to agents whose bombs exploded
            for agent_id in agents_to_restore_bombs {
                if let Some(agent) = grid_lock.agents_mut().iter_mut().find(|a| a.id == agent_id) {
                    agent.bombs_left = agent.bombs_left.saturating_add(1);
                }
            }
            
            // Now remove the exploding bombs from the grid
            for bomb_pos in &bombs_to_remove {
                grid_lock.bombs_mut().retain(|b| b.position != *bomb_pos);
            }
            
            drop(grid_lock);
            
            // Return a delta indicating explosion occurred
            return Some(GridDelta::None);
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_broadcasts_events() {
        let mut system = BombSystem::new();
        let grid = Arc::new(RwLock::new(GameGrid::new(1, 1)));
        let bus = EventBus::new();
        system.run(&grid, &bus);
        // no assertion on content, just ensure call succeeds
    }
}