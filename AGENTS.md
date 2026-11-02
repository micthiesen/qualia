# Qualia

A private, preference-led studio that renders abstract motion for music.
The design lives in the Qualia artifact
(https://michael.artifacts.boo/a/a8fyyvvytt9ff); `docs/design.md` summarizes
what the code depends on. Read both before product or architecture work.

## Layout

- `crates/qualia-core`: sans-I/O world intermediate representation: typed
  operators, ports, immediate and delayed edges, validation and the step
  schedule. Tests in `crates/qualia-core/tests/`.
- `crates/qualia`: thin CLI (`qualia check <world.json>`).
- `crates/xtask`: `cargo xtask verify`.
- `worlds/`: example world graphs used as test fixtures.

Planned per the design: a wgpu engine crate, a React/TypeScript studio, and
Python analysis/search workers. Add them as separate crates or top-level
directories; `qualia-core` must stay free of GPU, I/O, UI and AI provider
dependencies.

## Conventions

- Stable Rust pinned in `rust-toolchain.toml`, edition 2024, Clippy at
  `-D warnings`.
- Strong types and explicit enums over flags or stringly typed state; small
  focused modules.
- No `unwrap`/`expect` outside tests, no `dbg!`, no `println!` outside the CLI
  and xtask. Log with `tracing`.
- `thiserror` in libraries, `anyhow` only at binary and xtask edges.
- Keep determinism: explicit seeds, fixed steps, declaration-order tie breaks.
  No language-model call in a per-frame path.
- Media and preference history stay local by default.
- Write docs in plain language without em dashes.

## Validation

```bash
cargo fmt --all
cargo xtask verify
```

## Working here

- Personal project: commit and push directly to `main` after `verify` passes.
- Sibling Rust projects (`~/Code/oopsillator`, `~/Code/beastie`,
  `~/Code/omni`) are the reference for evolving conventions.
- Update this file as conventions emerge.
