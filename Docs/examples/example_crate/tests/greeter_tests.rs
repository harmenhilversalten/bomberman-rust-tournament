use example_crate::examples::{Greeter};
use example_crate::traits::Messenger;

#[derive(Default)]
struct MockMessenger {
    sent: std::sync::Mutex<Vec<String>>,
}

impl Messenger for MockMessenger {
    fn send_message(&self, msg: &str) {
        self.sent.lock().unwrap().push(msg.to_string());
    }
}

#[test]
fn greet_sends_message() {
    let messenger = MockMessenger::default();
    let greeter = Greeter::new(&messenger);
    greeter.greet("Alice");
    let messages = messenger.sent.lock().unwrap();
    assert_eq!(messages.as_slice(), ["Hello, Alice!"]);
}
