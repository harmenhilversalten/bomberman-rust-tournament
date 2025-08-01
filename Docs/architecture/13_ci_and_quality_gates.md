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

