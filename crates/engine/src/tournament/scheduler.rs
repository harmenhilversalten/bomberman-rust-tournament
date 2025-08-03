use crate::config::TournamentFormat;
use events::events::bot_events::BotId;

pub type GameId = usize;

#[derive(Debug, Clone)]
pub struct GameMatch {
    pub id: GameId,
    pub participants: Vec<BotId>,
}

#[derive(Debug, Clone)]
pub struct GameScheduler {
    pub format: TournamentFormat,
    pub current_round: u32,
}

impl GameScheduler {
    pub fn new(format: TournamentFormat) -> Self {
        Self {
            format,
            current_round: 0,
        }
    }

    pub fn has_next_round(&self) -> bool {
        match self.format {
            TournamentFormat::RoundRobin { total_rounds } => self.current_round < total_rounds,
            TournamentFormat::SingleElimination { bracket_size } => {
                let rounds = bracket_size.next_power_of_two().trailing_zeros();
                self.current_round < rounds
            }
            TournamentFormat::Swiss { rounds } => self.current_round < rounds,
        }
    }

    pub fn schedule_next_round(&mut self, bots: &[BotId]) -> Vec<GameMatch> {
        self.current_round += 1;
        match self.format {
            TournamentFormat::RoundRobin { .. } => self.generate_round_robin(bots),
            TournamentFormat::SingleElimination { .. } => self.generate_round_robin(bots),
            TournamentFormat::Swiss { .. } => self.generate_round_robin(bots),
        }
    }

    fn generate_round_robin(&self, bots: &[BotId]) -> Vec<GameMatch> {
        let mut games = Vec::new();
        let mut id = 0;
        for i in 0..bots.len() {
            for j in (i + 1)..bots.len() {
                games.push(GameMatch {
                    id,
                    participants: vec![bots[i], bots[j]],
                });
                id += 1;
            }
        }
        games
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TournamentFormat;

    #[test]
    fn schedules_round_robin() {
        let mut sched = GameScheduler::new(TournamentFormat::RoundRobin { total_rounds: 1 });
        let bots = vec![0, 1, 2];
        let matches = sched.schedule_next_round(&bots);
        assert_eq!(matches.len(), 3); // 3 choose 2
        let m0 = &matches[0];
        assert_eq!(m0.participants.len(), 2);
    }
}
