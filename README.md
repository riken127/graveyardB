# GraveyardDB

A distributed, high-performance Event Store backed by ScyllaDB and RocksDB.

## Architecture

*   **Primary Storage**: ScyllaDB (Cluster)
*   **Fallback Storage**: RocksDB (Local)
*   **Communication**: gRPC
*   **Concurrency**: Local Worker Pool with Consistent Hashing (Sharding)

## Features

*   **Hybrid Storage**: Writes default to ScyllaDB. Automatically falls back to local RocksDB if the cluster is unreachable to ensure availability.
*   **Sequential Processing**: Uses a sharded worker pool to ensure linearizable stream processing without database locks.
*   **Distributed Clustering**:
    *   **Consistent Hashing**: Streams are deterministically sharded across nodes.
    *   **Forwarding**: Nodes forward requests to the responsible peer via gRPC.
*   **Schema Governance**: Protobuf-based schema validation with immutable schema versioning stored in `$schema` streams.

## Getting Started

### Prerequisites
*   Rust (1.75+)
*   Docker & Docker Compose

### Running Helper
Start the 2-node cluster and ScyllaDB:
```bash
docker-compose up -d
```

### Manual execution
```bash
# Single node default
cargo run --release

# Environment Variables
SCYLLA_URI=127.0.0.1:9042
SCYLLA_KEYSPACE=graveyard
CLUSTER_NODES=127.0.0.1:50051,127.0.0.1:50052
NODE_ID=0
PORT=50051
DB_PATH=data/rocksdb
```

## Benchmarks

System: Local Docker Cluster (2 Nodes), 50 Concurrent Workers.

| Metric | Value | Note |
|:-------|:------|:-----|
| **Throughput (Single)** | ~3,128 ops/sec | RocksDB (Baseline) |
| **Throughput (Cluster)**| ~2,887 ops/sec | 2 Nodes + Forwarding |
| **Failover Rate** | ~11,560 ops/sec | ScyllaDB Down -> RocksDB Fallback |

## SDKs

*   **Java**: `com.eventstore.client` (Stable)
*   **Go**: `graveyar_db/go` (Beta)
*   **TypeScript**: `@graveyar_db/client` (Beta)
