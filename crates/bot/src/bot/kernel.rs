use std::{sync::Arc, thread::JoinHandle, time::Instant};

use events::{
    bus::{EventBus, EventFilter},
    events::{BotDecision, BotEvent, Event, SystemEvent},
    queue::EventPriority,
};
use state::grid::GridDelta;

use super::{BotConfig, BotState, DecisionMaker};
use crate::ai::AIDecisionPipeline;

use goals::GoalManager;
use influence::map::InfluenceMap;
use path::Pathfinder;
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
    pathfinder: Arc<std::sync::Mutex<Pathfinder>>,
    #[allow(dead_code)]
    influence_map: Arc<Mutex<InfluenceMap>>,
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
        let pathfinder = Arc::new(std::sync::Mutex::new(Pathfinder::new()));
        let influence_map = Arc::new(Mutex::new(InfluenceMap::new(1, 1)));

        let ai: Box<dyn DecisionMaker<GridDelta, BotDecision>> = Box::new(AIDecisionPipeline::new(
            Arc::clone(&goal_manager),
            Arc::clone(&pathfinder),
            Arc::clone(&influence_map),
        ));

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
                    // Emit status if available
                    if let Some(status) = self.ai.status() {
                        self.events.emit(
                            Event::Bot(BotEvent::Status { bot_id: self.config.id, status }),
                            EventPriority::Low,
                        );
                    }
                    self.events.emit(
                        Event::Bot(BotEvent::Decision {
                            bot_id: self.config.id,
                            decision,
                        }),
                        EventPriority::Normal,
                    );
                }
                Event::System(SystemEvent::EngineStopped) => {
                    break;
                }
                _ => {
                    // Ignore unexpected events
                }
            }
        }
        self.state
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use events::events::Event;


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


