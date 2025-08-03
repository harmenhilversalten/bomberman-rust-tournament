//! AI strategy implementations.

mod heuristic_ai;
mod pipeline;
mod planning_ai;
mod reactive_ai;

pub use heuristic_ai::HeuristicAI;
pub use pipeline::AIDecisionPipeline;
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
    pub fn new(
        initial: AiType,
        goal_manager: std::sync::Arc<goals::GoalManager>,
        pathfinder: std::sync::Arc<path::Pathfinder>,
        influence_map: std::sync::Arc<std::sync::Mutex<influence::map::InfluenceMap>>,
    ) -> Self {
        Self {
            current: initial,
            heuristic: HeuristicAI::new(
                std::sync::Arc::clone(&goal_manager),
                std::sync::Arc::clone(&pathfinder),
                std::sync::Arc::clone(&influence_map),
            ),
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
        let gm = std::sync::Arc::new(goals::GoalManager::new());
        let pf = std::sync::Arc::new(path::Pathfinder::new());
        let im = std::sync::Arc::new(std::sync::Mutex::new(influence::map::InfluenceMap::new(
            1, 1,
        )));
        let mut ai = SwitchingAI::new(AiType::Heuristic, gm, pf, im);
        assert_eq!(ai.decide(GridDelta::None), BotDecision::Wait);

        ai.switch(AiType::Reactive);
        assert_eq!(ai.decide(GridDelta::None), BotDecision::Wait);

        ai.switch(AiType::Planning);
        assert_eq!(ai.decide(GridDelta::None), BotDecision::Wait);
    }
}
