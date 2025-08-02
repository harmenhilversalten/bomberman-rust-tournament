//! Processed view of a snapshot used by the AI.

/// Minimal observation derived from a raw snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Observation {
    /// Value extracted from the snapshot.
    pub value: i32,
}

impl Observation {
    /// Create an [`Observation`] from the raw snapshot data.
    pub fn from_snapshot(snapshot: i32) -> Self {
        Self { value: snapshot }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observation_wraps_snapshot_value() {
        let obs = Observation::from_snapshot(7);
        assert_eq!(obs.value, 7);
    }
}
