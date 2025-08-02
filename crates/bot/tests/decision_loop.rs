use bot::{Bot, BotConfig, DecisionMaker};
use tokio::sync::mpsc;

struct DummyAI;

impl DecisionMaker<i32, i32> for DummyAI {
    fn decide(&mut self, snapshot: i32) -> i32 {
        snapshot + 1
    }
}

#[tokio::test]
async fn bot_processes_snapshots_and_sends_commands() {
    let (snap_tx, snap_rx) = mpsc::unbounded_channel();
    let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel();

    let bot = Bot::new(BotConfig::default(), snap_rx, cmd_tx, Box::new(DummyAI));

    let handle = tokio::spawn(bot.run());

    snap_tx.send(1).unwrap();
    snap_tx.send(41).unwrap();
    drop(snap_tx);

    let first = cmd_rx.recv().await.unwrap();
    let second = cmd_rx.recv().await.unwrap();

    assert_eq!(first, 2);
    assert_eq!(second, 42);

    let state = handle.await.unwrap();
    assert_eq!(state.decisions(), 2);
}
