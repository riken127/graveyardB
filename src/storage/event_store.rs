use crate::domain::events::event::Event;
use tonic::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum EventStoreError {
    #[error("Stream not found")]
    NotFound,
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_cbor::Error),
    #[error("Concurrency conflict: expected version {expected}, actual {actual}")]
    ConcurrencyError { expected: u64, actual: u64 },
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Abstract storage interface for persistence.
///
/// Implementations (e.g., RocksDB, ScyllaDB, Memory) must ensure:
/// 1. Atomicity of append operations.
/// 2. Strict ordering of sequence numbers within a stream.
/// 3. Persistence of data to durable media (except MemoryStore).
#[async_trait]
pub trait EventStore: Send + Sync {
    /// Appends a single event to a stream.
    ///
    /// Must enforce Optimistic Concurrency Control using `expected_version`.
    /// If `expected_version` does not match the current stream version, `ConcurrencyError` is returned.
    async fn append_event(
        &self,
        stream: &str,
        event: Event,
        expected_version: u64,
    ) -> Result<(), EventStoreError>;

    /// Retrieves all events for a given stream, ordered by sequence number.
    async fn fetch_stream(&self, stream: &str) -> Result<Vec<Event>, EventStoreError>;

    /// Registers or updates a schema.
    async fn upsert_schema(
        &self,
        schema: crate::domain::schema::model::Schema,
    ) -> Result<(), EventStoreError>;

    /// Retrieves a schema by name.
    async fn get_schema(
        &self,
        name: &str,
    ) -> Result<Option<crate::domain::schema::model::Schema>, EventStoreError>;
}
