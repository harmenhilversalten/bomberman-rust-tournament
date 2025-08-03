use std::collections::HashMap;
use std::time::Duration;

use events::events::bot_events::BotId;

use super::GameResult;
use crate::config::ScoringSystem;

#[derive(Debug, Clone, Default)]
pub struct BotScore {
    pub wins: u32,
    pub losses: u32,
    pub survival_time_total: Duration,
    pub destruction_points: u32,
    pub powerups_collected: u32,
}

#[derive(Debug, Clone)]
pub struct ScoreTracker {
    pub bot_scores: HashMap<BotId, BotScore>,
    pub _scoring_system: ScoringSystem,
}

impl ScoreTracker {
    pub fn new(scoring_system: ScoringSystem) -> Self {
        Self {
            bot_scores: HashMap::new(),
            _scoring_system: scoring_system,
        }
    }

    pub fn update_scores(&mut self, game_results: &[GameResult]) {
        for result in game_results {
            self.update_individual_scores(result);
        }
    }

    fn update_individual_scores(&mut self, result: &GameResult) {
        for &bot in &result.participants {
            let entry = self.bot_scores.entry(bot).or_default();
            if bot == result.winner {
                entry.wins += 1;
            } else {
                entry.losses += 1;
            }
            entry.survival_time_total +=
                result.survival_times.get(&bot).copied().unwrap_or_default();
            entry.destruction_points += result.destruction_points.get(&bot).copied().unwrap_or(0);
            entry.powerups_collected += result.powerups_collected.get(&bot).copied().unwrap_or(0);
        }
    }

    pub fn get_rankings(&self) -> Vec<(BotId, BotScore, u32)> {
        let mut scores: Vec<(BotId, BotScore)> = self
            .bot_scores
            .iter()
            .map(|(id, score)| (*id, score.clone()))
            .collect();
        scores.sort_by(|a, b| b.1.wins.cmp(&a.1.wins));
        scores
            .into_iter()
            .enumerate()
            .map(|(idx, (id, score))| (id, score, idx as u32 + 1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn sample_result(winner: BotId, loser: BotId) -> GameResult {
        GameResult {
            winner,
            participants: vec![winner, loser],
            survival_times: HashMap::from([
                (winner, Duration::from_secs(10)),
                (loser, Duration::from_secs(5)),
            ]),
            destruction_points: HashMap::new(),
            powerups_collected: HashMap::new(),
        }
    }

    #[test]
    fn updates_and_ranks() {
        let mut tracker = ScoreTracker::new(ScoringSystem::WinLoss {
            win_points: 1,
            loss_points: 0,
        });
        let r1 = sample_result(1, 2);
        let r2 = sample_result(2, 1);
        tracker.update_scores(&[r1, r2]);
        let rankings = tracker.get_rankings();
        assert_eq!(rankings.len(), 2);
        assert_eq!(rankings[0].1.wins, 1);
    }
}
