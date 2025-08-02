use bot::{
    Bot, BotConfig, DecisionMaker,
    ai::{HeuristicAI, PlanningAI, ReactiveAI},
};
use tokio::sync::mpsc;

async fn run_bot_with_ai<A: DecisionMaker<i32, i32> + 'static>(ai: A, snapshot: i32) -> i32 {
    let (snap_tx, snap_rx) = mpsc::unbounded_channel();
    let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel();

    let bot = Bot::new(BotConfig::default(), snap_rx, cmd_tx, Box::new(ai));
    let handle = tokio::spawn(bot.run());

    snap_tx.send(snapshot).unwrap();
    drop(snap_tx);

    let cmd = cmd_rx.recv().await.unwrap();
    drop(cmd_rx);

    handle.await.unwrap();
    cmd
}

#[tokio::test]
async fn heuristic_ai_produces_incremented_command() {
    let command = run_bot_with_ai(HeuristicAI, 1).await;
    assert_eq!(command, 2);
}

#[tokio::test]
async fn reactive_ai_produces_same_command() {
    let command = run_bot_with_ai(ReactiveAI, 7).await;
    assert_eq!(command, 7);
}

#[tokio::test]
async fn planning_ai_produces_decremented_command() {
    let command = run_bot_with_ai(PlanningAI, 3).await;
    assert_eq!(command, 2);
}
