//! Danger layer implementation.

use std::any::Any;

use state::GameState;

use super::{DirtyRegion, InfluenceType, layer::InfluenceLayer};

/// A danger source such as a bomb.
#[derive(Debug, Clone, Copy)]
pub struct DangerSource {
    /// X coordinate.
    pub x: u16,
    /// Y coordinate.
    pub y: u16,
    /// Base strength of the influence.
    pub strength: f32,
    /// Maximum propagation range measured in Manhattan distance.
    pub range: u16,
}

/// Influence layer representing dangers.
pub struct DangerMap {
    width: u16,
    data: Vec<f32>,
    sources: Vec<DangerSource>,
}

impl DangerMap {
    /// Creates a new danger map.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            data: vec![0.0; width as usize * height as usize],
            sources: Vec::new(),
        }
    }

    fn index(&self, x: u16, y: u16) -> usize {
        y as usize * self.width as usize + x as usize
    }

    /// Adds a source to the map.
    pub fn add_source(&mut self, source: DangerSource) {
        self.sources.push(source);
    }
}

impl InfluenceLayer for DangerMap {
    fn get_influence(&self, x: u16, y: u16) -> f32 {
        self.data[self.index(x, y)]
    }

    fn set_influence(&mut self, x: u16, y: u16, value: f32) {
        let idx = self.index(x, y);
        self.data[idx] = value;
    }

    fn update(&mut self, _state: &GameState, dirty: &[DirtyRegion]) {
        // Clear the data first
        self.data.fill(0.0);
        
        // Then recalculate danger for all regions
        for region in dirty {
            for y in region.y..region.y + region.height {
                for x in region.x..region.x + region.width {
                    let mut value = 0.0;
                    for src in &self.sources {
                        let dist = x.abs_diff(src.x) + y.abs_diff(src.y);
                        if dist <= src.range {
                            // Calculate influence with proper normalization
                            // At the source, influence is full strength
                            // At the edge of range, influence is 0
                            let influence = src.strength * (1.0 - (dist as f32 / src.range as f32));
                            value += influence;
                        }
                    }
                    self.set_influence(x, y, value);
                }
            }
        }
    }

    fn clear(&mut self) {
        self.data.fill(0.0);
        self.sources.clear();
    }

    fn get_layer_type(&self) -> InfluenceType {
        InfluenceType::Danger
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
