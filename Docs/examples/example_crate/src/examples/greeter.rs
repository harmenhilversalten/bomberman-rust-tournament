use crate::traits::Messenger;

/// Greeter uses a `Messenger` to greet users.
pub struct Greeter<'a, M: Messenger> {
    messenger: &'a M,
}

impl<'a, M: Messenger> Greeter<'a, M> {
    /// Create a new `Greeter` with the given messenger.
    pub fn new(messenger: &'a M) -> Self {
        Self { messenger }
    }

    /// Greet a user by name.
    pub fn greet(&self, name: &str) {
        let msg = format!("Hello, {name}!");
        self.messenger.send_message(&msg);
    }
}
