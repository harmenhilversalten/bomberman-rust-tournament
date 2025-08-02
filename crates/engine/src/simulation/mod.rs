pub mod determinism;
pub mod replay;

pub use determinism::{DeterminismChecker, hash_grid};
pub use replay::{Replay, ReplayRecorder};

#[cfg(test)]
mod tests {
    use crate::{engine::Engine, systems::MovementSystem};

    #[test]
    fn replay_reproduces_state() {
        let (mut engine, _rx) = Engine::new(1);
        engine.add_system(Box::new(MovementSystem::new()));
        engine.start_replay_recording();
        for _ in 0..3 {
            engine.tick();
        }
        let replay = engine.stop_replay_recording();
        let recorded_hashes = engine.determinism_hashes().to_vec();

        let (mut engine2, _rx2) = Engine::new(1);
        engine2.load_replay(&replay);
        assert_eq!(engine2.determinism_hashes(), recorded_hashes.as_slice());
    }
}
