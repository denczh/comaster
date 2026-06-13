//! Infrastructure layer for Pharos.
//!
//! Implements the persistence and I/O ports defined by the domain:
//! file-system repositories for world data (YAML/binary), caching strategies,
//! and any external geography data adapters.
//! This layer is the only place that interacts with the file system or network.
