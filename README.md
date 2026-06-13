# Comaster

Comaster is a desktop application written in Rust and Slint that assists a human game master (GM) during tabletop role-playing sessions, and serves as a fiction-world authoring environment. It hosts the GM workflow, a Claude-based AI co-pilot, authoring editors, and — in future iterations — an embedded Claude Code terminal.

## Components

| Crate | Description |
|-------|-------------|
| **pharos** | Universe, geography, locations, connections, and nested levels of detail (LOD), inspired by OpenStreetMap extended for narrative. |
| **gens** | Creatures, objects, bonds, motivations, and NPC narrative voice. |
| **codex** | Adapter to Foundry VTT for querying table state, character sheets, and rolls; includes a Year Zero Engine (YZE) rule-doctrine layer. |
| **nexus** | Narrative structure: acts, scenes, open threads, beats, and tension clocks. |
| **vigil** | Knowledge and witnesses: who knows what, when, in an append-only register. |

Each component exposes a library crate for direct use by the Comaster application and a binary crate (`<component>-mcp`) that runs as an MCP server for the AI co-pilot.

Additional crates:

- **shared** — Cross-cutting identifiers and error types.
- **copilot** — Orchestration layer for the Claude AI co-pilot.
- **ui** — Reusable Slint UI components.

## Building

```sh
cargo build --workspace
cargo run -p comaster
```

Requires a stable Rust toolchain. Install via [rustup](https://rustup.rs/).

## Development

```sh
just fmt      # format
just clippy   # lint
just test     # test suite
just run-mcp pharos   # start the Pharos MCP server
```
