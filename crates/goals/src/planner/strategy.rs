//! Planning strategies that can be used by the goal planner.

/// Planning strategies that can be used by the goal planner.
#[derive(Debug, Clone, Copy)]
pub enum PlanningStrategy {
    /// Select the goal with the highest score.
    HighestScore,
}
