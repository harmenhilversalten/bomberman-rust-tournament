//! AI strategy implementations.

mod heuristic_ai;
mod planning_ai;
mod reactive_ai;

pub use heuristic_ai::HeuristicAI;
pub use planning_ai::PlanningAI;
pub use reactive_ai::ReactiveAI;

/// Available AI strategy types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiType {
    /// Basic heuristic-based decision making.
    Heuristic,
    /// Reactive AI responding directly to snapshots.
    Reactive,
    /// Planning AI evaluating future states.
    Planning,
}

/// AI that can switch between different strategies at runtime.
pub struct SwitchingAI {
    current: AiType,
    heuristic: HeuristicAI,
    reactive: ReactiveAI,
    planning: PlanningAI,
}

impl SwitchingAI {
    /// Create a new [`SwitchingAI`] with the initial strategy [`AiType`].
    pub fn new(initial: AiType) -> Self {
        Self {
            current: initial,
            heuristic: HeuristicAI,
            reactive: ReactiveAI,
            planning: PlanningAI,
        }
    }

    /// Switch to a different AI strategy.
    pub fn switch(&mut self, new: AiType) {
        self.current = new;
    }
}

use crate::bot::decision::DecisionMaker;
use events::events::BotDecision;
use state::grid::GridDelta;

impl DecisionMaker<GridDelta, BotDecision> for SwitchingAI {
    fn decide(&mut self, snapshot: GridDelta) -> BotDecision {
        match self.current {
            AiType::Heuristic => self.heuristic.decide(snapshot),
            AiType::Reactive => self.reactive.decide(snapshot),
            AiType::Planning => self.planning.decide(snapshot),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::decision::DecisionMaker;
    use events::events::BotDecision;
    use state::grid::GridDelta;

    #[test]
    fn switching_between_strategies_changes_behavior() {
        let mut ai = SwitchingAI::new(AiType::Heuristic);
        assert_eq!(ai.decide(GridDelta::None), BotDecision::PlaceBomb);

        ai.switch(AiType::Reactive);
        assert_eq!(ai.decide(GridDelta::None), BotDecision::Wait);

        ai.switch(AiType::Planning);
        assert_eq!(ai.decide(GridDelta::None), BotDecision::Wait);
    }
}
