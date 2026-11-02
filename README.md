# Qualia

A private generative art studio that turns how music feels into abstract
motion. You pick between candidates; the system invents, simulates and
composes. See [docs/design.md](docs/design.md) and the full design artifact.

## Status

Early scaffold: `qualia-core` defines and validates world graphs, and the
`qualia` CLI checks them.

```sh
cargo run -p qualia -- check worlds/stress-lattice.json
cargo xtask verify
```
