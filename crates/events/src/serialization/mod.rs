//! Serialization utilities for events and RL transitions.

use serde::{Deserialize, Serialize};

pub mod decoder;
pub mod encoder;

/// RL transition record used for learning.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transition {
    /// Observation before taking an action.
    pub obs: Vec<f32>,
    /// Action taken by the agent.
    pub action: i32,
    /// Reward obtained after the action.
    pub reward: f32,
    /// Observation after taking the action.
    pub next_obs: Vec<f32>,
    /// Whether the episode has terminated.
    pub done: bool,
}

/// Recorder that accumulates transitions during gameplay.
#[derive(Debug, Default)]
pub struct TransitionRecorder {
    transitions: Vec<Transition>,
}

impl TransitionRecorder {
    /// Creates a new, empty recorder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a transition constructed from components.
    pub fn record(
        &mut self,
        obs: Vec<f32>,
        action: i32,
        reward: f32,
        next_obs: Vec<f32>,
        done: bool,
    ) {
        let transition = Transition {
            obs,
            action,
            reward,
            next_obs,
            done,
        };
        self.transitions.push(transition);
    }

    /// Returns all recorded transitions.
    pub fn transitions(&self) -> &[Transition] {
        &self.transitions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{Event, GameEvent};

    #[test]
    fn event_serialization_round_trip() {
        let event = Event::Game(GameEvent::TickCompleted { tick: 7 });
        let json = encoder::encode_event(&event).unwrap();
        let decoded = decoder::decode_event(&json).unwrap();
        assert_eq!(event, decoded);
    }

    #[test]
    fn transition_record_and_round_trip() {
        let mut recorder = TransitionRecorder::new();
        recorder.record(vec![0.1, 0.2], 1, 0.5, vec![0.3, 0.4], true);
        let transition = &recorder.transitions()[0];
        let json = encoder::encode_transition(transition).unwrap();
        let decoded = decoder::decode_transition(&json).unwrap();
        assert_eq!(*transition, decoded);
    }
}
