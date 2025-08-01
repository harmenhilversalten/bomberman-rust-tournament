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

