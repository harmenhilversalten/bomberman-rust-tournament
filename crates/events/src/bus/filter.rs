use std::sync::Arc;

use crate::events::Event;

/// Filter applied to subscriber delivery.
#[derive(Clone)]
pub struct EventFilter {
    predicate: Arc<dyn Fn(&Event) -> bool + Send + Sync>,
}

impl EventFilter {
    /// Creates a new filter from the given predicate.
    pub fn new<F>(predicate: F) -> Self
    where
        F: Fn(&Event) -> bool + Send + Sync + 'static,
    {
        Self {
            predicate: Arc::new(predicate),
        }
    }

    /// Returns true if the event matches the filter.
    pub fn matches(&self, event: &Event) -> bool {
        (self.predicate)(event)
    }
}
