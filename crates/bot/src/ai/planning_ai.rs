use crate::bot::decision::DecisionMaker;

/// Planning AI that decrements the snapshot to simulate forward planning.
pub struct PlanningAI;

impl DecisionMaker<i32, i32> for PlanningAI {
    fn decide(&mut self, snapshot: i32) -> i32 {
        snapshot - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::decision::DecisionMaker;

    #[test]
    fn planning_ai_decrements_snapshot() {
        let mut ai = PlanningAI;
        assert_eq!(ai.decide(3), 2);
    }
}
