# Rust runtime

`aep-core` owns domain records and deterministic constraints. `aep-store` owns UTF-8 canonical files, optimistic revision checks, and recoverable transactions. `aep-cli` owns commands, embedded guidance, and explicit Git/process/provider adapters. The working agent owns intent and skill selection.

Use the pinned toolchain and Cargo.lock. Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace`. CLI fixtures use disposable Git repositories and actual subprocesses. Provider tests use isolated mocks; label them as mocks. Optional OpenSpec reference checks use 1.12.0.

Keep read commands free of project mutations. Tie verification to exact candidate inputs, retain failed/stale evidence, and reconcile external outcomes from recorded intent. A completion label is not verification evidence. Preserve standard/deep independent review and the two-round limit.

Native CI targets Linux and macOS. Keep large/binary evidence outside managed text stores and link it with digest and provenance. The CLI is not an OS sandbox or an identity/authorization service. Project check configuration is executable project policy.

The dashboard consumes the CLI JSON contract. It does not implement readiness or promotion rules in TypeScript.
