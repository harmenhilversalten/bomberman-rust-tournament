use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

/// Represents a task that can be executed by the scheduler.
///
/// Tasks are identified by a unique name and may declare dependencies
/// on other tasks. Tasks marked as `parallelizable` can run alongside
/// other tasks in the same scheduling stage.
struct ScheduledTask {
    task: Arc<dyn Fn() + Send + Sync>,
    dependencies: Vec<String>,
    parallelizable: bool,
}

/// Simple task scheduler executing tasks in dependency order and running
/// independent tasks in parallel using Tokio.
pub struct TaskScheduler {
    tasks: HashMap<String, ScheduledTask>,
}

impl TaskScheduler {
    /// Create a new scheduler.
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    /// Add a task to the scheduler.
    pub fn add_task<F, S>(
        &mut self,
        name: S,
        dependencies: Vec<String>,
        parallelizable: bool,
        task: F,
    ) where
        F: Fn() + Send + Sync + 'static,
        S: Into<String>,
    {
        self.tasks.insert(
            name.into(),
            ScheduledTask {
                task: Arc::new(task),
                dependencies,
                parallelizable,
            },
        );
    }

    /// Execute all scheduled tasks respecting dependency ordering.
    ///
    /// Tasks with no dependencies at the same stage are run in parallel
    /// when marked as `parallelizable`.
    pub async fn run(&self) {
        let mut indegree: HashMap<String, usize> =
            self.tasks.keys().map(|k| (k.clone(), 0)).collect();
        let mut dependents: HashMap<String, Vec<String>> = HashMap::new();
        for (name, task) in &self.tasks {
            for dep in &task.dependencies {
                *indegree.entry(name.clone()).or_default() += 1;
                dependents
                    .entry(dep.clone())
                    .or_default()
                    .push(name.clone());
            }
        }

        let mut ready: VecDeque<String> = indegree
            .iter()
            .filter(|(_, deg)| **deg == 0)
            .map(|(n, _)| n.clone())
            .collect();
        let mut executed = HashSet::new();

        while !ready.is_empty() {
            let mut batch = Vec::new();
            while let Some(name) = ready.pop_front() {
                if executed.insert(name.clone()) {
                    batch.push(name);
                }
            }

            let mut joins = Vec::new();
            for name in &batch {
                let task = self.tasks.get(name).expect("task must exist");
                let func = Arc::clone(&task.task);
                if task.parallelizable {
                    joins.push(tokio::spawn(async move {
                        (func)();
                    }));
                } else {
                    (func)();
                }
            }
            for join in joins {
                let _ = join.await;
            }

            for name in batch {
                if let Some(children) = dependents.get(&name) {
                    for child in children {
                        if let Some(d) = indegree.get_mut(child) {
                            *d -= 1;
                            if *d == 0 {
                                ready.push_back(child.clone());
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::time::{Duration, Instant};

    #[test]
    fn tasks_follow_dependency_order() {
        let mut scheduler = TaskScheduler::new();
        let order = Arc::new(Mutex::new(Vec::new()));

        let order_a = Arc::clone(&order);
        scheduler.add_task("A", vec![], true, move || {
            order_a.lock().unwrap().push("A");
        });

        let order_b = Arc::clone(&order);
        scheduler.add_task("B", vec!["A".into()], true, move || {
            order_b.lock().unwrap().push("B");
        });

        let order_c = Arc::clone(&order);
        scheduler.add_task("C", vec!["B".into()], true, move || {
            order_c.lock().unwrap().push("C");
        });

        scheduler.run();

        assert_eq!(order.lock().unwrap().as_slice(), &["A", "B", "C"]);
    }

    #[test]
    fn runs_independent_tasks_in_parallel() {
        let mut scheduler = TaskScheduler::new();
        scheduler.add_task("A", vec![], true, || {
            std::thread::sleep(Duration::from_millis(200));
        });
        scheduler.add_task("B", vec![], true, || {
            std::thread::sleep(Duration::from_millis(200));
        });

        let start = Instant::now();
        scheduler.run();
        assert!(start.elapsed() < Duration::from_millis(350));
    }
}
