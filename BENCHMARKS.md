# Performance Benchmarks

This document details the performance characteristics of GraveyardDB under various conditions.

## Methodology
- **Tool**: `stress_test` binary (Rust).
- **Environment**: Local Docker Cluster (2 Nodes).
- **Workload**: 50,000 events, 50 concurrent workers, random payloads.
- **Hardware**: Local Development Machine (equivalent to AWS t3.medium).

## Results

### 1. Single Node (Baseline)
Pure Writes to RocksDB.

| Metric | Result |
|:-------|:-------|
| **Throughput** | **~3,128 events/sec** |
| **Latency** | < 1ms |
| **Success Rate**| 100% |

### 2. Distributed Cluster (2 Nodes)
Writes with Consistent Hashing and gRPC Forwarding.

| Metric | Result | Impact |
|:-------|:-------|:-------|
| **Throughput** | **~2,887 events/sec** | -7.7% vs Single Node |
| **Latency** | < 1.2ms | Negligible overhead |

### 3. High Availability / Failover
Scenario: Primary ScyllaDB cluster is forcibly stopped (`docker stop`).

| Metric | Result | Implication |
|:-------|:-------|:------------|
| **Throughput** | **~11,560 events/sec** | Failures flush to local disk immediately. |
| **Availability**| **100%** | No writes were rejected. |
| **Behavior** | Automatic fallback from Network I/O -> Local Disk I/O. |

## Conclusion
GraveyardDB provides predictable performance with minimal overhead for distributed features. The Hybrid Storage architecture ensures that **availability maximizes during outages**, effectively "failing open" to local storage.
