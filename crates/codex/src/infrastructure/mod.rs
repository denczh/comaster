//! Infrastructure layer for Codex.
//!
//! Implements the Foundry VTT adapter: HTTP/WebSocket client for the Foundry REST API,
//! response deserialization, and reconnection logic.
//! This is the only layer that performs network I/O.
