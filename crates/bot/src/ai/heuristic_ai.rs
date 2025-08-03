use std::sync::{Arc, Mutex};

use crate::bot::decision::DecisionMaker;
use events::events::BotDecision;
use goals::GoalManager;
use influence::map::InfluenceMap;
use path::Pathfinder;
use state::grid::GridDelta;

use super::AIDecisionPipeline;

/// Heuristic AI backed by the [`AIDecisionPipeline`].
pub struct HeuristicAI {
    pipeline: AIDecisionPipeline,
}

impl HeuristicAI {
    /// Construct a new [`HeuristicAI`].
    pub fn new(
        goal_manager: Arc<GoalManager>,
        pathfinder: Arc<Pathfinder>,
        influence_map: Arc<Mutex<InfluenceMap>>,
    ) -> Self {
        Self {
            pipeline: AIDecisionPipeline::new(goal_manager, pathfinder, influence_map),
        }
    }
}

impl DecisionMaker<GridDelta, BotDecision> for HeuristicAI {
    fn decide(&mut self, snapshot: GridDelta) -> BotDecision {
        self.pipeline.decide(snapshot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::decision::DecisionMaker;

    #[test]
    fn heuristic_ai_uses_pipeline() {
        let gm = Arc::new(GoalManager::new());
        let pf = Arc::new(Pathfinder::new());
        let im = Arc::new(Mutex::new(InfluenceMap::new(1, 1)));
        let mut ai = HeuristicAI::new(gm, pf, im);
        assert_eq!(ai.decide(GridDelta::None), BotDecision::Wait);
    }
}
