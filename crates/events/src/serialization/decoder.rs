//! JSON decoder for events and transitions.

use crate::{events::Event, serialization::Transition};

/// Decodes an [`Event`] from a JSON string.
pub fn decode_event(json: &str) -> serde_json::Result<Event> {
    serde_json::from_str(json)
}

/// Decodes a [`Transition`] from a JSON string.
pub fn decode_transition(json: &str) -> serde_json::Result<Transition> {
    serde_json::from_str(json)
}
