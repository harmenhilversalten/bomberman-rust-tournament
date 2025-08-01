//! Bomb component with timing and properties.

/// Live bomb placed on the grid.
#[derive(Debug, Clone)]
pub struct Bomb {
    /// Identifier of the owner agent.
    pub owner: usize,
    /// Bomb position on the grid.
    pub position: (u16, u16),
    /// Ticks until the bomb explodes.
    pub timer: u8,
    /// Blast radius.
    pub power: u8,
    /// Whether the bomb's blast pierces obstacles.
    pub pierce: bool,
    /// Whether the bomb can be detonated remotely.
    pub remote: bool,
}

impl Bomb {
    /// Creates a new bomb instance.
    pub fn new(owner: usize, position: (u16, u16), timer: u8, power: u8) -> Self {
        Self {
            owner,
            position,
            timer,
            power,
            pierce: false,
            remote: false,
        }
    }

    /// Advances the timer by one tick.
    pub fn tick(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }
    }

    /// Returns true if the bomb should explode.
    pub fn is_exploding(&self) -> bool {
        self.timer == 0
    }
}
