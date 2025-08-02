//! Event bus with queuing and filtering.

use std::sync::{
    Mutex,
    atomic::{AtomicU32, Ordering},
};

use crossbeam::channel::{Receiver, Sender, unbounded};

use crate::{
    events::Event,
    queue::{EventPriority, EventQueue},
};

use super::{EventFilter, SubscriberId};

struct Subscriber {
    #[allow(dead_code)]
    id: SubscriberId,
    tx: Sender<Event>,
    filter: Option<EventFilter>,
}

/// Event bus capable of broadcasting events to subscribers.
pub struct EventBus {
    subscribers: Mutex<Vec<Subscriber>>,
    next_id: AtomicU32,
    queue: EventQueue,
}

impl EventBus {
    /// Creates a new, empty event bus.
    pub fn new() -> Self {
        Self {
            subscribers: Mutex::new(Vec::new()),
            next_id: AtomicU32::new(1),
            queue: EventQueue::new(),
        }
    }

    /// Registers a new subscriber without a filter and returns its ID and receiver.
    pub fn subscribe(&self) -> (SubscriberId, Receiver<Event>) {
        self.subscribe_with_filter(None)
    }

    /// Registers a new subscriber with an optional filter.
    pub fn subscribe_with_filter(
        &self,
        filter: Option<EventFilter>,
    ) -> (SubscriberId, Receiver<Event>) {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = unbounded();
        let mut subscribers = self.subscribers.lock().expect("lock poisoned");
        subscribers.push(Subscriber { id, tx, filter });
        (id, rx)
    }

    /// Enqueues an event with a priority to be processed later.
    pub fn emit(&self, event: Event, priority: EventPriority) {
        self.queue.push(event, priority);
    }

    /// Processes all queued events, delivering them to subscribers.
    pub fn process(&self) -> usize {
        let mut count = 0;
        while let Some(event) = self.queue.pop() {
            self.broadcast(event);
            count += 1;
        }
        count
    }

    /// Broadcasts an event immediately to all matching subscribers.
    pub fn broadcast(&self, event: Event) {
        let subscribers = self.subscribers.lock().expect("lock poisoned");
        for subscriber in subscribers.iter() {
            if subscriber.filter.as_ref().is_none_or(|f| f.matches(&event)) {
                let _ = subscriber.tx.send(event.clone());
            }
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{BotDecision, BotEvent, GameEvent};

    #[test]
    fn broadcasts_events_to_all_subscribers() {
        let bus = EventBus::new();
        let (_id1, rx1) = bus.subscribe();
        let (_id2, rx2) = bus.subscribe();

        bus.broadcast(Event::Game(GameEvent::TickCompleted { tick: 1 }));

        assert_eq!(
            rx1.try_recv().unwrap(),
            Event::Game(GameEvent::TickCompleted { tick: 1 })
        );
        assert_eq!(
            rx2.try_recv().unwrap(),
            Event::Game(GameEvent::TickCompleted { tick: 1 })
        );
    }

    #[test]
    fn processes_events_by_priority() {
        let bus = EventBus::new();
        let (_id, rx) = bus.subscribe();

        bus.emit(
            Event::Game(GameEvent::TickCompleted { tick: 1 }),
            EventPriority::Low,
        );
        bus.emit(
            Event::Game(GameEvent::TickCompleted { tick: 2 }),
            EventPriority::High,
        );
        bus.process();

        assert_eq!(
            rx.try_recv().unwrap(),
            Event::Game(GameEvent::TickCompleted { tick: 2 })
        );
        assert_eq!(
            rx.try_recv().unwrap(),
            Event::Game(GameEvent::TickCompleted { tick: 1 })
        );
    }

    #[test]
    fn filters_events_for_subscribers() {
        let bus = EventBus::new();
        let filter = EventFilter::new(|e| matches!(e, Event::Game(_)));
        let (_id, rx) = bus.subscribe_with_filter(Some(filter));

        bus.emit(
            Event::Game(GameEvent::TickCompleted { tick: 3 }),
            EventPriority::Normal,
        );
        bus.emit(
            Event::Bot(BotEvent::Decision {
                bot_id: 1,
                decision: BotDecision::Wait,
            }),
            EventPriority::Normal,
        );
        bus.process();

        assert_eq!(
            rx.try_recv().unwrap(),
            Event::Game(GameEvent::TickCompleted { tick: 3 })
        );
        assert!(rx.try_recv().is_err());
    }
}
