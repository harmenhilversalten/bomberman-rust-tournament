use crate::bot::decision::DecisionMaker;
use events::events::BotDecision;
use state::grid::GridDelta;

/// Simple heuristic AI that always places a bomb.
pub struct HeuristicAI;

impl DecisionMaker<GridDelta, BotDecision> for HeuristicAI {
    fn decide(&mut self, _snapshot: GridDelta) -> BotDecision {
        BotDecision::PlaceBomb
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::decision::DecisionMaker;
    use events::events::BotDecision;
    use state::grid::GridDelta;

    #[test]
    fn heuristic_ai_places_bomb() {
        let mut ai = HeuristicAI;
        assert_eq!(ai.decide(GridDelta::None), BotDecision::PlaceBomb);
    }
}
