# schema-codegen

A code generation tool that utilizes FlatBuffers schemas (`.fbs`) as the Single Source of Truth (SSoT) to automatically generate Rust structures (such as ECS Components), C++ headers, and `cxx` bridge definitions.

This project is decoupled from the main `Engine-Agnostic-Rust-Template` repository to ensure reusability across other future system-level projects.

## Architecture

- `crates/schema-ir` — Parses `.fbs` files and custom annotations, converting them into a language-agnostic Intermediate Representation (IR).
- `crates/codegen-rust` — Generates Rust structures from the IR.
- `crates/codegen-cxx` — Generates C++ headers and `cxx` bridge definitions from the IR.
- `crates/cli` — An executable binary that orchestrates the sub-crates listed above.
- `python/` — Handles schema validation prototyping, IR debug visualization, and build orchestration (via `nox`).

## Current Status

Only the build system configurations have been established. Implementation has not yet begun (all crates currently serve as placeholders).

## Expected Consumption by the Main Project

```toml
[dependencies]
schema-codegen-cli = { git = "https://github.com/you/schema-codegen", tag = "v0.1.0" }
```

For active local development, temporarily override the dependency path as follows:

```toml
[patch."https://github.com/you/schema-codegen"]
schema-codegen-cli = { path = "../schema-codegen/crates/cli" }
```
