//! Opportunity layer implementation.

use std::any::Any;

use state::GameState;

use super::{DirtyRegion, InfluenceType, layer::InfluenceLayer};

/// A positive influence source such as a power-up.
#[derive(Debug, Clone, Copy)]
pub struct OpportunitySource {
    /// X coordinate.
    pub x: u16,
    /// Y coordinate.
    pub y: u16,
    /// Base value of the influence.
    pub value: f32,
    /// Maximum propagation range measured in Manhattan distance.
    pub range: u16,
}

/// Influence layer representing opportunities.
pub struct OpportunityMap {
    width: u16,
    data: Vec<f32>,
    sources: Vec<OpportunitySource>,
}

impl OpportunityMap {
    /// Creates a new opportunity map.
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
    pub fn add_source(&mut self, source: OpportunitySource) {
        self.sources.push(source);
    }
}

impl InfluenceLayer for OpportunityMap {
    fn get_influence(&self, x: u16, y: u16) -> f32 {
        self.data[self.index(x, y)]
    }

    fn set_influence(&mut self, x: u16, y: u16, value: f32) {
        let idx = self.index(x, y);
        self.data[idx] = value;
    }

    fn update(&mut self, _state: &GameState, dirty: &[DirtyRegion]) {
        for region in dirty {
            for y in region.y..region.y + region.height {
                for x in region.x..region.x + region.width {
                    let mut value = 0.0;
                    for src in &self.sources {
                        let dist = x.abs_diff(src.x) + y.abs_diff(src.y);
                        if dist <= src.range {
                            let influence = src.value * (1.0 - dist as f32 / src.range as f32);
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
        InfluenceType::Opportunity
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
