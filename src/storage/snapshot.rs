use tonic::async_trait;

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub stream_id: String,
    pub version: u64,
    pub payload: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum SnapshotError {
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_cbor::Error),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[async_trait]
pub trait SnapshotStore: Send + Sync {
    async fn save_snapshot(&self, snapshot: Snapshot) -> Result<(), SnapshotError>;
    async fn get_snapshot(&self, stream_id: &str) -> Result<Option<Snapshot>, SnapshotError>;
}
