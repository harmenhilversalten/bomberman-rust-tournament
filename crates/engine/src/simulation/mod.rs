pub mod determinism;
pub mod replay;

pub use determinism::{DeterminismChecker, hash_grid};
pub use replay::{Replay, ReplayRecorder};

#[cfg(test)]
mod tests {
    use crate::{config::EngineConfig, engine::Engine, systems::MovementSystem};

    #[test]
    fn replay_reproduces_state() {
        let cfg = EngineConfig {
            width: 1,
            height: 1,
            ..EngineConfig::default()
        };
        let (mut engine, _rx, _events) = Engine::new(cfg.clone());
        engine.add_system(Box::new(MovementSystem::new()));
        engine.start_replay_recording();
        for _ in 0..3 {
            engine.tick();
        }
        let replay = engine.stop_replay_recording();
        let recorded_hashes = engine.determinism_hashes().to_vec();

        let (mut engine2, _rx2, _events2) = Engine::new(cfg);
        engine2.load_replay(&replay);
        assert_eq!(engine2.determinism_hashes(), recorded_hashes.as_slice());
    }
}
