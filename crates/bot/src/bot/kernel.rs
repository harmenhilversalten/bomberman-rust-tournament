use std::{sync::Arc, time::Instant};

use events::{
    bus::{EventBus, EventFilter},
    events::{BotDecision, BotEvent, Event, SystemEvent},
    queue::EventPriority,
};
use state::grid::GridDelta;

use super::{BotConfig, BotState, DecisionMaker};
use crate::ai::{AIDecisionPipeline, RLAI};
use goals::GoalManager;
use influence::map::InfluenceMap;
use path::Pathfinder;
use rl::{Policy, RewardRecord, TorchPolicy, Value};
use std::sync::Mutex;

/// Core bot structure coordinating decision making via the event bus.
pub struct Bot {
    config: BotConfig,
    events: Arc<EventBus>,
    ai: Box<dyn DecisionMaker<GridDelta, BotDecision>>,
    state: BotState,
    #[allow(dead_code)]
    goal_manager: Arc<GoalManager>,
    #[allow(dead_code)]
    pathfinder: Arc<Pathfinder>,
    #[allow(dead_code)]
    influence_map: Arc<Mutex<InfluenceMap>>,
    rl_policy: Option<Arc<Mutex<dyn Policy>>>,
    value_network: Option<Arc<dyn Value>>,
    reward_buffer: Vec<RewardRecord>,
}

impl Bot {
    /// Create a new [`Bot`] referencing the shared [`EventBus`].
    pub fn new(config: BotConfig, events: Arc<EventBus>) -> Self {
        let goal_manager = Arc::new(GoalManager::new());
        let pathfinder = Arc::new(Pathfinder::new());
        let influence_map = Arc::new(Mutex::new(InfluenceMap::new(1, 1)));

        let (ai, rl_policy, value_network) = if config.rl_mode {
            match config
                .rl_model_path
                .as_ref()
                .and_then(|p| TorchPolicy::load(std::path::Path::new(p), 4, 2).ok())
            {
                Some(policy) => {
                    let policy_arc: Arc<Mutex<dyn Policy>> = Arc::new(Mutex::new(policy));
                    let ai = Box::new(RLAI::new(
                        Arc::clone(&policy_arc),
                        None,
                        config.rl_exploration_rate,
                    ));
                    (ai, Some(policy_arc), None)
                }
                None => {
                    let ai = Box::new(AIDecisionPipeline::new(
                        Arc::clone(&goal_manager),
                        Arc::clone(&pathfinder),
                        Arc::clone(&influence_map),
                    ));
                    (ai, None, None)
                }
            }
        } else {
            let ai = Box::new(AIDecisionPipeline::new(
                Arc::clone(&goal_manager),
                Arc::clone(&pathfinder),
                Arc::clone(&influence_map),
            ));
            (ai, None, None)
        };

        Self {
            config,
            events,
            ai,
            state: BotState::default(),
            goal_manager,
            pathfinder,
            influence_map,
            rl_policy,
            value_network,
            reward_buffer: Vec::new(),
        }
    }

    /// Run the bot loop processing `GridDelta` events and emitting commands.
    ///
    /// The loop terminates when the event bus is dropped. The final [`BotState`] is returned.
    pub fn run(mut self) -> BotState {
        let filter = EventFilter::new(|e| matches!(e, Event::Grid(_) | Event::System(_)));
        let (_id, rx) = self.events.subscribe_with_filter(Some(filter));
        while let Ok(event) = rx.recv() {
            match event {
                Event::Grid(delta) => {
                    let start = Instant::now();
                    let decision = self.ai.decide(delta);
                    let duration = start.elapsed();
                    self.state.record_decision(duration);
                    if duration > self.config.decision_timeout {
                        // In future, log or handle long decision times.
                    }
                    self.events.emit(
                        Event::Bot(BotEvent::Decision {
                            bot_id: self.config.id,
                            decision,
                        }),
                        EventPriority::Normal,
                    );
                }
                Event::System(SystemEvent::EngineStopped) => break,
                _ => {}
            }
        }
        self.state
    }

    /// Returns true if an RL policy is loaded.
    pub fn has_rl_policy(&self) -> bool {
        self.rl_policy.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use events::events::Event;
    use tempfile::tempdir;

    #[test]
    fn bot_emits_decision_on_grid_event() {
        let bus = Arc::new(EventBus::new());
        let filter = EventFilter::new(|e| matches!(e, Event::Bot(_)));
        let (_id, rx) = bus.subscribe_with_filter(Some(filter));
        let bot = Bot::new(
            BotConfig::new("b", crate::ai::AiType::Heuristic),
            Arc::clone(&bus),
        );
        let handle = std::thread::spawn(move || bot.run());
        std::thread::sleep(std::time::Duration::from_millis(10));
        bus.broadcast(Event::Grid(GridDelta::None));
        bus.broadcast(Event::System(SystemEvent::EngineStopped));
        let _ = handle.join();
        bus.process();
        assert!(matches!(
            rx.try_recv().unwrap(),
            Event::Bot(BotEvent::Decision { .. })
        ));
    }

    #[test]
    fn bot_loads_rl_policy_when_enabled() {
        let dir = tempdir().unwrap();
        let model_path = dir.path().join("policy.ot");
        let policy = rl::TorchPolicy::new(4, 2);
        policy.save(&model_path).unwrap();
        let mut cfg = BotConfig::new("b", crate::ai::AiType::Heuristic);
        cfg.rl_mode = true;
        cfg.rl_model_path = Some(model_path.to_string_lossy().into());
        let bus = Arc::new(EventBus::new());
        let bot = Bot::new(cfg, Arc::clone(&bus));
        assert!(bot.has_rl_policy());
    }
}
