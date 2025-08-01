## 15. Common Design Patterns

The repository encourages SOLID, modular code. Several design patterns help
structure bots and services effectively. The reference implementation in
[`Docs/examples`](examples/README.md) showcases a
service-oriented layout with lightweight dependency injection. The following
patterns are recommended when building new crates or expanding existing ones.

### 15.1 Strategy for Bot Behavior

Use the **Strategy** pattern when a bot should support interchangeable decision
algorithms. Implement a `BotStrategy` trait with a `decide` method and swap
implementations at runtime via configuration or testing. This keeps the core bot
agnostic of specific heuristics or learning algorithms.

### 15.2 Command for Game Actions

Represent player actions as command objects that encapsulate all data required
to apply them. The existing `Command` enum is a starting point; extending it and
processing commands through a dispatcher keeps game logic decoupled from bot
code.

### 15.3 Builder for Configuration

Configuration structs can grow large as features expand. Employ the **Builder**
pattern to construct them with sensible defaults and optional parameters. This
avoids lengthy constructors and makes tests easier to read.

### 15.4 Dependency Injection for Services

Complex crates often rely on services that can be mocked during testing. Use a
simple dependency injection container, similar to the example crate, to register
service implementations. Inject interfaces into processors rather than creating
them directly so that units remain small and testable.
(End of document)
