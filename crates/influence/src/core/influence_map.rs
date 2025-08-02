//! Influence map structure and management.

use std::collections::HashMap;

use state::GameState;

use super::{
    danger::{DangerMap, DangerSource},
    layer::InfluenceLayer,
    opportunity::{OpportunityMap, OpportunitySource},
};

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
    dirty_regions: Vec<DirtyRegion>,
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
            dirty_regions: Vec::new(),
        }
    }

    /// Marks a region of the map as dirty for the next update.
    pub fn mark_dirty(&mut self, region: DirtyRegion) {
        self.dirty_regions.push(region);
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
    }

    /// Adds an opportunity source to the underlying opportunity layer.
    pub fn add_opportunity_source(&mut self, source: OpportunitySource) {
        if let Some(layer) = self.layers.get_mut(&InfluenceType::Opportunity) {
            if let Some(opportunity) = layer.as_any().downcast_mut::<OpportunityMap>() {
                opportunity.add_source(source);
            }
        }
    }

    /// Recomputes layers using the provided state and current dirty regions.
    pub fn update(&mut self, state: &GameState) -> Result<(), InfluenceError> {
        for layer in self.layers.values_mut() {
            layer.update(state, &self.dirty_regions);
        }
        self.dirty_regions.clear();
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
        map.mark_dirty(DirtyRegion {
            x: 0,
            y: 0,
            width: 5,
            height: 5,
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
        map.mark_dirty(DirtyRegion {
            x: 0,
            y: 0,
            width: 5,
            height: 5,
        });
        map.update(&GameState::new(5, 5)).unwrap();
        assert!((map.opportunity_at(0, 0).unwrap() - 2.0).abs() < f32::EPSILON);
        assert!((map.opportunity_at(1, 0).unwrap() - (2.0 * (1.0 - 1.0 / 3.0))).abs() < 1e-6);
    }
}
