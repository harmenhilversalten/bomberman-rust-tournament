//! Simple event bus implementation.

use std::sync::{
    Mutex,
    atomic::{AtomicU32, Ordering},
};

use crossbeam::channel::{Receiver, Sender, unbounded};

use crate::events::Event;

use super::subscriber::SubscriberId;

/// Event bus capable of broadcasting events to subscribers.
pub struct EventBus {
    subscribers: Mutex<Vec<(SubscriberId, Sender<Event>)>>,
    next_id: AtomicU32,
}

impl EventBus {
    /// Creates a new, empty event bus.
    pub fn new() -> Self {
        Self {
            subscribers: Mutex::new(Vec::new()),
            next_id: AtomicU32::new(1),
        }
    }

    /// Registers a new subscriber and returns its ID and receiver.
    pub fn subscribe(&self) -> (SubscriberId, Receiver<Event>) {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = unbounded();
        let mut subscribers = self.subscribers.lock().expect("lock poisoned");
        subscribers.push((id, tx));
        (id, rx)
    }

    /// Broadcasts an event to all subscribers.
    pub fn broadcast(&self, event: Event) {
        let subscribers = self.subscribers.lock().expect("lock poisoned");
        for (_, tx) in subscribers.iter() {
            let _ = tx.send(event.clone());
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
    use crate::events::GameEvent;

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
}
