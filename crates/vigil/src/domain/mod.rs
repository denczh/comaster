//! Domain layer for Vigil.
//!
//! Contains entities for the knowledge-and-witness system: facts, witnesses,
//! and the append-only knowledge register. Enforces immutability invariants
//! (entries cannot be retracted, only superseded) with no external dependencies.
