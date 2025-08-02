use std::{sync::Arc, time::Instant};

use crossbeam::channel::Receiver;
use events::{
    bus::{EventBus, EventFilter},
    events::{BotDecision, BotEvent, Event, SystemEvent},
};
use state::grid::GridDelta;

use super::{BotConfig, BotState, DecisionMaker};

/// Core bot structure coordinating decision making via the event bus.
pub struct Bot {
    config: BotConfig,
    events: Arc<EventBus>,
    event_rx: Receiver<Event>,
    ai: Box<dyn DecisionMaker<GridDelta, BotDecision>>,
    state: BotState,
}

impl Bot {
    /// Create a new [`Bot`] subscribing to [`GridDelta`] events.
    pub fn new(
        config: BotConfig,
        events: Arc<EventBus>,
        ai: Box<dyn DecisionMaker<GridDelta, BotDecision>>,
    ) -> Self {
        let filter = EventFilter::new(|e| matches!(e, Event::Grid(_)));
        let (_id, rx) = events.subscribe_with_filter(Some(filter));
        Self {
            config,
            events,
            event_rx: rx,
            ai,
            state: BotState::default(),
        }
    }

    /// Run the bot loop processing `GridDelta` events and emitting commands.
    ///
    /// The loop terminates when the event bus is dropped. The final [`BotState`] is returned.
    pub fn run(mut self) -> BotState {
        while let Ok(event) = self.event_rx.recv() {
            match event {
                Event::Grid(delta) => {
                    let start = Instant::now();
                    let decision = self.ai.decide(delta);
                    let duration = start.elapsed();
                    self.state.record_decision(duration);
                    if duration > self.config.decision_timeout {
                        // In future, log or handle long decision times.
                    }
                    self.events.broadcast(Event::Bot(BotEvent::Decision {
                        bot_id: self.config.id as usize,
                        decision,
                    }));
                }
                Event::System(SystemEvent::EngineStopped) => break,
                _ => {}
            }
        }
        self.state
    }
}
