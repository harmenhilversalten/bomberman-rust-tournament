use engine::bot::random_bot::RandomBot;
use engine::game::Game;
use state::init as state_init;
use influence::init as influence_init;
use path::init as path_init;
use goals::init as goals_init;
use bombs::init as bombs_init;
use bot::init as bot_init;
use events::init as events_init;
use rl::init as rl_init;
use test_utils::init as test_utils_init;
use ffi::init as ffi_init;

#[test]
fn crates_compile_and_init() {
    assert_eq!(state_init(), "initialized");
    assert_eq!(influence_init(), "initialized");
    assert_eq!(path_init(), "initialized");
    assert_eq!(goals_init(), "initialized");
    assert_eq!(bombs_init(), "initialized");
    assert_eq!(bot_init(), "initialized");
    assert_eq!(events_init(), "initialized");
    assert_eq!(rl_init(), "initialized");
    assert_eq!(test_utils_init(), "initialized");
    assert_eq!(ffi_init(), "initialized");

    let bot1 = Box::new(RandomBot::new("Bot1".to_string()));
    let bot2 = Box::new(RandomBot::new("Bot2".to_string()));
    let mut game = Game::build(7, 7, vec![bot1, bot2]);
    assert!(game.winner.is_none());
}
