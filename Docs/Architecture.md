# Bomberman AI Agent Architecture: A High-Performance Rust Reference Implementation
*(Updated: `grid` split into `state` + `engine`; all references adjusted)*

---

## 1. Executive Summary

### 1.1 Overview of AI Agent Design

This document outlines a comprehensive architecture for an AI agent designed for a Bomberman-inspired game, leveraging the Rust programming language for its performance, safety, and concurrency features. The primary goal of this architecture is to provide a robust and scalable foundation for developing intelligent agents capable of competing effectively in a dynamic game environment. The design emphasizes a modular structure, allowing for independent development and testing of various AI components such as pathfinding, goal management, and bomb planning. A key aspect of the architecture is its ability to support both traditional programmatic AI, based on predefined rules and heuristics, and more advanced machine learning approaches, particularly reinforcement learning (RL). The system is engineered to handle large-scale game environments, targeting performance at 60 Hz on expansive 256x256 grids with potentially hundreds of bots, ensuring that decision-making processes are both rapid and efficient. This high-level overview sets the stage for a detailed exploration of the individual components and their interactions, which are critical for achieving the desired performance and flexibility.

### 1.2 Key Enhancements and Focus Areas

This revised version of the AI agent architecture incorporates several key enhancements aimed at improving modularity, robustness, and readiness for reinforcement learning. A significant focus has been placed on ensuring a clean separation of concerns, with distinct crates for core functionalities like state management, engine logic, pathfinding, goal selection, and bomb strategy. This modularity not only simplifies development and maintenance but also facilitates the integration of new features, such as novel power-ups or modified game rules, with minimal impact on existing code. The architecture now explicitly includes fallback mechanisms to enhance the robustness of the agents, allowing them to revert to safer or more exploratory behaviors when primary plans fail or when faced with uncertain situations. Furthermore, the design has been bolstered to better support RL pipelines, with dedicated hooks and interfaces for integrating neural networks. This includes provisions for observation serialization, policy execution, and reward buffering, paving the way for training agents that can learn and adapt through experience. These enhancements collectively contribute to a more polished and future-proof foundation for developing advanced Bomberman AI.

### 1.3 Reinforcement Learning (RL) Integration Strategy

The architecture is designed to seamlessly integrate Reinforcement Learning (RL) alongside traditional programmatic AI, offering a flexible pathway for developing more adaptive and intelligent agents. While programmatic logic, driven by predefined rules and heuristics, serves as the default mode for basic or rule-based bots, the system provides explicit hooks for neural network (NN) based decision-making. This is primarily facilitated through a new rl crate, which will house the interfaces and implementations for policies, value estimators, and environment wrappers. The Bot Kernel, the core decision-making unit for each agent, is designed to dynamically switch between programmatic and RL modes. This switching can be controlled, for instance, via configuration flags, allowing for A/B testing of different AI approaches or for using programmatic agents as baselines or opponents during RL training. During the training phase, the system will expose Gym-compatible environments, enabling a standardized observation-action-reward loop that can be utilized by common RL algorithms (e.g., PPO, DQN) and libraries. For inference, trained NNs can override critical components of the agent's decision-making process, such as goal selection or action planning, allowing learned behaviors to take precedence.

---

## 2. Domain & Terminology

### 2.1 Core Game Concepts

The game is defined by a set of core concepts that form the basis of the AI agent's understanding and interaction with its environment. The Map is represented as an N×N grid, where each cell is a Tile. Tiles can be empty, contain indestructible walls, destructible soft crates, or power-ups. The simulation progresses in discrete time steps called Ticks, with each tick typically lasting 16 ms, corresponding to a 60 FPS update rate. A central mechanic is the Blast Wave, which is a propagating explosion originating from a bomb. This wave expands in a Manhattan diamond shape, its extent determined by the bomb's power radius, and is halted by walls or agents. Agents can perform Macro-Moves, which are high-level sequences of actions (e.g., "advance 7 tiles east, plant bomb, withdraw west"), and Micro-Moves, which are single atomic actions such as Move::North or Move::South. The AI utilizes an Influence Map, a 2D array that quantifies the danger level at each tile over a series of upcoming ticks, to assess safety and plan movements. High-level objectives for the agent are defined as Goals, such as "demolish crate at (5, 7)" or "acquire BombUp at (10, 3)". To achieve these goals, the agent formulates a Plan, which is a chain of micro-moves and potential bomb placements. Efficient state management is achieved through State Deltas, which are compact representations of changes from the prior tick, enabling incremental computations and reducing processing overhead.

### 2.2 AI and RL Specific Terms

To bridge the gap between general game concepts and the specialized field of Reinforcement Learning (RL), the architecture employs a set of AI and RL-specific terminology. An Observation is a serialized representation of the game state, such as a flattened grid or agent statistics, formatted for input into a neural network. This is crucial for RL agents that perceive the world through these feature vectors. The agent's decision-making logic, whether programmatic or learned, is encapsulated in its Policy. A policy maps observations to action probabilities (in the case of stochastic policies) or directly to actions (for deterministic policies). For RL training, a Reward is a scalar signal derived from state changes or specific events (e.g., +1 for collecting a power-up, -10 for agent death). This reward signal guides the learning process, allowing the agent to discover strategies that maximize cumulative reward over time. The introduction of these standardized terms ensures clarity and facilitates the integration of RL libraries and frameworks, such as those following the OpenAI Gym interface, making the system more accessible for researchers and developers working in the RL domain. This common vocabulary also supports the development of hybrid agents that might combine programmatic logic with learned behaviors.

---

## 3. Global Non-Functional Requirements

### 3.1 Performance and Resource Constraints

The AI architecture is subject to stringent performance and resource constraints to ensure a responsive and scalable system, particularly when dealing with a large number of agents in extensive game worlds. The primary performance metric is Throughput, defined as a median decision time of ≤ 1 ms per bot per tick on a 256×256 map, utilizing a single 3 GHz core. This ensures that even with hundreds of bots, the AI processing does not become a bottleneck for the game's 60 Hz simulation rate. Complementing throughput, Latency is also critical, with a requirement that the 95th percentile decision time remains < 2 ms. This ensures that even in complex scenarios, most agents can make timely decisions. In terms of memory, each bot is limited to 16 MB, encompassing all its internal data structures, including maps, caches, and search trees. This constraint is vital for supporting large-scale simulations with many agents without exhausting system memory. For RL agents, these limits must also accommodate the memory footprint of neural network models, which are expected to be relatively small (e.g., 1-5 MB).

### 3.2 Determinism and Safety

Determinism and safety are paramount in the design of this AI architecture. Determinism is crucial for several reasons: it enables bit-identical replays when the same Random Number Generator (RNG) seeds and inputs are used, which is invaluable for debugging, testing, and analyzing agent behavior. For Reinforcement Learning, determinism in the environment (outside of agent exploration) is often preferred for reproducible training runs and reliable evaluation. While the core game mechanics and environment updates will be deterministic, stochastic policies used during RL exploration will require carefully managed, seeded RNG to ensure that any non-determinism is controlled and reproducible. Safety is addressed by strictly adhering to Rust's memory safety guarantees. The architecture mandates no use of unsafe Rust code beyond well-defined Foreign Function Interface (FFI) boundaries, and the entire codebase should be fully Miri-compatible. Miri is an experimental Rust interpreter that can detect certain classes of undefined behavior, providing an additional layer of confidence in the code's correctness and safety. This focus on safety prevents common bugs such as null pointer dereferencing, data races, and buffer overflows, which is especially important in a complex, concurrent system.

### 3.3 Extensibility and Modularity

The architecture prioritizes extensibility and modularity to accommodate future developments and diverse experimentation. A key requirement is the ability to add new game elements, such as power-ups or modified game rules, with minimal changes to core modules. This is quantified by the goal of implementing such additions in fewer than 100 lines of code, primarily by leveraging trait-based interfaces and a plugin system. This low barrier to modification encourages experimentation and rapid iteration. The modular design, with clearly defined interfaces between components (e.g., state, engine, influence, path, goals, bombs, bot crates), allows developers to work on individual parts of the AI system independently. This separation of concerns simplifies understanding, testing, and replacing components without affecting others. Furthermore, the architecture has been enhanced with specific hooks for Reinforcement Learning (RL) integration. This includes well-defined interfaces for loading NN policies, performing batched inference for efficiency, and managing RL-specific data like observations and rewards. This foresight in design ensures that the system can evolve beyond purely programmatic AI to incorporate sophisticated learning-based agents seamlessly.

---

## 4. High-Level Architecture Overview

### 4.1 System Components and Data Flow

The high-level architecture of the Bomberman AI agent is designed as a hierarchical, data-flow oriented system, facilitating parallelism and clear separation of concerns. At the top sits the Game Engine (in the`engine` crate), responsible for running the core game physics and simulation. The engine communicates with a central Shared State component (in the`state` crate), which holds the authoritative state of the game world (`Arc<RwLock<GameGrid>>`). This shared state is updated every tick by the engine, which also emits State Deltas (dashed lines in the diagram) representing changes from the previous tick. These deltas are crucial for efficient incremental updates in downstream components. The Shared State provides read-only, lock-free Grid Snapshots to individual Per-Bot Decision Tasks. Each bot operates in its own asynchronous task or thread, allowing for concurrent decision-making.

Within each Per-Bot Decision Task, the Bot Kernel orchestrates the agent's logic. The kernel first consults the Goal Manager to determine high-level objectives based on the current snapshot. The State Evaluator then scores these goals, considering factors like distance, safety, and potential rewards. Once a goal is selected, the Pathfinder generates a sequence of micro-moves to reach the target, and the Bomb Planner handles bomb-related tactics. Optionally, components like the Goal Manager or Pathfinder can be overridden by an RL Policy NN (Neural Network), allowing learned behaviors to guide these processes. Actions chosen by the kernel are submitted back to the Game Engine (via the Shared State or a direct channel) for execution. An RL Reward Buffer within the bot kernel can accumulate rewards during an RL episode. This structured flow ensures that each component has a well-defined responsibility and interacts with others through clear data interfaces, promoting both performance and maintainability.

### 4.2 Concurrency and Parallelism Model

The concurrency model is designed to maximize performance by leveraging Rust's robust support for asynchronous and parallel execution. The Main Engine Thread (in the`engine` crate) is responsible for managing the GameGrid, simulating game ticks, and emitting state deltas. To handle potentially hundreds of AI agents, each Bot operates within its own Per-Bot Decision Task. These tasks can be implemented as asynchronous tasks (e.g., using Tokio's runtime) or as standard library threads, scaling with the available CPU cores, potentially managed by a thread pool like Rayon. Communication between the main engine thread and the bot tasks, as well as between different components within a bot task, utilizes Rust's channel primitives. Specifically, watch channels are suitable for broadcasting state deltas from the GameGrid to all subscribed bot tasks, ensuring they receive timely updates. For bot commands (actions to be executed in the game world), mpsc (multi-producer, single-consumer) channels can be used to send requests from bot tasks back to the game engine.

To ensure data integrity and avoid locks on the hot path of decision-making, the architecture employs lock-free programming techniques, primarily through the use of crossbeam-epoch for managing GridSnapshot access. This allows multiple bot tasks to concurrently read the game state without blocking each other or the main engine thread. For computationally intensive operations within AI components, such as updating large influence maps or performing complex pathfinding, further parallelism can be introduced using Rayon's parallel iterators or by spawning additional short-lived tasks. For instance, influence map updates for multiple independent blast waves could be parallelized. The architecture also suggests considering an Entity-Component System (ECS) framework like specs if the number of agents and game entities grows very large (e.g., 100+), as ECS can offer a more data-oriented and cache-efficient approach to managing game state and AI logic. For Reinforcement Learning, batched inference for multiple agents can be performed in parallel using Rayon, and model loading can be handled asynchronously to avoid blocking the decision loop.

---

## 5. Module-by-Module Specification

### 5.1 **state crate** *(formerly part of grid)*

The`state` crate provides the foundational data structures and logic for representing the game world. Its central component is the`GameGrid` struct, which encapsulates the entire state of the game map. This struct contains several key fields:`tiles`, representing the N×N grid of Tile objects (e.g.,`Box<[Tile; N*N]>`, potentially using const generics for fixed-size optimization);`bombs`, a collection of active bombs (e.g.,`Slab<Bomb>` for efficient ID-based lookup and storage);`agents`, a collection of AgentState objects representing all players in the game (also potentially using`Slab<AgentState>`); and a version field (e.g.,`AtomicU64`) to track the current simulation tick, crucial for synchronizing snapshots and deltas. The`GameGrid` is designed to be shared across multiple threads (e.g., wrapped in an`Arc<RwLock<GameGrid>>`) and offers a mechanism for lock-free snapshots using`crossbeam::epoch::Guard`. This ensures that AI agents can access a consistent view of the game state without incurring the overhead of mutex locking during their decision-making process.

The`GameGrid` implements a`Grid` trait, which defines common operations for querying and manipulating the grid. Key methods include`apply_delta(delta: GridDelta)` for applying incremental changes to the grid state, typically received from the game engine after each tick. To enable reactive AI agents, the`GameGrid` provides a`subscribe() -> Receiver<GridDelta>` method, allowing bot tasks to receive streams of state changes. This is essential for agents to respond to dynamic events like bomb placements, explosions, or opponent movements. A significant addition for Reinforcement Learning (RL) is the`to_observation(agent_id: AgentId) -> Vec<f32>` method. This function is responsible for serializing the relevant game state, from the perspective of a specific agent, into a flat vector of f32 values. This vector serves as the input to a neural network policy. The serialization might include one-hot encoded tile types, agent positions, bomb timers, power-up statuses, or pre-processed features like influence maps. The design of this observation space is critical for RL performance and will likely be an area of experimentation.

### 5.2 **engine crate** *(new)*

The`engine` crate encapsulates the main game loop and simulation logic. It owns the authoritative`GameGrid` instance (via`Arc<RwLock<GameGrid>>` from the`state` crate) and is responsible for advancing the simulation by one tick at 60 Hz. Key responsibilities include:
- Updating bomb timers and triggering explosions.
- Resolving agent movements and bomb placements.
- Calculating blast propagation and damage.
- Generating`GridDelta` events summarizing all state changes since the previous tick.
- Broadcasting these deltas to all subscribers (e.g., bot tasks) via a`tokio::sync::watch` channel.

The engine exposes a simple API:
```rust
pub struct Engine {
    grid: Arc<RwLock<GameGrid>>,
    delta_tx: watch::Sender<GridDelta>,
}

impl Engine {
    pub fn new(size: usize) -> (Self, watch::Receiver<GridDelta>) { ... }
    pub fn tick(&mut self) { ... } // Advances simulation by one tick
}
```

### 5.3 Snapshot Layer (integrated in state crate)

The Snapshot Layer, integrated within the state crate (e.g., in a snapshot.rs file), is responsible for providing immutable, read-only views of the GameGrid at specific points in time (versions). The primary structure is SnapshotView<'a>, which offers a zero-copy, read-only perspective of the grid’s tiles and potentially other relevant state. This immutability is crucial for the AI’s decision-making process, as it ensures that the state does not change while an agent is evaluating it, leading to more consistent and predictable behavior.

SnapshotView is backed by triomphe::Arc<[Tile]> (or a similar thread-safe, immutable reference-counted structure) for the tile data, allowing multiple bot tasks to hold references to the same snapshot without cloning the entire grid. This approach is highly efficient and ensures freedom from allocations on the hot path of AI decision-making, which is critical for meeting performance targets.

The Snapshot Layer works in conjunction with the GameGrid’s versioning system. When a bot requests a snapshot, it receives a SnapshotView corresponding to the GameGrid’s current version. This snapshot remains valid and consistent even if the GameGrid is updated by the game engine in subsequent ticks. This is typically managed using epoch-based reclamation (e.g., via crossbeam-epoch) to safely free old snapshots when no longer needed. For Reinforcement Learning, the snapshot mechanism is particularly beneficial. RL algorithms often require a consistent state for calculating Q-values or policy gradients. The SnapshotView provides exactly this. Furthermore, a new method, observe_delta(prev: &SnapshotView) -> ObservationDelta, can be introduced. This method would compute the difference between the current live GameGrid (or its latest snapshot) and a previously captured SnapshotView. The resulting ObservationDelta could then be used to create an incremental observation for an RL agent, which can be more efficient than serializing the full state every tick, especially if only small parts of the game state have changed. This incremental approach can significantly reduce the data processing overhead for the neural network.

### 5.4 Core Bot (bot crate)

The bot crate defines the Bot struct, which serves as the central coordinating unit for an individual AI agent’s decision-making and interaction with the game environment. Each Bot instance is typically associated with a unique AgentId and holds a reference to the Arc<GameGrid> (from the state crate) to access game state information. It also contains a BotConfig struct, which allows for runtime customization of the bot’s behavior, such as adjusting heuristic weights, decision thresholds (e.g., via TOML configuration files), and, crucially, a flag to enable or disable Reinforcement Learning (RL) mode (rl_mode: bool). If RL mode is active, the Bot will also hold an optional reference to an RL policy, rl_policy: Option<Arc<dyn Policy>>, which can be loaded from a specified model_path: String.

The core logic of the Bot is typically implemented in a loop (e.g., within an async task). In each iteration of this loop, corresponding to a game tick or decision cycle, the bot retrieves a current GridSnapshot from the GameGrid. Based on its configuration, it then decides whether to use programmatic AI or RL for decision making. If config.rl_mode is true, the Bot Kernel (an internal component of Bot) invokes its rl_tick(&snapshot).await method, which uses the loaded NN policy to determine an action. Otherwise, it calls kernel.tick(&snapshot).await to use the programmatic logic. Once an action (or command) is determined, the bot submits this command back to the GameGrid (or directly to the game engine via a channel) for execution. This design allows for seamless toggling between different AI paradigms without significant refactoring, facilitating comparative analysis and hybrid approaches where, for example, an RL agent might fall back to programmatic logic under certain conditions or during specific phases of learning. The Bot struct acts as the primary interface for creating, configuring, and managing the lifecycle of AI agents within the simulation.

### 5.5 Goal Manager (goals crate)

The goals crate is responsible for generating, prioritizing, and managing the high-level objectives (Goals) for an AI agent. It defines a core trait, GoalGenerator: Send + Sync, which includes a method fn generate(&self, snap: &Snapshot) -> Vec<Goal>;. This trait allows for various strategies for goal creation to be implemented and plugged into the system. Default implementations of GoalGenerator might include DestroyNearestCrate, CollectNearestPowerUp, HuntWeakestEnemy, and FleeToSafeZone. These generators analyze the current game Snapshot and propose a list of potential goals for the agent to pursue. The GoalManager itself maintains a priority queue of (Goal, Score, Plan) tuples. The Score is typically calculated by the StateEvaluator (from the scoring.rs module), and the Plan (a sequence of micro-actions) is generated by the Pathfinder and BombPlanner. Replanning is a critical aspect of the GoalManager. It is triggered under specific conditions: either when the GridDelta (representing changes in the game state) indicates a significant shift that invalidates the current plan (e.g., a new threat appears, a target is destroyed), or when the score of the current active goal drops below a certain threshold (e.g., 60% of the score of a newly identified, better goal). This adaptive replanning ensures that the agent remains responsive to the dynamic game environment. An important enhancement is the introduction of fallback mechanisms. If primary plans fail (e.g., the Pathfinder cannot find a route to the current goal), the GoalManager can revert to safer “idle” or “explore” goals to prevent the agent from becoming stuck or making suboptimal decisions. For Reinforcement Learning integration, a new RlGoalSelector implementation of GoalGenerator can be introduced. This selector would potentially use a neural network to score or directly select goals from a candidate set generated by other (potentially programmatic) goal generators, allowing learned preferences to guide high-level strategic decisions.

### 5.6 State Evaluator (integrated in goals crate)

The StateEvaluator, integrated within the goals crate (e.g., in scoring.rs), is responsible for assigning a quantitative score to potential goals or states, guiding the GoalManager in its selection process. It defines a trait StateEvaluator { fn score(&self, snap: &Snapshot, goal: &Goal) -> f32; }. This trait allows for various evaluation heuristics to be implemented and combined. Factors considered in scoring typically include the A* pathfinding distance to the goal, the safety of the path and the goal location (often by negating influence map values), the potential rewards associated with achieving the goal (e.g., value of a power-up, strategic advantage of destroying a crate), and the level of threat posed by opponents or imminent explosions. The system supports a plugin architecture, potentially using type_map::TypeMap, to allow different scoring components to be registered and combined flexibly.

A significant addition for Reinforcement Learning is the introduction of a RewardFunction trait. This trait defines RL-specific dense reward signals, which can include not only rewards for achieving sub-goals (e.g., collecting a power-up) but also shaping rewards that encourage progress towards larger objectives or survival (e.g., small positive rewards for moving towards a target, negative rewards for staying in dangerous areas). This RewardFunction would be used by the rl crate to provide feedback to the learning algorithm. Furthermore, the StateEvaluator’s capabilities can be expanded for RL by implementing an NnValueEstimator. This component would query a value network (a type of neural network) to estimate the long-term value of a given state-goal pair. This value estimate can then be used as a critical input to the scoring process, allowing the agent to leverage learned value functions for more informed decision-making, effectively integrating a critic model as seen in actor-critic RL architectures. This makes the evaluator a central point for blending heuristic knowledge with learned value estimates.

### 5.7 Pathfinder (path crate)

The path crate is dedicated to all aspects of pathfinding for the AI agents. Its core component is an A* search algorithm, typically implemented with a BinaryHeap<Node> for the open set and a Slab<Node> or similar efficient collection for the closed set. The heuristic function used by A* is crucial and often combines Manhattan distance with penalties derived from the InfluenceMap to discourage paths through dangerous areas. To handle the dynamic nature of the Bomberman environment (e.g., new bombs, destroyed crates opening up paths), the pathfinder supports incremental updates, potentially using algorithms like D* Lite. This allows previously calculated paths to be repaired more efficiently than recalculating from scratch. Beyond simple point-to-point pathfinding, the path crate also includes a macro-move planner. This component, through a function like fn expand_macro(start: Pos, macro_cmd: Macro) -> Vec<Micro>, translates high-level movement commands (e.g., “move 5 tiles north”) into a sequence of atomic micro moves. This can be optimized using lazy waypoint generation to avoid unnecessary computation.

For performance optimization, if path-cost calculations become a bottleneck, the use of SIMD (Single Instruction, Multiple Data) instructions could be explored for parallelizing certain computations within the A* algorithm, such as heuristic calculations or node comparisons. For Reinforcement Learning integration, an optional RlPathSampler can be introduced. This component would allow a neural network to guide the pathfinding process, perhaps by influencing the heuristic, suggesting preferred directions, or even sampling entire paths based on learned preferences. This could be particularly useful in complex or uncertain environments where traditional heuristics might fall short, allowing the agent to learn more nuanced navigation strategies. The RlPathSampler would provide a hook for learned behaviors to directly influence low-level movement decisions, complementing the higher-level goal selection potentially also guided by RL.

### 5.8 Bomb Planner (bombs crate)

The bombs crate encapsulates the logic related to bomb placement, explosion propagation, and safety assessment. A key feature is the caching of blast chains using a graph data structure like petgraph. This allows for efficient calculation of the total area affected by a series of chain reactions when a bomb explodes, which is crucial for both offensive and defensive planning. The crate provides functions like fn safe_tiles(pos: Pos, power: u8, snap: &Snapshot) -> Array2<bool>, which, given a potential bomb placement position, its power, and the current game snapshot, returns a 2D array (e.g., using ndarray) indicating which tiles will be safe from the resulting explosion. This is fundamental for agents to avoid self-destruction and to assess the threat posed by enemy bombs.

To determine if an agent can reach a safe tile before a bomb explodes, a Breadth-First Search (BFS) is used to check for reachable safe spots within the bomb’s timer. The performance of grid representations (ndarray versus fixed-size arrays) should be profiled, and parallelism (e.g., using rayon) can be considered for calculating blast radii or safety maps for very large or numerous simultaneous explosions. For Reinforcement Learning, a RlBombPolicy can be implemented. This policy would use a neural network to predict the probabilities of good bomb placements or to decide whether placing a bomb at the current location is advantageous. The NN could learn complex patterns related to trapping opponents, destroying specific crate configurations, or creating advantageous future board states. This allows the bomb planning, a critical tactical element in Bomberman, to be guided by learned experience rather than solely by pre-programmed heuristics, potentially leading to more sophisticated and effective bombing strategies.

### 5.9 Influence Map (influence crate)

The influence crate is responsible for generating and managing influence maps, which are 2D arrays (e.g., Array2<f32>) quantifying the danger level at each tile over a series of upcoming ticks. An InfluenceMap struct typically contains the map data itself, a dirty flag (e.g., a BitVec<N*N>) to track which tiles need updating, and a version number to synchronize with game ticks. The primary purpose of these maps is to provide agents with a spatial understanding of threats, such as active bomb blast zones and predicted explosion timings. The maps are updated incrementally; when a new bomb is placed or a change in the environment occurs, only the affected tiles (marked as dirty) and their surroundings need to be recalculated, rather than the entire map. This efficiency is crucial for performance.

A common characteristic of influence maps is the decay of influence over time and distance. For example, the influence I at a tile at time t+1 might be calculated as I(t+1) = max(0, I(t) * decay_factor – constant_decay), where decay_factor could be 0.9 and constant_decay 0.05. This models the fading danger as a bomb’s explosion recedes or its timer counts down. For scenarios with many bombs, optimizations like rayon::par_iter can be used to parallelize the update calculations across different regions of the map or for different bombs. For Reinforcement Learning, the influence map itself, or features derived from it, can be exported as part of the agent’s observation vector. Providing the NN with this pre-processed spatial danger information can significantly enhance its ability to learn safe navigation and tactical positioning, acting as a powerful form of feature engineering that simplifies the learning problem.

### 5.10 Event System (events crate)

The events crate provides a system for handling and broadcasting game events. It defines an enum, GameEvent, which enumerates various significant occurrences in the game, such as BombExplode { pos, power }, AgentDeath { id }, PowerUpCollected { id, kind }, and others. These events are crucial for multiple aspects of the AI system. Firstly, they drive the incremental updates in components like the InfluenceMap and the GoalManager’s replanning logic. Secondly, they are essential for Reinforcement Learning, as events often define rewards (e.g., positive reward for PowerUpCollected, negative for AgentDeath) and mark the end of episodes. The event system typically uses an asynchronous broadcast mechanism (e.g., tokio::sync::broadcast channel) to disseminate events to all interested subscribers, which can include logging systems, RL agents, and replay recorders.

To enhance its utility, particularly for offline RL, the event system can be expanded to include episode serialization. This means capturing streams of GridDeltas and GameEvents and saving them, for example, to JSON files. These serialized episodes can then be used for dataset generation or offline training of RL models. For RL training, the event system is further expanded to produce Transition objects. A Transition typically contains { obs, action, reward, next_obs, done } and represents a single step in an RL episode. These transitions are fed into replay buffers, which are then sampled by RL algorithms to train the neural networks. The events crate thus serves as a central nervous system for the AI, conveying critical information about state changes and game occurrences, and is a key enabler for both logging/debugging and sophisticated RL training pipelines.

### 5.11 RL Integration (rl crate)

A new rl crate is introduced to centralize all components related to Reinforcement Learning, ensuring a clean separation from the core programmatic AI logic and preventing bloat in other modules. This crate defines essential traits for RL functionality. The Policy: Send + Sync trait includes methods like fn act(&self, obs: &Vec<f32>) -> Action; for deterministic or probabilistic action selection, and fn sample(&self, obs: &Vec<f32>, epsilon: f32) -> Action; specifically for exploration, often incorporating an epsilon-greedy strategy. The ValueEstimator: Send + Sync trait provides a method fn value(&self, obs: &Vec<f32>) -> f32; for estimating the state value, crucial for algorithms like Actor-Critic or for value-based methods.

An implementation of TorchPolicy (and TorchValueEstimator) would be provided, leveraging Rust bindings for PyTorch (e.g., the tch crate). This allows loading pre-trained neural network models (e.g., CNNs for processing grid-based observations, or MLPs for feature vectors) and using them for inference directly within the Rust environment. To facilitate training, the rl crate also provides a Gym-compatible environment wrapper, BomberEnv. This struct, holding a reference to the Arc<GameGrid> (from state) and an agent_id, implements methods like fn step(&mut self, action: Action) -> (Vec<f32>, f32, bool); and fn reset(&mut self) -> Vec<f32>;. This standardized interface allows the Bomberman game to be easily integrated with existing RL libraries (e.g., Stable Baselines3, Ray RLlib) that expect a Gym-like environment. The rl crate also supports mechanisms for both external training (e.g., via gRPC if the RL logic runs in Python) and on-device training with replay buffers and training loops managed within Rust. This comprehensive approach makes the rl crate a self-contained module for all learning-related functionalities.

---

## 6. Data-Structure Cheat-Sheet

### 6.1 Core Game State

The core game state is represented by several fundamental data structures. The Tile enum defines the possible states of a single cell in the game grid, such as Empty, Wall (Indestructible), SoftCrate (destructible), and PowerUp(P) (where P is a type for power-ups). This enum is designed for memory efficiency and fast pattern matching. The Bomb struct represents an active bomb in the game, containing fields like a unique Id, its position on the grid, a timer indicating ticks until explosion, and its power (blast radius). AgentState struct holds the current status of an agent, including its Id, position, bombsLeft (number of bombs the agent can still place), and power (current blast radius of its bombs). These structures are designed to be compact and efficient, as many instances of them will exist and be frequently accessed. Their complexity for basic access and update operations is typically O(1), ensuring that game logic remains fast.

### 6.2 AI and Planning Structures

For AI planning and decision-making, specialized data structures are introduced. The InfluenceMap is an Array2<f32> (from the ndarray crate) that stores danger levels for each tile, with updates being O(k) per bomb, where k is the number of tiles affected by the blast. The A* pathfinding algorithm utilizes a BinaryHeap<Node> for its open set, where Node represents a search state with cost and heuristic. The complexity of A* is O(E log V), where E is the number of edges and V is the number of vertices (tiles) in the search graph. The GoalHeap is another BinaryHeap<GoalEntry> used by the GoalManager to prioritize goals based on their scores. Insertion and removal from a binary heap are O(log G), where G is the number of goals.

### 6.3 RL Specific Structures

For Reinforcement Learning, specific data structures are introduced to manage observations, actions, and training data. An Observation is represented as a Vec<f32>. This vector is a serialized, flattened representation of the game state relevant to an agent, suitable for input into a neural network. The serialization process typically involves iterating over the game grid and agent properties, resulting in a complexity of O(N²) for a grid of size NxN, though incremental observations might reduce this. A Transition struct is a fundamental unit for RL training, representing a single step in an agent's interaction with the environment. It typically contains fields like obs (current observation), act (action taken), rew (reward received), next_obs (observation after the action), and done (a boolean indicating if the episode terminated). Creating a Transition is an O(1) operation with respect to the size of the game state, as it primarily involves storing references or small pieces of data. These structures are designed to be compatible with common RL libraries and neural network frameworks, often requiring data to be in contiguous arrays of floating-point numbers.

| Name          | Type                              | Purpose                  |
|---------------|-----------------------------------|--------------------------|
| Tile         | enum [Empty, Wall, SoftCrate, PowerUp(P)] | Grid cell state         |
| Bomb         | struct [id, pos, timer, power]   | Live bombs              |
| AgentState   | struct [id, pos, bombs_left, power] | Agents                 |
| SnapshotView | &[Tile] + Indices                | Lock-free read          |
| InfluenceMap | Array2<f32> + bitset             | Danger field            |
| AStar        | BinaryHeap<Node>                 | Micro-move path         |
| GoalHeap     | BinaryHeap<GoalEntry>            | Goal prioritization     |
| Observation  | Vec<f32>                         | NN input (flattened)    |
| Transition   | struct [obs, act, rew, next, done] | RL training data       |

---

## 7. Threading & Concurrency Model

### 7.1 Engine and Bot Task Management

The threading and concurrency model is designed to maximize parallelism and ensure responsive AI decision-making. The Main Engine Thread is responsible for the core game loop: updating the GameGrid, simulating physics, handling player inputs (if any), and emitting GridDelta events that represent changes in the game state. This thread operates at a fixed tick rate (e.g., 60 Hz). To handle AI for multiple agents, each Bot is assigned its own Per-Bot Decision Task. These tasks can be implemented as asynchronous tasks, managed by a runtime like Tokio, or as standard library threads. If using threads, a thread pool, possibly managed by Rayon, can be used to limit the number of concurrent threads to the number of available CPU cores, preventing resource exhaustion. These bot tasks run concurrently, allowing multiple agents to process their logic and make decisions simultaneously. This parallel processing is crucial for achieving the target throughput of 1 ms per bot per tick, especially when dealing with hundreds of agents. The architecture also supports batched inference for RL agents, where multiple agents' observations can be processed by a neural network in a single batch, further leveraging parallelism, potentially on a GPU if available.

(Note: References to "GameGrid" updated to "GameState from state crate"; engine thread lives in engine crate.)

### 7.2 Communication Mechanisms

Communication between the main engine thread, the shared GameGrid, and the individual bot tasks is facilitated by Rust's channel primitives, which are designed for safe and efficient inter-thread messaging. The GameGrid uses watch channels to broadcast GridDelta events to all subscribed bot tasks. A watch channel is suitable for this purpose because it maintains a single value (the latest delta) that multiple receivers can observe, and it efficiently notifies subscribers of changes. For bot commands (actions to be executed in the game world), mpsc (multi-producer, single-consumer) channels are used to send requests from bot tasks back to the engine thread. This setup ensures that commands from multiple bots can be queued and processed by the single engine thread without contention. Other communication, such as between internal AI components within a bot task, can use simpler synchronous channels or direct method calls if no concurrency is needed. Asynchronous channels (from tokio) are preferred when the Tokio runtime is used, depending on the complexity and asynchronicity required. The choice of channels ensures that data is passed between threads safely, avoiding shared mutable state issues common in concurrent programming.

### 7.3 Lock-Free Strategies and ECS Consideration

To minimize contention and ensure low-latency decision-making, the architecture employs lock-free strategies for accessing shared game state. The most prominent example is the use of crossbeam-epoch for managing GridSnapshot access. When a bot task requests a snapshot of the GameGrid, it receives an immutable, lock-free view of the grid at a specific version. This allows multiple bots to read the game state concurrently without blocking each other or the main engine thread that might be updating the GameGrid. The epoch-based reclamation mechanism ensures that these snapshots are kept alive as long as any bot task is using them and are safely deallocated when no longer needed. This approach is critical for meeting the stringent throughput and latency requirements. For managing a very large number of entities (agents, bombs, etc.), the architecture suggests considering an Entity-Component-System (ECS) framework like specs. ECS provides a data-oriented design pattern that can improve cache locality and parallelism for systems that process many entities. While not explicitly implemented in the current module structure, it's a viable path for future scalability if the complexity of game entities and their interactions grows significantly beyond the initial targets. The RL components, such as model loading, can also benefit from asynchronous operations to avoid blocking the main decision loops of the bots.

---

## 8. Extensibility Hooks

### 8.1 Serialization and Custom Effects

The architecture incorporates several extensibility hooks to allow for easy modification and enhancement of the game and AI behavior. A key feature is the use of serde for serialization and deserialization of various game state types. This is particularly important for Reinforcement Learning, as it enables the export of game states, events, and transitions to formats like JSON for offline analysis, dataset generation, or model training. Serde compatibility also facilitates saving and loading game replays, which is invaluable for debugging and performance benchmarking. Another extensibility point is the PowerUpEffect trait, which can be implemented for custom power-ups. This trait would include a method like fn apply(&self, agent: &mut AgentState);. New power-ups can be registered in a global registry. The apply method would define how the power-up modifies the agent's state (e.g., increasing bomb count, blast radius, or speed). This registry-based approach means new power-ups can be introduced without modifying core game logic or AI modules, adhering to the open/closed principle.

### 8.2 Opponent Modeling and ML Integration

To support more sophisticated AI strategies, particularly those involving predictions about other agents' behavior, an OpponentModel trait is proposed: trait OpponentModel: Send { fn predict(&self, opp_id: AgentId, snap: &Snapshot) -> Vec<Pos>; }. This trait allows for the development of modules that can predict the likely future positions or actions of opponent agents. Such models could range from simple rule-based predictors (e.g., "opponent will likely move towards nearest powerup") to more complex machine learning models trained to anticipate opponent behavior based on historical data or observed patterns. The predict method would take the ID of an opponent and the current game snapshot, and return a probability distribution or a set of likely future positions. This information could then be used by the GoalManager or BombPlanner to make more informed decisions, such as setting traps or avoiding ambushes. This hook provides a clear interface for integrating various forms of opponent modeling, enhancing the strategic depth of the AI agents.

### 8.3 RL Environment and Policy Registry

The integration of Reinforcement Learning is further facilitated by specific extensibility hooks. Gym-like wrappers are provided to expose the Bomberman game as a standard RL environment. This allows the game to be easily used with a wide range of existing RL algorithms and libraries that expect the common step and reset interface. These wrappers handle the conversion between the game's internal state representation and the Observation vectors expected by RL agents, as well as managing the Reward signals and done flags. To manage the neural network models used by RL agents, a PolicyRegistry is proposed. This registry would allow NN models (e.g., TorchScript models) to be loaded dynamically at runtime based on configuration, with support for fallback to programmatic logic if a model fails to load or encounters an error. This registry can be extended to support multiple model types or versions, enabling A/B testing of different RL policies. By providing these hooks, the system can gracefully handle unexpected situations, such as a failed model load.

---

## 9. Testing Strategy

### 9.1 Property-Based and Random Testing

A comprehensive testing strategy is essential to ensure the correctness and robustness of the AI agents and the game logic. Property-based testing, using frameworks like QuickCheck or Proptest, will be employed to verify that certain invariants hold true across a wide range of inputs. For example, tests can ensure that valid moves always keep the agent within map bounds, or that bomb explosions correctly affect tiles within their blast radius. Random map generation, also facilitated by Proptest, will be used to test the AI's behavior on a variety of procedurally generated scenarios, helping to uncover edge cases that might not be apparent in hand-crafted test maps. This approach helps in validating the AI's adaptability and resilience to different game configurations. These tests will focus on the fundamental rules of the game and the basic functionalities of the AI components, providing a solid foundation of correctness.

### 9.2 Replay and Benchmarking

Replay functionality is a critical tool for debugging and analyzing AI behavior. The system will support saving and loading game replays, which are essentially sequences of GridDelta events and initial game states. By replaying these deltas, the exact state of the game at any tick can be reconstructed, allowing developers to step through an agent's decisions and understand its reasoning process. This is invaluable for diagnosing unexpected behaviors or verifying the impact of code changes. Complementing functional testing, Criterion benchmarks will be used to measure the performance of key AI components, such as pathfinding, influence map updates, and the overall decision-making loop. These benchmarks will help ensure that the nonfunctional requirements for throughput and latency are being met and can identify performance regressions as the codebase evolves. For RL agents, replays are also crucial for visualizing learned behaviors and comparing the performance of different trained models.

### 9.3 Mocking and Unit/Integration Tests

To isolate and test individual components effectively, the architecture employs mocking for dependencies. For instance, the SnapshotView can be mocked with a MockSnapshot struct that implements the necessary traits, allowing pathfinding or goal management tests to run without a full GameGrid. This is particularly useful for unit tests that focus on a single module's logic. Integration tests will combine multiple modules, such as testing the end-to-end decision-making process from goal generation to action execution, potentially using a small, fixed game scenario. For RL components, tests will include verifying that observations are correctly serialized, that policies can load and infer from mock models, and that the Gym wrapper correctly handles step and reset calls. Fuzz testing, using tools like cargo-fuzz, can be applied to critical functions like blast propagation or pathfinding to detect crashes or invalid states under random inputs. These tests, combined with CI pipelines enforcing them, ensure that the system remains reliable as it evolves. This multi-faceted testing approach aims to deliver a reliable and high-quality AI system.

---

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

## 11. Roadmap for Future Work

### 11.1 Performance and Scalability Enhancements

The architecture is designed with future performance and scalability enhancements in mind. One significant area is the optimization of the InfluenceMap calculations. While incremental updates and parallel processing with rayon are initial steps, future work could explore GPU acceleration for these computations, potentially using frameworks like wgpu. This would be particularly beneficial for very large maps or extremely high numbers of simultaneous explosions. Another avenue for scalability is improving the handling of massive maps, such as 1024x1024 grids. This might involve streaming techniques for game state, where only relevant portions of the map are fully loaded or processed by an agent, or more sophisticated data structures for sparse map representations. These enhancements aim to push the boundaries of the AI system's capacity, allowing for even more complex and large-scale Bomberman simulations.

### 11.2 Advanced RL and Multi-Agent Support

The roadmap includes significant advancements in Reinforcement Learning capabilities. A key area is supporting multi-agent RL (MARL) scenarios, where multiple agents learn simultaneously, potentially in cooperative or competitive settings. This would involve extending the Gym wrapper to handle multi-agent environments and integrating algorithms suited for MARL, like QMIX or MADDPG. Another focus is the development of hybrid programmatic-NN ensembles. This involves developing sophisticated ways to combine the strengths of pre-programmed heuristic AI with the adaptive learning capabilities of neural networks. For example, an NN might override specific sub-components of a programmatic agent, or programmatic logic could provide safe exploratory behaviors or interpretable fallbacks for an NN-driven agent. Distributed RL training via gRPC or other RPC mechanisms is also on the roadmap, allowing training to be scaled out across multiple machines, potentially with dedicated hardware for different parts of the pipeline (e.g., simulation, NN training).

### 11.3 Game Engine and Platform Expansion

Beyond core AI and performance, the roadmap includes expanding the game's reach and integration with other platforms and engines. One goal is to enable WebAssembly (WASM) compilation for the game and AI logic, allowing it to run in web browsers. This would facilitate easier deployment, sharing of agents, and web-based tournaments. Integration with more full-featured game engines like Bevy is also a possibility. While the current architecture focuses on the AI and server logic, integrating with an engine like Bevy would allow for the development of richer graphical clients and more complex game modes, while still leveraging the robust Rust backend for simulation and AI. These expansions aim to make the Bomberman AI framework more versatile and accessible to a wider community of developers and researchers.

---

## 12. Revised Crate Layout

### 12.1 Workspace Structure and Crate Dependencies

The project is organized as a Cargo workspace, promoting modularity, independent compilation, and clear dependency management. The root of the workspace contains a Cargo.toml file that lists all member crates. The primary crates are located within a crates/ directory. These include:

- state/ : Immutable game-state data structures.
- engine/ : Main game loop & physics.
- influence/ : Danger map calculations.
- path/ : Pathfinding algorithms (A*, D* Lite).
- goals/ : Goal generation, management, and state evaluation/scoring.
- bombs/ : Bomb-related logic and safety checks.
- bot/ : The bot kernel, configuration, and example bots.
- events/ : Game event system and broadcasting.
- rl/ : New crate for RL policies, environments, and buffers.
- test_utils/ : Utilities for testing, such as mock objects.
- ffi/ : (optional) For Foreign Function Interface bindings.

Additional directories at the root include docs/ for architecture documentation and UML diagrams, benches/ for Criterion benchmarks, and tests/ for integration tests. This structure ensures that each component has a well-defined scope and API, reducing compile times and improving code organization. Dependencies between crates are carefully managed to avoid cyclic dependencies, ensuring a clean and maintainable build process.

```
bomberman_ai/
├── Cargo.toml
├── crates/
│   ├── state/          # Immutable game-state data structures
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── grid.rs
│   │   │   └── snapshot.rs
│   ├── engine/         # Main game loop & physics
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── tick.rs
│   ├── influence/
│   ├── path/
│   ├── goals/
│   ├── bombs/
│   ├── bot/
│   ├── events/
│   ├── rl/
│   ├── test_utils/
│   └── ffi/
├── docs/
├── benches/
└── tests/
```

### 12.2 Cargo Workspace Configuration

The root Cargo.toml file defines the workspace and its common dependencies. This configuration ensures that all crates within the workspace use consistent versions of external libraries, managed by Cargo's dependency resolver (resolver = "2"). Key dependencies include tokio for asynchronous runtime, crossbeam for concurrent data structures and epoch-based memory reclamation, criterion for benchmarking, proptest for property-based testing, ndarray for N-dimensional array operations, petgraph for graph data structures, and triomphe for Arc variants. For error handling, anyhow is included. A crucial addition for the RL components is the tch crate (assuming a Rust binding for PyTorch like tch-rs), which will enable loading and running neural network models directly within Rust. This centralized dependency management simplifies builds and ensures compatibility across the entire project.

```toml
[workspace]
members = [
    "crates/state",
    "crates/engine",
    "crates/influence",
    "crates/path",
    "crates/goals",
    "crates/bombs",
    "crates/bot",
    "crates/events",
    "crates/rl",
    "crates/test_utils",
    "crates/ffi",
]
resolver = "2"

[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
crossbeam = "0.8"
criterion = "0.3"
proptest = "1.0"
ndarray = "0.15"
petgraph = "0.6"
triomphe = "0.1"
anyhow = "1.0"
tch = "0.7"  # For PyTorch bindings
```

### 12.3 Dependency Flow Between Crates

Dependencies flow from lower-level crates (e.g., state, engine) to higher-level ones (e.g., bot, rl). For example:
- All crates depend on state for GameGrid and snapshots.
- Engine depends on state.
- Influence, path, goals, bombs depend on state and engine (for deltas).
- Bot depends on state, engine, influence, path, goals, bombs, events.
- Rl depends on state, engine, bot (for policy integration).
- Test_utils depends on most crates for mocks.

This acyclic dependency graph ensures clean compilation and modularity.

---

## 13. CI & Quality Gates

### 13.1 Standard Rust CI Practices

To maintain high code quality and ensure stability, a robust Continuous Integration (CI) pipeline will be established using GitHub Actions. This pipeline will be configured in a .github/workflows/ directory. Standard Rust CI practices will be enforced, including running cargo check to verify code compilation, cargo clippy for linting and identifying common code smells or potential improvements, and cargo test to execute all unit and integration tests. A crucial step will be running tests under Miri, Rust's experimental interpreter.

(Note: Crate names updated in scripts to reflect state and engine split.)

### 13.2 RL Specific Testing and Benchmarks

Beyond standard Rust practices, the CI pipeline will include specific checks and benchmarks for the Reinforcement Learning components. This involves adding RL-specific tests to the ci.yml workflow. These tests might include smoke tests for model loading (e.g., verifying that a sample TorchScript model can be loaded correctly by the TorchPolicy implementation) and integration tests that run simple RL agents with mock or small pre-trained NNs to ensure the end-to-end RL loop (observation, action, reward) is functioning as expected. Benchmarks for RL inference speed will also be incorporated, likely using Criterion, to ensure that NN-based decision-making meets the latency requirements. These benchmarks will measure the time taken to serialize an observation, pass it through a representative NN policy, and sample an action. Code coverage tools like Tarpaulin will be used, with reports potentially uploaded to services like CodeCov, to track test coverage and identify areas of the codebase, especially within the rl crate, that may need more thorough testing. These quality gates are essential for ensuring the reliability and performance of the learning-based agents.

```
.github/
└── workflows/
    ├── ci.yml     # clippy, test, miri
    ├── bench.yml  # criterion on PRs
    └── coverage.yml # tarpaulin to codecov
```

---

## 14. Usage from a New Crate

### 14.1 Adding Dependencies to the Workspace

To use the bomberman_ai framework from a new crate, perhaps for developing a specific AI agent or running simulations, the new crate should first be added as a member of the existing Cargo workspace. This is done by editing the root Cargo.toml and adding the path to the new crate under the [workspace] members section. Once the crate is part of the workspace, its own Cargo.toml file can specify dependencies on the internal bomberman_ai crates using path dependencies. For example, to use the core grid functionality, the bot logic, and the new RL capabilities, the new crate's Cargo.toml would include:

```toml
[dependencies]
bomberman_state = { path = "../crates/state" }
bomberman_engine = { path = "../crates/engine" }
bomberman_rl = { path = "../crates/rl" }
```

This setup allows the new crate to access the public APIs of these internal libraries directly. It leverages Cargo's workspace feature to ensure that all crates are built with consistent compiler versions and dependency resolutions, simplifying the development and build process for projects that build upon this AI framework.

### 14.2 Safety and API Surface Considerations

A key design principle for the bomberman_ai framework is to maintain a high degree of safety and a minimal, well-defined API surface for its constituent crates. The #![forbid(unsafe_code)] attribute will be used at the top of each crate's lib.rs (except for the optional ffi crate, where unsafe might be necessary for interfacing with C libraries or system calls). This strict prohibition of unsafe code ensures that the vast majority of the framework benefits from Rust's memory safety guarantees, significantly reducing the risk of undefined behavior, data races, and other common vulnerabilities associated with systems programming. By keeping the API surface of each crate minimal and focused, the framework becomes easier to learn, use, and maintain. It also reduces the likelihood of breaking changes when internal implementations are modified, as long as the public API remains stable. This disciplined approach to API design and safety contributes to the overall robustness and reliability of the AI agents built using this framework.

(End of document)
