//! Application layer for Codex.
//!
//! Use cases for querying Foundry VTT: reading actor sheets, resolving roll results,
//! and applying YZE rule doctrine to produce structured answers for the co-pilot.
//! Depends only on domain ports; the concrete HTTP adapter is in infrastructure.
