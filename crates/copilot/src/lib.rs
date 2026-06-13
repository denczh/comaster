//! Co-pilot orchestration layer.
//!
//! Manages the lifecycle of the Claude AI co-pilot: building context from the five
//! domain components (Pharos, Gens, Codex, Nexus, Vigil), constructing prompts,
//! sending requests to the Anthropic API via `reqwest`, and streaming responses
//! back to the Comaster UI over an async channel.
//!
//! No official Anthropic Rust SDK existed at the time of writing; this crate
//! communicates with the REST API directly using `reqwest` and `serde`.
