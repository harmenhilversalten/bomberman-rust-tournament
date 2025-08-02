use std::time::Instant;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use super::{BotConfig, BotState, DecisionMaker};

/// Core bot structure coordinating decision making.
pub struct Bot<Snap, Command>
where
    Snap: Send + 'static,
    Command: Send + 'static,
{
    config: BotConfig,
    snapshot_rx: UnboundedReceiver<Snap>,
    command_tx: UnboundedSender<Command>,
    ai: Box<dyn DecisionMaker<Snap, Command>>,
    state: BotState,
}

impl<Snap, Command> Bot<Snap, Command>
where
    Snap: Send + 'static,
    Command: Send + 'static,
{
    /// Create a new [`Bot`].
    pub fn new(
        config: BotConfig,
        snapshot_rx: UnboundedReceiver<Snap>,
        command_tx: UnboundedSender<Command>,
        ai: Box<dyn DecisionMaker<Snap, Command>>,
    ) -> Self {
        Self {
            config,
            snapshot_rx,
            command_tx,
            ai,
            state: BotState::default(),
        }
    }

    /// Run the bot loop processing snapshots and emitting commands.
    ///
    /// The loop terminates when the snapshot channel closes or when sending
    /// commands fails. The final [`BotState`] is returned for inspection.
    pub async fn run(mut self) -> BotState {
        while let Some(snapshot) = self.snapshot_rx.recv().await {
            let start = Instant::now();
            let command = self.ai.decide(snapshot);
            let duration = start.elapsed();
            self.state.record_decision(duration);

            if duration > self.config.decision_timeout {
                // In future, log or handle long decision times.
            }

            if self.command_tx.send(command).is_err() {
                break;
            }
        }
        self.state
    }
}
