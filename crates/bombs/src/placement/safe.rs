//! Safe bomb placement ensuring the chosen tile is not dangerous.

use std::collections::HashSet;

use crate::bomb::entity::Position;

use super::PlacementStrategy;

/// Strategy selecting the first option that is not in the danger set.
pub struct SafePlacer<'a> {
    danger: &'a HashSet<Position>,
}

impl<'a> SafePlacer<'a> {
    /// Creates a new `SafePlacer` given a set of dangerous positions.
    pub fn new(danger: &'a HashSet<Position>) -> Self {
        Self { danger }
    }
}

impl<'a> PlacementStrategy for SafePlacer<'a> {
    fn choose(&self, options: &[Position]) -> Option<Position> {
        options.iter().copied().find(|p| !self.danger.contains(p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_safe_option() {
        let danger: HashSet<Position> = vec![(1, 1)].into_iter().collect();
        let placer = SafePlacer::new(&danger);
        let choice = placer.choose(&[(1, 1), (2, 2)]);
        assert_eq!(choice, Some((2, 2)));
    }
}
