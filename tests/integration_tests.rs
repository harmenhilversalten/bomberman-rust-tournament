use bot::{AiType, BotConfig};
use engine::{config::EngineConfig, Engine};
use events::events::{BotDecision, BotEvent, Event, GameEvent};
use events::queue::EventPriority;
use state::grid::GridDelta;

mod event_bus_integration {
    use super::*;

    #[test]
    fn test_event_broadcast_and_reception() {
        let (mut engine, _rx, events) = Engine::new(EngineConfig::default());
        let (_id, rx) = events.subscribe();
        engine.tick().unwrap();
        let event = rx.try_recv().unwrap();
        assert!(matches!(event, Event::Game(GameEvent::TickCompleted { .. })));
    }

    #[test]
    fn test_multi_component_event_flow() {
        let (mut engine, _rx, events) = Engine::new(EngineConfig::default());
        let cfg = BotConfig::new("bot", AiType::Heuristic);
        let bot_id = engine.spawn_bot(cfg).unwrap();
        events.emit(
            Event::Bot(BotEvent::Decision { bot_id, decision: BotDecision::Wait }),
            EventPriority::Normal,
        );
        assert!(engine.tick().is_ok());
    }
}

mod bot_engine_integration {
    use super::*;

    #[test]
    fn test_bot_command_processing() {
        let (mut engine, mut rx, events) = Engine::new(EngineConfig::default());
        events.emit(
            Event::Bot(BotEvent::Decision { bot_id: 1, decision: BotDecision::PlaceBomb }),
            EventPriority::Normal,
        );
        engine.tick().unwrap();
        assert!(matches!(rx.borrow_and_update().clone(), GridDelta::AddBomb(_)));
    }

    #[test]
    fn test_multiple_bots_interaction() {
        let (mut engine, _rx, _events) = Engine::new(EngineConfig::default());
        let id1 = engine.spawn_bot(BotConfig::new("b1", AiType::Heuristic)).unwrap();
        let id2 = engine.spawn_bot(BotConfig::new("b2", AiType::Heuristic)).unwrap();
        assert_ne!(id1, id2);
    }
}
