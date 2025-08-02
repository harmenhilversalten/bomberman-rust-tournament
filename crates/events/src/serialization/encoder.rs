//! JSON encoder for events and transitions.

use crate::{events::Event, serialization::Transition};

/// Encodes an [`Event`] into a JSON string.
pub fn encode_event(event: &Event) -> serde_json::Result<String> {
    serde_json::to_string(event)
}

/// Encodes a [`Transition`] into a JSON string.
pub fn encode_transition(transition: &Transition) -> serde_json::Result<String> {
    serde_json::to_string(transition)
}
