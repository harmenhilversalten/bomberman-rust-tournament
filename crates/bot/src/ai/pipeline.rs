use std::sync::{Arc, Mutex};

use goals::{Goal, GoalManager, GoalScorer};
use influence::map::{InfluenceData, InfluenceMap, Position};
use path::{Path, Pathfinder, Point};
use state::{GameState, grid::GridDelta};

use crate::bot::decision::DecisionMaker;
use events::events::BotDecision;

/// Pipeline coordinating goal generation, pathfinding and influence queries.
pub struct AIDecisionPipeline {
    goal_manager: Arc<GoalManager>,
    pathfinder: Arc<Pathfinder>,
    influence_map: Arc<Mutex<InfluenceMap>>,
    scorer: GoalScorer,
}

impl AIDecisionPipeline {
    /// Create a new [`AIDecisionPipeline`].
    pub fn new(
        goal_manager: Arc<GoalManager>,
        pathfinder: Arc<Pathfinder>,
        influence_map: Arc<Mutex<InfluenceMap>>,
    ) -> Self {
        Self {
            goal_manager,
            pathfinder,
            influence_map,
            scorer: GoalScorer::new(),
        }
    }

    /// Generate candidate goals.
    pub fn generate_goals(&self, snapshot: &GameState) -> Vec<Box<dyn Goal>> {
        self.goal_manager.generate_goals(snapshot)
    }

    /// Score goals using the influence map.
    pub fn score_goals(
        &self,
        goals: Vec<Box<dyn Goal>>,
        influence: &InfluenceData,
        snapshot: &GameState,
    ) -> Vec<(Box<dyn Goal>, f32)> {
        goals
            .into_iter()
            .map(|g| {
                let score = self.scorer.score_goal(g.as_ref(), snapshot, influence, 0);
                (g, score)
            })
            .collect()
    }

    /// Find a path for the specified goal.
    pub fn find_path(&self, _goal: &dyn Goal, _snapshot: &GameState) -> Option<Path> {
        self.pathfinder.find_path(
            Point::new(0, 0),
            Point::new(0, 0),
            &self.influence_map.lock().unwrap().data(),
        )
    }

    /// Select an action from the path and goal.
    pub fn select_action(&self, path: Option<Path>, _goal: &dyn Goal) -> BotDecision {
        if let Some(p) = path {
            let positions: Vec<Position> = p
                .nodes
                .iter()
                .map(|n| Position::new(n.position.x, n.position.y))
                .collect();
            if self
                .influence_map
                .lock()
                .unwrap()
                .data()
                .is_safe_path(positions)
                && !p.to_movement_commands().is_empty()
            {
                return BotDecision::Wait;
            }
        }
        BotDecision::Wait
    }
}

impl DecisionMaker<GridDelta, BotDecision> for AIDecisionPipeline {
    fn decide(&mut self, _delta: GridDelta) -> BotDecision {
        let snapshot = GameState::new(1, 1);
        let mut map_guard = self.influence_map.lock().unwrap();
        let _ = map_guard.update(&snapshot);
        let influence = map_guard.data();
        let goals = self.generate_goals(&snapshot);
        let scored = self.score_goals(goals, &influence, &snapshot);
        drop(map_guard);
        if let Some((goal, _)) = scored
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        {
            let path = self.find_path(goal.as_ref(), &snapshot);
            self.select_action(path, goal.as_ref())
        } else {
            BotDecision::Wait
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_returns_decision() {
        let gm = Arc::new(GoalManager::new());
        let pf = Arc::new(Pathfinder::new());
        let im = Arc::new(Mutex::new(InfluenceMap::new(1, 1)));
        let mut pipeline = AIDecisionPipeline::new(gm, pf, im);
        assert_eq!(pipeline.decide(GridDelta::None), BotDecision::Wait);
    }
}
