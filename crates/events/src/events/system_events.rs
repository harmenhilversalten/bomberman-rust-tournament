//! Internal system events.

/// Events emitted for internal system lifecycle changes.
#[derive(Debug, Clone, PartialEq)]
pub enum SystemEvent {
    /// The engine has started.
    EngineStarted,
    /// The engine is shutting down.
    EngineStopped,
}
