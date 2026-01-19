# Project Scope and Purpose

## Purpose

**graveyar_db** is designed to be a robust, high-throughput, and low-latency event store. It serves as the backbone for event-sourced systems, providing a reliable append-only log of immutable events.

The primary goals are:
- **Performance**: Leverage Rust's async capabilities (Tokio) and efficient storage engines (RocksDB, ScyllaDB) to handle high write and read loads.
- **Simplicity**: specific focus on the core "Event Store" responsibilities—appending and reading events.
- **Reliability**: Ensure data durability and consistency.

## Scope

### In Scope
- **Event Appending**: gRPC API to append events to a specific stream.
- **Event Reading**: gRPC API to read events from a stream (forward/backward/streaming).
- **Concurrency Control**: Optimistic locking via `expected_version` checks.
- **Pluggable Storage**: Abstraction layer allowing different storage backends (initially RocksDB, planned ScyllaDB).
- **Stream Management**: Basic stream metadata management.

### Out of Scope
- **Complex Querying**: No complex SQL-like queries or projections within the store itself. This is intended to be handled by downstream consumers (CQRS read models).
- **Authentication/Authorization**: The initial version assumes a trusted network or an external gateway handling auth.
- **Built-in Projections**: Logic to fold events into state is the SDK's/Consumer's responsibility or handled by a separate "Projection Engine" service, not the core store.

## Roadmap
1. **Fase 0 (Foundations)**: Core API, RocksDB storage, basic pipeline.
2. **Fase 1 (ScyllaDB)**: Scalable backend implementation.
3. **Fase 2 (Cluster)**: Distribution and consensus.
