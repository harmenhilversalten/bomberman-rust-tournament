use crate::bot::decision::DecisionMaker;
use events::events::BotDecision;
use state::grid::GridDelta;

/// Planning AI that currently waits each tick.
pub struct PlanningAI;

impl DecisionMaker<GridDelta, BotDecision> for PlanningAI {
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
    fn planning_ai_waits() {
        let mut ai = PlanningAI;
        assert_eq!(ai.decide(GridDelta::None), BotDecision::Wait);
    }
}
