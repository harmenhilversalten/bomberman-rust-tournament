//! Trait definitions for influence layers.

use std::any::Any;

use state::GameState;

use super::{DirtyRegion, InfluenceType};

/// Trait implemented by influence map layers like danger or opportunity.
pub trait InfluenceLayer: Send + Sync {
    /// Returns influence value at given coordinates.
    fn get_influence(&self, x: u16, y: u16) -> f32;

    /// Sets influence value at given coordinates.
    fn set_influence(&mut self, x: u16, y: u16, value: f32);

    /// Updates the layer based on the current game state and dirty regions.
    fn update(&mut self, state: &GameState, dirty: &[DirtyRegion]);

    /// Clears layer state.
    fn clear(&mut self);

    /// Returns the layer type.
    fn get_layer_type(&self) -> InfluenceType;

    /// Converts to `Any` for downcasting.
    fn as_any(&mut self) -> &mut dyn Any;
}
