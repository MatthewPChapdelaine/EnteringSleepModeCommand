# EnteringSleepModeCommand

This repository now contains a runnable Rust game engine that turns the original design document into a small, testable engine runtime.

## What’s included

- A Rust binary crate with an interactive command loop
- A lightweight ECS-inspired world with transforms, rigid bodies, logic nodes, and renderables
- A gravity simulation system and an entity logic pipeline
- Frame stepping plus render snapshots so the engine can advance over time
- Tests covering sleep-mode activation, world construction, frame updates, and logic transitions

## Run it

```bash
cargo run
```

You can enter commands such as:

- Enter Sleep Mode
- Build
- quit

## Test it

```bash
cargo test
```

## Notes

The implementation is intentionally minimal and dependency-free so it can be built and explored quickly while staying faithful to the original concept of an autonomous, regulated system that can enter sleep mode and wake into a build phase.
