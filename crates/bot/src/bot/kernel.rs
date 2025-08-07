use std::{sync::Arc, thread::JoinHandle, time::Instant};

use events::{
    bus::{EventBus, EventFilter},
    events::{BotDecision, BotEvent, Event, SystemEvent},
    queue::EventPriority,
};
use state::grid::GridDelta;

use super::{BotConfig, BotState, DecisionMaker};
use crate::ai::AIDecisionPipeline;
#[cfg(feature = "rl")]
use crate::ai::RLAI;
use goals::GoalManager;
use influence::map::InfluenceMap;
use path::Pathfinder;
#[cfg(feature = "rl")]
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
    #[cfg(feature = "rl")]
    rl_policy: Option<Arc<Mutex<dyn Policy>>>,
    #[cfg(feature = "rl")]
    #[allow(dead_code)]
    value_network: Option<Arc<dyn Value>>,
    #[cfg(feature = "rl")]
    #[allow(dead_code)]
    reward_buffer: Vec<RewardRecord>,
}

/// Handle to a running bot instance allowing lifecycle control.
pub struct BotHandle {
    handle: JoinHandle<BotState>,
    events: Arc<EventBus>,
}

impl BotHandle {
    /// Stops the bot by broadcasting a shutdown event and waits for completion.
    pub fn stop(self) -> BotState {
        self.events
            .broadcast(Event::System(SystemEvent::EngineStopped));
        self.handle.join().expect("bot thread panicked")
    }
}

impl Bot {
    /// Create a new [`Bot`] referencing the shared [`EventBus`].
    pub fn new(config: BotConfig, events: Arc<EventBus>) -> Self {
        let goal_manager = Arc::new(GoalManager::new());
        let pathfinder = Arc::new(Pathfinder::new());
        let influence_map = Arc::new(Mutex::new(InfluenceMap::new(1, 1)));

        #[cfg(feature = "rl")]
        let (ai, rl_policy, value_network) =
            if config.rl_mode {
                match config
                    .rl_model_path
                    .as_ref()
                    .and_then(|p| TorchPolicy::load(std::path::Path::new(p), 4, 2).ok())
                {
                    Some(policy) => {
                        let policy_arc: Arc<Mutex<dyn Policy>> = Arc::new(Mutex::new(policy));
                        let ai: Box<dyn DecisionMaker<GridDelta, BotDecision>> = Box::new(
                            RLAI::new(Arc::clone(&policy_arc), None, config.rl_exploration_rate),
                        );
                        (ai, Some(policy_arc), None)
                    }
                    None => {
                        let ai: Box<dyn DecisionMaker<GridDelta, BotDecision>> =
                            Box::new(AIDecisionPipeline::new(
                                Arc::clone(&goal_manager),
                                Arc::clone(&pathfinder),
                                Arc::clone(&influence_map),
                            ));
                        (ai, None, None)
                    }
                }
            } else {
                let ai: Box<dyn DecisionMaker<GridDelta, BotDecision>> =
                    Box::new(AIDecisionPipeline::new(
                        Arc::clone(&goal_manager),
                        Arc::clone(&pathfinder),
                        Arc::clone(&influence_map),
                    ));
                (ai, None, None)
            };

        #[cfg(not(feature = "rl"))]
        let ai: Box<dyn DecisionMaker<GridDelta, BotDecision>> = Box::new(AIDecisionPipeline::new(
            Arc::clone(&goal_manager),
            Arc::clone(&pathfinder),
            Arc::clone(&influence_map),
        ));

        #[cfg(feature = "rl")]
        return Self {
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
        };

        #[cfg(not(feature = "rl"))]
        Self {
            config,
            events,
            ai,
            state: BotState::default(),
            goal_manager,
            pathfinder,
            influence_map,
        }
    }

    /// Spawn the bot on a new thread returning a [`BotHandle`] for control.
    pub fn spawn(self) -> BotHandle {
        let events = Arc::clone(&self.events);
        let handle = std::thread::spawn(move || self.run());
        BotHandle { handle, events }
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
    #[cfg(feature = "rl")]
    pub fn has_rl_policy(&self) -> bool {
        self.rl_policy.is_some()
    }

    #[cfg(not(feature = "rl"))]
    pub fn has_rl_policy(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use events::events::Event;
    #[cfg(feature = "rl")]
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
        let handle = bot.spawn();
        std::thread::sleep(std::time::Duration::from_millis(10));
        bus.broadcast(Event::Grid(GridDelta::None));
        let _state = handle.stop();
        bus.process();
        assert!(matches!(
            rx.try_recv().unwrap(),
            Event::Bot(BotEvent::Decision { .. })
        ));
    }

    #[cfg(feature = "rl")]
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

    #[test]
    fn spawn_returns_handle_and_stop_yields_state() {
        let bus = Arc::new(EventBus::new());
        let bot = Bot::new(
            BotConfig::new("b", crate::ai::AiType::Heuristic),
            Arc::clone(&bus),
        );
        let handle = bot.spawn();
        std::thread::sleep(std::time::Duration::from_millis(10));
        bus.broadcast(Event::Grid(GridDelta::None));
        let state = handle.stop();
        assert_eq!(state.decisions(), 1);
    }
}
