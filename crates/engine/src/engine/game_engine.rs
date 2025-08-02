use std::sync::{Arc, Mutex, RwLock};

use super::scheduler::TaskScheduler;
use crate::{
    config::EngineConfig,
    simulation::{DeterminismChecker, Replay, ReplayRecorder},
    systems::System,
};
use state::{GameGrid, grid::GridDelta};
use tokio::sync::watch;

/// Core game engine advancing the simulation and broadcasting changes.
pub struct Engine {
    config: EngineConfig,
    grid: Arc<RwLock<GameGrid>>,
    delta_tx: watch::Sender<GridDelta>,
    scheduler: TaskScheduler,
    systems: Vec<Arc<Mutex<Box<dyn System>>>>,
    replay_recorder: ReplayRecorder,
    determinism_checker: DeterminismChecker,
}

impl Engine {
    /// Creates a new engine configured via [`EngineConfig`].
    pub fn new(config: EngineConfig) -> (Self, watch::Receiver<GridDelta>) {
        let grid = GameGrid::new(config.width, config.height);
        let (tx, rx) = watch::channel(GridDelta::None);
        (
            Self {
                config,
                grid: Arc::new(RwLock::new(grid)),
                delta_tx: tx,
                scheduler: TaskScheduler::new(),
                systems: Vec::new(),
                replay_recorder: ReplayRecorder::new(),
                determinism_checker: DeterminismChecker::new(),
            },
            rx,
        )
    }

    /// Advances the game by a single tick by running all registered systems.
    pub fn tick(&mut self) {
        self.scheduler.run();
        let grid = self.grid.read().expect("grid lock poisoned");
        self.determinism_checker.record(&grid);
    }

    /// Access the shared game grid.
    pub fn grid(&self) -> Arc<RwLock<GameGrid>> {
        Arc::clone(&self.grid)
    }

    /// Access the engine configuration.
    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    /// Start recording a replay.
    pub fn start_replay_recording(&mut self) {
        self.replay_recorder.start();
        self.determinism_checker = DeterminismChecker::new();
    }

    /// Stop recording and return the replay.
    pub fn stop_replay_recording(&mut self) -> Replay {
        self.replay_recorder.stop()
    }

    /// Access determinism hashes collected each tick.
    pub fn determinism_hashes(&self) -> &[u64] {
        self.determinism_checker.hashes()
    }

    /// Apply a replay to the current grid recording hashes.
    pub fn load_replay(&mut self, replay: &Replay) {
        let mut grid = self.grid.write().expect("grid lock poisoned");
        for delta in replay.deltas() {
            grid.apply_delta(delta.clone());
            self.determinism_checker.record(&grid);
        }
    }

    /// Add a task to the internal scheduler.
    pub fn add_task<F>(&mut self, name: &str, deps: Vec<String>, parallel: bool, task: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.scheduler.add_task(name, deps, parallel, task);
    }

    /// Register a new system with the engine.
    pub fn add_system(&mut self, system: Box<dyn System>) {
        let deps = system
            .dependencies()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let parallel = system.parallelizable();
        let name = system.name().to_string();
        let sys = Arc::new(Mutex::new(system));
        let grid = Arc::clone(&self.grid);
        let tx = self.delta_tx.clone();
        let sys_clone = Arc::clone(&sys);
        let recorder = self.replay_recorder.clone();
        self.scheduler.add_task(name, deps, parallel, move || {
            let mut s = sys_clone.lock().expect("system lock poisoned");
            if let Some(delta) = s.run(&grid) {
                let mut g = grid.write().expect("grid lock poisoned");
                g.apply_delta(delta.clone());
                recorder.record(delta.clone());
                let _ = tx.send(delta);
            }
        });
        self.systems.push(sys);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    #[test]
    fn tick_broadcasts_system_delta() {
        use crate::{config::EngineConfig, systems::MovementSystem};

        let config = EngineConfig {
            width: 1,
            height: 1,
            ..EngineConfig::default()
        };
        let (mut engine, mut rx) = Engine::new(config);
        engine.add_system(Box::new(MovementSystem::new()));
        assert_eq!(*rx.borrow(), GridDelta::None);
        engine.tick();
        assert!(matches!(
            rx.borrow_and_update().clone(),
            GridDelta::SetTile { x: 0, y: 0, .. }
        ));
    }

    #[test]
    fn tick_runs_scheduler_tasks() {
        use crate::config::EngineConfig;
        let config = EngineConfig {
            width: 1,
            height: 1,
            ..EngineConfig::default()
        };
        let (mut engine, _rx) = Engine::new(config);
        let flag = Arc::new(AtomicBool::new(false));
        let flag_clone = Arc::clone(&flag);
        engine.add_task("flag", vec![], true, move || {
            flag_clone.store(true, Ordering::SeqCst);
        });
        engine.tick();
        assert!(flag.load(Ordering::SeqCst));
    }

    #[test]
    fn engine_uses_config() {
        use crate::config::EngineConfig;
        let cfg = EngineConfig {
            width: 2,
            height: 3,
            tick_rate: 30,
            ..EngineConfig::default()
        };
        let (engine, _rx) = Engine::new(cfg.clone());
        assert_eq!(engine.config().tick_rate, 30);
        assert_eq!(engine.config().width, 2);
        assert_eq!(engine.config().height, 3);
    }
}
