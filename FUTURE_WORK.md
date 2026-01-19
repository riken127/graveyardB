# Future Work & Roadmap

GraveyardDB has reached a stable MVP state with:
- **Hybrid Storage**: ScyllaDB (Primary) + RocksDB (Local).
- **Cluster**: Distributed forwarding with Consistent Hashing.
- **Worker Pool**: High-concurrency local event processing (actor model).
- **SDKs**: Java (Schema support) and Go.

## 🚀 Near-Term Improvements

### 1. Telemetry (Deferred)
- **Objective**: Re-enable OpenTelemetry integration.
- **Challenge**: Dependency churn in `opentelemetry` Rust crate (0.24 vs 0.27 vs 0.31).
- **Action**: Wait for stabilization or pin exact compatible versions for `opentelemetry`, `tracing-opentelemetry`, and `opentelemetry-otlp`.

### 2. Cluster Membership
- **Objective**: Dynamic node discovery.
- **Current**: Static list in `CLUSTER_NODES`.
- **Action**: Implement Gossip protocol (SWIM) or integration with Etcd/Consul.

### 3. Snapshotting
- **Objective**: Faster replay for long streams.
- **Action**: Implement periodic snapshots of aggregate state to Scylla/S3.

### 4. Compaction
- **Objective**: Clean up old events or soft deletions.
- **Action**: Implement retention policies (TTL).

### 5. Advanced Querying
- **Objective**: Projections and Read Models.
- **Action**: Implement a "Projection Engine" that consumes the stream and updates read-optimized views (e.g., Postgres, ElasticSearch).

## 🛡 Security
- Enable TLS for gRPC (Server-side and mTLS).
- Authentication via JWT/OAuth2.

## 🧪 Testing
- Write property-based tests (`proptest`) for the EventStore trait.
- Run Jepsen tests to verify linearizability during network partitions.
