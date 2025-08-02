use crate::bot::decision::DecisionMaker;

/// Reactive AI that echoes the snapshot as the command.
pub struct ReactiveAI;

impl DecisionMaker<i32, i32> for ReactiveAI {
    fn decide(&mut self, snapshot: i32) -> i32 {
        snapshot
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::decision::DecisionMaker;

    #[test]
    fn reactive_ai_echoes_snapshot() {
        let mut ai = ReactiveAI;
        assert_eq!(ai.decide(5), 5);
    }
}
