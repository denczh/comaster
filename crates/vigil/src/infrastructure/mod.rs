//! Infrastructure layer for Vigil.
//!
//! Implements persistence ports for the append-only knowledge register:
//! file-system append logs and any future event-store adapters.
//! This layer is the only place that writes to durable storage.
