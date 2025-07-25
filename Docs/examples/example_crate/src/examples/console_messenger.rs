use crate::traits::Messenger;

/// Simple `Messenger` implementation that prints to stdout.
#[derive(Default)]
pub struct ConsoleMessenger;

impl Messenger for ConsoleMessenger {
    fn send_message(&self, msg: &str) {
        println!("{}", msg);
    }
}
