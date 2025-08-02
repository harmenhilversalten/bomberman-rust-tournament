//! Strategic placement choosing the highest scoring option.

use crate::bomb::entity::Position;

use super::PlacementStrategy;

/// Strategy that scores each option using a provided function and selects the maximum.
pub struct StrategicPlacer<F>
where
    F: Fn(Position) -> i32,
{
    score_fn: F,
}

impl<F> StrategicPlacer<F>
where
    F: Fn(Position) -> i32,
{
    /// Creates a new strategic placer with the given scoring function.
    pub fn new(score_fn: F) -> Self {
        Self { score_fn }
    }
}

impl<F> PlacementStrategy for StrategicPlacer<F>
where
    F: Fn(Position) -> i32,
{
    fn choose(&self, options: &[Position]) -> Option<Position> {
        options.iter().copied().max_by_key(|p| (self.score_fn)(*p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_highest_scoring_option() {
        let score = |p: Position| -(p.0 as i32 + p.1 as i32); // prefer lower sums
        let placer = StrategicPlacer::new(score);
        let choice = placer.choose(&[(2, 2), (0, 0), (1, 1)]);
        assert_eq!(choice, Some((0, 0)));
    }
}
