use events::events::bot_events::BotId;

use crate::SystemHandle;

use super::scheduler::GameId;
use super::{GameResult, TournamentError};

#[derive(Debug)]
pub struct GameSession {
    pub _id: GameId,
    pub participants: Vec<BotId>,
    pub state: SessionState,
    pub result: Option<GameResult>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    Scheduled,
    Running,
    Completed,
}

impl GameSession {
    pub fn new(id: GameId, participants: Vec<BotId>) -> Self {
        Self { _id: id, participants, state: SessionState::Scheduled, result: None }
    }

    pub async fn start(&mut self, _system_handle: &SystemHandle) -> Result<(), TournamentError> {
        self.state = SessionState::Running;
        let winner = *self
            .participants
            .first()
            .ok_or_else(|| TournamentError::GameFailed("no participants".into()))?;
        self.result = Some(GameResult::new(self.participants.clone(), winner));
        self.state = SessionState::Completed;
        Ok(())
    }

    pub async fn wait_for_completion(&mut self) -> Result<GameResult, TournamentError> {
        match self.result.clone() {
            Some(r) => Ok(r),
            None => Err(TournamentError::GameFailed("missing result".into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SystemHandle;

    #[test]
    fn session_completes() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let handle = dummy_handle();
            let mut session = GameSession::new(0, vec![1, 2]);
            session.start(&handle).await.unwrap();
            let res = session.wait_for_completion().await.unwrap();
            assert_eq!(res.winner, 1);
        });
    }

    fn dummy_handle() -> SystemHandle {
        use crate::{config::*, engine::Engine};
        use events::bus::EventBus;
        use std::sync::{Arc, RwLock};
        let cfg = EngineConfig::default();
        let grid = Arc::new(RwLock::new(state::GameGrid::new(5, 5)));
        let (engine, _) = Engine::with_components(cfg, grid, Arc::new(EventBus::new()));
        SystemHandle::new(Arc::new(EventBus::new()), engine, 0, None)
    }
}
