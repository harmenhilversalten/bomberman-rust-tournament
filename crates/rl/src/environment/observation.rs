//! Definitions of observation and action spaces.

/// Description of the observation vector shape.
#[derive(Debug, Clone, Copy)]
pub struct ObservationSpace {
    /// Size of the observation vector.
    pub size: usize,
}

/// Discrete action space description.
#[derive(Debug, Clone, Copy)]
pub struct ActionSpace {
    /// Number of discrete actions available.
    pub actions: i64,
}
