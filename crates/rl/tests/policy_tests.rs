use rl::policy::{Policy, RandomPolicy, TorchPolicy};
use rl::types::Observation;

#[test]
fn random_policy_action_within_bounds() {
    let mut policy = RandomPolicy::new(4);
    let obs = Observation::new(vec![0.0; 2]);
    let action = policy.select_action(&obs).unwrap();
    assert!((0..4).contains(&action));
}

#[test]
fn torch_policy_loading_preserves_output() {
    let mut policy = TorchPolicy::new(2, 2);
    let obs = Observation::new(vec![0.1, 0.2]);
    let before = policy.select_action(&obs).unwrap();
    let path = std::env::temp_dir().join("policy_integration_test.ot");
    policy.save(&path).unwrap();

    let mut loaded = TorchPolicy::new(2, 2);
    loaded.load(&path).unwrap();
    let after = loaded.select_action(&obs).unwrap();
    assert_eq!(before, after);
    let _ = std::fs::remove_file(path);
}
