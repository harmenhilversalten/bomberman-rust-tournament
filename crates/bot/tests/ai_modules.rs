use bot::DecisionMaker;
use bot::ai::{HeuristicAI, PlanningAI, ReactiveAI};
use events::events::BotDecision;
use goals::GoalManager;
use influence::map::InfluenceMap;
use path::Pathfinder;
use state::grid::GridDelta;
use std::sync::{Arc, Mutex};

#[test]
fn heuristic_ai_uses_pipeline() {
    let gm = Arc::new(GoalManager::new());
    let pf = Arc::new(Pathfinder::new());
    let im = Arc::new(Mutex::new(InfluenceMap::new(1, 1)));
    let mut ai = HeuristicAI::new(gm, pf, im);
    assert_eq!(ai.decide(GridDelta::None), BotDecision::Wait);
}

#[test]
fn reactive_ai_waits() {
    let mut ai = ReactiveAI;
    assert_eq!(ai.decide(GridDelta::None), BotDecision::Wait);
}

#[test]
fn planning_ai_waits() {
    let mut ai = PlanningAI;
    assert_eq!(ai.decide(GridDelta::None), BotDecision::Wait);
}
