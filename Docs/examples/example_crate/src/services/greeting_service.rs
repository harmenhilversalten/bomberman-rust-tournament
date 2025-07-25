use crate::traits::Greeter;

pub struct GreetingService<G: Greeter> {
    greeter: G,
}

impl<G: Greeter> GreetingService<G> {
    pub fn new(greeter: G) -> Self {
        Self { greeter }
    }

    pub fn send_greeting(&self, name: &str) -> String {
        self.greeter.greet(name)
    }
}
