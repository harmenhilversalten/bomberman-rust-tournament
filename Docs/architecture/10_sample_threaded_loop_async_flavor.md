## 10. Sample Threaded Loop (Async Flavor)

### 10.1 Main Simulation and Bot Spawning

The main simulation loop, particularly when using an asynchronous runtime like Tokio, orchestrates the game ticks and manages the lifecycle of AI agents. The loop begins by initializing a shared GameGrid, typically wrapped in an Arc for thread-safe reference counting. A watch channel is created to broadcast GridDelta messages from the game engine to the bot tasks. A JoinSet, from Tokio, is used to manage the asynchronous tasks spawned for each bot. For each AI agent (from 0 to NUM_BOTS), a new asynchronous task is spawned. Within this task, a Bot instance is created, passing its Id and a clone of the Arc<GameGrid>. The task also clones the Receiver end of the watch channel to receive state updates. This setup ensures that each bot operates independently in its own execution context, processing state updates and making decisions concurrently.

```rust
use bomberman_state::GameGrid;
use bomberman_engine::{Engine, GridDelta};
// (rest of code identical, referencing engine.tick() instead of grid.tick())

#[tokio::main]
async fn main() {
    let (mut engine, rx) = Engine::new(256); // e.g., 256x256 grid
    let mut set = tokio::task::JoinSet::new();
    for id in 0..NUM_BOTS {
        let rx = rx.clone();
        let grid = engine.grid.clone();
        set.spawn(async move {
            let mut bot = Bot::new(id, grid);
            while let Ok(delta) = rx.changed().await {
                bot.tick(&delta).await;
            }
        });
    }
    loop {
        engine.tick();
        // Sleep for ~16ms or use a timer
    }
}
```

### 10.2 Bot Decision-Making Loop with RL Switching

Inside each bot's asynchronous task, after initialization, the bot enters its main decision-making loop. This loop typically waits for a change notification from the watch channel (rx.changed().await) indicating that a new game tick has occurred and a GridDelta is available. Upon receiving an update, the bot calls its tick() method. The Bot struct, as described in section 5.3, contains logic to switch between programmatic AI and RL-based decision-making. If the bot config.rl_mode flag is set to true, the bot would first ensure its RL policy is loaded (e.g., bot.load_rl_policy("model.pt"))? Then during its tick, it would delegate the decision to an RL-specific method, such as kernel.rl_tick(&snapshot).await. This method would use the loaded neural network policy to select an action based on the current game snapshot. If RL mode is false, the kernel.tick(&snapshot).await method would be used, employing the programmatic AI logic. The chosen action is then submitted back to the game engine (e.g., via grid.submit_command(cmd)). This design allows for flexible switching between AI paradigms, facilitating development, testing, and hybrid approaches where an agent might use programmatic logic as a fallback or for specific sub-tasks while primarily relying on learned behaviors.

```rust
// Inside the bot's spawned async block
let mut bot = Bot::new(id, grid);
if bot.config.rl_mode {
    bot.load_rl_policy("model.pt"); // e.g., Torch model
}
loop {
    let _ = rx.changed().await;
    bot.tick().await;
}
```

---

