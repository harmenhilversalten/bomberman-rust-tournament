//! Countdown timer for bombs.

/// Simple countdown timer operating on game ticks.
#[derive(Debug, Clone, Copy)]
pub struct BombTimer {
    remaining: u8,
}

impl BombTimer {
    /// Creates a new timer with the given `duration` in ticks.
    pub fn new(duration: u8) -> Self {
        Self {
            remaining: duration,
        }
    }

    /// Advances the timer by one tick. Returns `true` if the timer has expired.
    pub fn tick(&mut self) -> bool {
        if self.remaining > 0 {
            self.remaining -= 1;
        }
        self.remaining == 0
    }

    /// Returns the number of ticks remaining until detonation.
    pub fn remaining(&self) -> u8 {
        self.remaining
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_down_to_zero() {
        let mut timer = BombTimer::new(2);
        assert_eq!(timer.remaining(), 2);
        assert!(!timer.tick());
        assert_eq!(timer.remaining(), 1);
        assert!(timer.tick());
        assert_eq!(timer.remaining(), 0);
    }
}
