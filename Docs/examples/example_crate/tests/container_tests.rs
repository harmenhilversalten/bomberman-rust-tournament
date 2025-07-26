use example_crate::adapters::UuidStringAdapter;
use example_crate::{config::Config, container::Container};

#[test]
fn container_provides_processor() {
    let c = Container::new(Config::new("svc"));
    let p = c.greeter_processor();
    let msg = p.process("Bob").unwrap();
    assert!(msg.starts_with("Hello, Bob!"));
}

#[test]
fn container_provides_adapter() {
    let c = Container::new(Config::new("svc"));
    let a = c.uuid_string_adapter();
    let id = a.uuid_string().unwrap();
    assert_eq!(id.parse::<uuid::Uuid>().unwrap().get_version_num(), 4);
}
