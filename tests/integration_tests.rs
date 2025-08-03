use engine::{Engine, config::EngineConfig};
use events::{events::{Event, GameEvent, BotEvent, BotDecision}, queue::EventPriority};
use state::grid::GridDelta;

#[test]
fn test_event_broadcast_and_reception() {
    let (mut engine, _rx, events) = Engine::new(EngineConfig::default());
    let (_id, rx) = events.subscribe();
    engine.tick().unwrap();
    let event = rx.try_recv().unwrap();
    assert!(matches!(event, Event::Game(GameEvent::TickCompleted { .. })));
}

#[test]
fn test_bot_command_processing() {
    let cfg = EngineConfig { width: 1, height: 1, ..EngineConfig::default() };
    let (mut engine, mut rx, events) = Engine::new(cfg);
    events.emit(
        Event::Bot(BotEvent::Decision { bot_id: 0, decision: BotDecision::PlaceBomb }),
        EventPriority::Normal,
    );
    engine.tick().unwrap();
    assert!(matches!(rx.borrow_and_update().clone(), GridDelta::AddBomb(_)));
}
