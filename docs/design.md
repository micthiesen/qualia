# Qualia design

The full specification is the Qualia artifact:
https://michael.artifacts.boo/a/a8fyyvvytt9ff (private). It is the source of
truth for product behavior; this file records only what the code depends on.

## Product in one paragraph

A private art-direction instrument for abstract motion set to music. Michael
recognizes what feels right through A/B comparisons (with Neither, Both,
Skip, Keep this moment, and aspect-scoped responses); the system invents,
simulates, compares and assembles. The foundation is a growing language of
interacting visual worlds, not a bank of effects or a prompt box.

## Architecture (recommended route: code-native)

- World engine: Rust on wgpu, shared by native rendering and the browser
  (WebAssembly) where capabilities allow.
- Studio: React and TypeScript, one large stage with an A/B rail.
- Workers: Python for audio analysis (Essentia) and preference research
  (BoTorch PairwiseGP), separate from the engine.
- Storage: local SQLite plus a content-addressed asset store; append-only
  decision history.

## Contracts already in code

`qualia-core` holds the world intermediate representation:

- Port families: signal, event, field, entity-set, geometry, material, state.
  Connections never convert between families; conversions are explicit
  adapter operators.
- Each input has at most one driver.
- Within one step, immediate edges are acyclic. Feedback goes through delayed
  edges that read the previous step. The schedule is deterministic
  (declaration order breaks ties).

## Delivery gates

A. Three or more coupled worlds and six clips covering sparse, dense, slow,
precise, unstable and materially complex behavior.
B. Full comparison vocabulary, scoped pins, branches; a meaningful 20 to 40
second passage in roughly 10 to 15 choices.
C. A coherent full-track piece from sparse anchors, with preserved pins and
reproducible seeking.
D. A new operator added through the invention pipeline that creates
previously unavailable behavior.
E. Durable daily tool: library, export, recovery, migrations, private gallery.

Everything stays private and local by default. No public site, upload, or
remote model use without a separate decision.
