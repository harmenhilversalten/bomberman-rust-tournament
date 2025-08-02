use crossbeam::queue::SegQueue;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::events::Event;

/// Priority levels for events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventPriority {
    /// High priority events are processed first.
    High,
    /// Normal priority events are processed after high priority.
    Normal,
    /// Low priority events are processed last.
    Low,
}

/// Thread-safe queue holding events across three priority levels.
pub struct EventQueue {
    high: SegQueue<Event>,
    normal: SegQueue<Event>,
    low: SegQueue<Event>,
    pending: AtomicUsize,
}

impl EventQueue {
    /// Creates an empty event queue.
    pub fn new() -> Self {
        Self {
            high: SegQueue::new(),
            normal: SegQueue::new(),
            low: SegQueue::new(),
            pending: AtomicUsize::new(0),
        }
    }

    /// Pushes an event with the specified priority.
    pub fn push(&self, event: Event, priority: EventPriority) {
        match priority {
            EventPriority::High => self.high.push(event),
            EventPriority::Normal => self.normal.push(event),
            EventPriority::Low => self.low.push(event),
        }
        self.pending.fetch_add(1, Ordering::Relaxed);
    }

    /// Pops the next event in priority order.
    pub fn pop(&self) -> Option<Event> {
        let event = self
            .high
            .pop()
            .or_else(|| self.normal.pop())
            .or_else(|| self.low.pop());

        if event.is_some() {
            self.pending.fetch_sub(1, Ordering::Relaxed);
        }
        event
    }

    /// Returns the number of pending events.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.pending.load(Ordering::Relaxed)
    }

    /// Returns true if the queue has no events.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.pending.load(Ordering::Relaxed) == 0
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::GameEvent;

    #[test]
    fn pops_high_priority_first() {
        let queue = EventQueue::new();
        queue.push(
            Event::Game(GameEvent::TickCompleted { tick: 1 }),
            EventPriority::Low,
        );
        queue.push(
            Event::Game(GameEvent::TickCompleted { tick: 2 }),
            EventPriority::High,
        );

        assert_eq!(
            queue.pop(),
            Some(Event::Game(GameEvent::TickCompleted { tick: 2 }))
        );
        assert_eq!(
            queue.pop(),
            Some(Event::Game(GameEvent::TickCompleted { tick: 1 }))
        );
        assert!(queue.pop().is_none());
    }
}
