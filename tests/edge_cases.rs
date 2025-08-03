use bot::{AiType, BotConfig};
use events::EventBus;
use events::events::{Event, GameEvent};

mod error_handling {
    use super::*;

    #[test]
    fn test_event_bus_handles_many_subscribers() {
        let bus = EventBus::new();
        let mut receivers = Vec::new();
        for _ in 0..100 {
            let (_id, rx) = bus.subscribe();
            receivers.push(rx);
        }
        bus.broadcast(Event::Game(GameEvent::TickCompleted { tick: 1 }));
        for rx in receivers {
            assert!(rx.try_recv().is_ok());
        }
    }

    #[test]
    fn test_bot_config_rejects_empty_name() {
        let cfg = BotConfig::new("", AiType::Heuristic);
        assert!(cfg.validate().is_err());
    }
}
