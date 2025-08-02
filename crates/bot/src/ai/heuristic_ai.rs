use crate::bot::decision::DecisionMaker;

/// Simple heuristic AI that increments the snapshot.
pub struct HeuristicAI;

impl DecisionMaker<i32, i32> for HeuristicAI {
    fn decide(&mut self, snapshot: i32) -> i32 {
        snapshot + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::decision::DecisionMaker;

    #[test]
    fn heuristic_ai_increments_snapshot() {
        let mut ai = HeuristicAI;
        assert_eq!(ai.decide(1), 2);
    }
}
