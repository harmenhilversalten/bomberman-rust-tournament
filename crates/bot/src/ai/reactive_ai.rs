use crate::bot::decision::DecisionMaker;
use events::events::BotDecision;
use state::grid::GridDelta;

/// Reactive AI that waits on every tick.
pub struct ReactiveAI;

impl DecisionMaker<GridDelta, BotDecision> for ReactiveAI {
    fn decide(&mut self, _snapshot: GridDelta) -> BotDecision {
        BotDecision::Wait
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::decision::DecisionMaker;
    use events::events::BotDecision;
    use state::grid::GridDelta;

    #[test]
    fn reactive_ai_waits() {
        let mut ai = ReactiveAI;
        assert_eq!(ai.decide(GridDelta::None), BotDecision::Wait);
    }
}
