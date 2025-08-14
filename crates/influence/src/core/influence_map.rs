//! Influence map structure and management.

use std::collections::HashMap;

use state::GameState;

use super::{
    danger::{DangerMap, DangerSource},
    layer::InfluenceLayer,
    opportunity::{OpportunityMap, OpportunitySource},
};
use crate::update::{DirtyTracker, FullUpdate, UpdateStrategy};

/// Types of influence layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InfluenceType {
    /// Danger layer representing threats.
    Danger,
    /// Opportunity layer representing beneficial tiles.
    Opportunity,
}

/// Region of the map marked as dirty needing recomputation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirtyRegion {
    /// X coordinate of the region origin.
    pub x: u16,
    /// Y coordinate of the region origin.
    pub y: u16,
    /// Width of the region.
    pub width: u16,
    /// Height of the region.
    pub height: u16,
}

/// Errors returned by influence map operations.
#[derive(Debug, thiserror::Error)]
pub enum InfluenceError {
    /// Requested layer not found.
    #[error("influence layer {0:?} not found")]
    LayerNotFound(InfluenceType),
}

/// Main influence map containing multiple layers and dirty region tracking.
pub struct InfluenceMap {
    width: u16,
    height: u16,
    layers: HashMap<InfluenceType, Box<dyn InfluenceLayer>>,
    dirty: DirtyTracker,
    strategy: Box<dyn UpdateStrategy>,
}

impl InfluenceMap {
    /// Creates a new influence map with default danger and opportunity layers.
    pub fn new(width: u16, height: u16) -> Self {
        let mut layers: HashMap<InfluenceType, Box<dyn InfluenceLayer>> = HashMap::new();
        layers.insert(
            InfluenceType::Danger,
            Box::new(DangerMap::new(width, height)) as Box<dyn InfluenceLayer>,
        );
        layers.insert(
            InfluenceType::Opportunity,
            Box::new(OpportunityMap::new(width, height)) as Box<dyn InfluenceLayer>,
        );

        Self {
            width,
            height,
            layers,
            dirty: DirtyTracker::new(),
            strategy: Box::new(FullUpdate::new()),
        }
    }

    /// Creates a new map with a custom update strategy.
    pub fn with_strategy(width: u16, height: u16, strategy: Box<dyn UpdateStrategy>) -> Self {
        let mut map = Self::new(width, height);
        map.strategy = strategy;
        map
    }

    /// Sets the update strategy at runtime.
    pub fn set_update_strategy(&mut self, strategy: Box<dyn UpdateStrategy>) {
        self.strategy = strategy;
    }

    /// Marks a region of the map as dirty for the next update.
    pub fn mark_dirty(&mut self, region: DirtyRegion) {
        self.dirty.mark(region);
    }

    /// Returns the map width.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Returns the map height.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Adds a danger source to the underlying danger layer.
    pub fn add_danger_source(&mut self, source: DangerSource) {
        if let Some(layer) = self.layers.get_mut(&InfluenceType::Danger) {
            if let Some(danger) = layer.as_any().downcast_mut::<DangerMap>() {
                danger.add_source(source);
            }
        }
        self.mark_dirty(region_from_source(
            source.x,
            source.y,
            source.range,
            self.width,
            self.height,
        ));
    }

    /// Adds an opportunity source to the underlying opportunity layer.
    pub fn add_opportunity_source(&mut self, source: OpportunitySource) {
        if let Some(layer) = self.layers.get_mut(&InfluenceType::Opportunity) {
            if let Some(opportunity) = layer.as_any().downcast_mut::<OpportunityMap>() {
                opportunity.add_source(source);
            }
        }
        self.mark_dirty(region_from_source(
            source.x,
            source.y,
            source.range,
            self.width,
            self.height,
        ));
    }

    /// Recomputes layers using the provided state and current dirty regions.
    pub fn update(&mut self, state: &GameState) -> Result<(), InfluenceError> {
        self.strategy
            .update(&mut self.dirty, self.width, self.height);
        let regions: Vec<DirtyRegion> = self.dirty.regions().to_vec();
        for layer in self.layers.values_mut() {
            layer.update(state, &regions);
        }
        self.dirty.clear();
        Ok(())
    }

    fn layer(&self, ty: InfluenceType) -> Result<&dyn InfluenceLayer, InfluenceError> {
        self.layers
            .get(&ty)
            .map(|l| l.as_ref())
            .ok_or(InfluenceError::LayerNotFound(ty))
    }

    /// Returns danger influence at coordinates.
    pub fn danger_at(&self, x: u16, y: u16) -> Result<f32, InfluenceError> {
        Ok(self.layer(InfluenceType::Danger)?.get_influence(x, y))
    }

    /// Returns opportunity influence at coordinates.
    pub fn opportunity_at(&self, x: u16, y: u16) -> Result<f32, InfluenceError> {
        Ok(self.layer(InfluenceType::Opportunity)?.get_influence(x, y))
    }
}

fn region_from_source(x: u16, y: u16, range: u16, width: u16, height: u16) -> DirtyRegion {
    let start_x = x.saturating_sub(range);
    let start_y = y.saturating_sub(range);
    let end_x = (u32::from(x) + u32::from(range)).min(u32::from(width - 1));
    let end_y = (u32::from(y) + u32::from(range)).min(u32::from(height - 1));
    DirtyRegion {
        x: start_x,
        y: start_y,
        width: (end_x - u32::from(start_x) + 1) as u16,
        height: (end_y - u32::from(start_y) + 1) as u16,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use state::GameState;

    #[test]
    fn danger_source_updates_cells() {
        let mut map = InfluenceMap::new(5, 5);
        assert_eq!(map.width(), 5);
        assert_eq!(map.height(), 5);
        map.add_danger_source(DangerSource {
            x: 2,
            y: 2,
            strength: 1.0,
            range: 2,
        });
        map.update(&GameState::new(5, 5)).unwrap();
        assert!((map.danger_at(2, 2).unwrap() - 1.0).abs() < f32::EPSILON);
        assert!((map.danger_at(3, 2).unwrap() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn opportunity_source_updates_cells() {
        let mut map = InfluenceMap::new(5, 5);
        assert_eq!(map.width(), 5);
        assert_eq!(map.height(), 5);
        map.add_opportunity_source(OpportunitySource {
            x: 0,
            y: 0,
            value: 2.0,
            range: 3,
        });
        map.update(&GameState::new(5, 5)).unwrap();
        assert!((map.opportunity_at(0, 0).unwrap() - 2.0).abs() < f32::EPSILON);
        assert!((map.opportunity_at(1, 0).unwrap() - (2.0 * (1.0 - 1.0 / 3.0))).abs() < 1e-6);
    }
}
