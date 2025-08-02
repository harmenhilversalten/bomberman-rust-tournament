//! Internal system events.

use serde::{Deserialize, Serialize};

/// Events emitted for internal system lifecycle changes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SystemEvent {
    /// The engine has started.
    EngineStarted,
    /// The engine is shutting down.
    EngineStopped,
}
