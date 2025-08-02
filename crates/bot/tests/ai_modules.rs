use bot::DecisionMaker;
use bot::ai::{HeuristicAI, PlanningAI, ReactiveAI};
use events::events::BotDecision;
use state::grid::GridDelta;

#[test]
fn heuristic_ai_places_bomb() {
    let mut ai = HeuristicAI;
    assert_eq!(ai.decide(GridDelta::None), BotDecision::PlaceBomb);
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
