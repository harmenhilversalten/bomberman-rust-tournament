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

