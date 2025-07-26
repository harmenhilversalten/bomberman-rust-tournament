use example_crate::{config::Config, container::Container};

#[test]
fn container_provides_processor() {
    let c = Container::new(Config::new("svc"));
    let p = c.greeter_processor();
    let msg = p.process("Bob").unwrap();
    assert!(msg.starts_with("Hello, Bob!"));
}
