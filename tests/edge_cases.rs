use engine::{Engine, config::EngineConfig};
use events::{events::{Event, GameEvent}, queue::EventPriority};

#[test]
fn event_bus_handles_many_subscribers() {
    use events::bus::EventBus;
    let bus = EventBus::new();
    for _ in 0..100 {
        bus.subscribe();
    }
    bus.broadcast(Event::Game(GameEvent::TickCompleted { tick: 1 }));
}

#[test]
fn engine_stays_healthy_under_empty_tick() {
    let (mut engine, _rx, _events) = Engine::new(EngineConfig::default());
    engine.tick().unwrap();
}
