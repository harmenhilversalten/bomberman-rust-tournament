use std::sync::{Arc, Mutex, RwLock};

use super::scheduler::TaskScheduler;
use crate::systems::System;
use state::{GameGrid, grid::GridDelta};
use tokio::sync::watch;

/// Core game engine advancing the simulation and broadcasting changes.
pub struct Engine {
    grid: Arc<RwLock<GameGrid>>,
    delta_tx: watch::Sender<GridDelta>,
    scheduler: TaskScheduler,
    systems: Vec<Arc<Mutex<Box<dyn System>>>>,
}

impl Engine {
    /// Creates a new engine with a square grid of the given size.
    pub fn new(size: usize) -> (Self, watch::Receiver<GridDelta>) {
        let grid = GameGrid::new(size, size);
        let (tx, rx) = watch::channel(GridDelta::None);
        (
            Self {
                grid: Arc::new(RwLock::new(grid)),
                delta_tx: tx,
                scheduler: TaskScheduler::new(),
                systems: Vec::new(),
            },
            rx,
        )
    }

    /// Advances the game by a single tick by running all registered systems.
    pub fn tick(&mut self) {
        self.scheduler.run();
    }

    /// Access the shared game grid.
    pub fn grid(&self) -> Arc<RwLock<GameGrid>> {
        Arc::clone(&self.grid)
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
        self.scheduler.add_task(name, deps, parallel, move || {
            let mut s = sys_clone.lock().expect("system lock poisoned");
            if let Some(delta) = s.run(&grid) {
                let mut g = grid.write().expect("grid lock poisoned");
                g.apply_delta(delta.clone());
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
        use crate::systems::MovementSystem;

        let (mut engine, mut rx) = Engine::new(1);
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
        let (mut engine, _rx) = Engine::new(1);
        let flag = Arc::new(AtomicBool::new(false));
        let flag_clone = Arc::clone(&flag);
        engine.add_task("flag", vec![], true, move || {
            flag_clone.store(true, Ordering::SeqCst);
        });
        engine.tick();
        assert!(flag.load(Ordering::SeqCst));
    }
}
