use crate::core::InfluenceMap as CoreMap;

/// Re-export of the core influence map with helper methods.
pub type InfluenceMap = CoreMap;

/// Position used for influence queries.
#[derive(Clone, Copy)]
pub struct Position {
    /// Horizontal coordinate.
    pub x: i32,
    /// Vertical coordinate.
    pub y: i32,
}

impl Position {
    /// Create a new [`Position`].
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Immutable view into influence values.
pub struct InfluenceData<'a> {
    pub(crate) map: &'a InfluenceMap,
}

impl InfluenceMap {
    /// Obtain an immutable data view.
    pub fn data(&self) -> InfluenceData<'_> {
        InfluenceData { map: self }
    }
}

impl<'a> InfluenceData<'a> {
    /// Width of the influence map.
    pub fn width(&self) -> u16 {
        self.map.width()
    }

    /// Height of the influence map.
    pub fn height(&self) -> u16 {
        self.map.height()
    }

    /// Danger score at the given position.
    pub fn get_danger_at(&self, position: Position) -> f32 {
        self.map
            .danger_at(position.x as u16, position.y as u16)
            .unwrap_or(0.0)
    }

    /// Whether a given set of positions represents a safe path.
    pub fn is_safe_path<I>(&self, positions: I) -> bool
    where
        I: IntoIterator<Item = Position>,
    {
        positions.into_iter().all(|p| self.get_danger_at(p) <= 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use state::GameState;

    #[test]
    fn data_reports_zero_danger_initially() {
        let mut map = InfluenceMap::new(1, 1);
        let _ = map.update(&GameState::new(1, 1));
        let data = map.data();
        assert_eq!(data.get_danger_at(Position::new(0, 0)), 0.0);
    }
}
