use example_crate::config::Config;

#[test]
fn load_default_and_env() {
    std::env::remove_var("GREETING_PREFIX");
    let cfg = Config::load().unwrap();
    assert_eq!(cfg.prefix, "Hello");
    std::env::set_var("GREETING_PREFIX", "Hey");
    let cfg = Config::load().unwrap();
    assert_eq!(cfg.prefix, "Hey");
}

#[test]
fn invalid_prefix_errors() {
    std::env::set_var("GREETING_PREFIX", "");
    assert!(Config::load().is_err());
    std::env::set_var("GREETING_PREFIX", "x".repeat(40));
    assert!(Config::load().is_err());
}
