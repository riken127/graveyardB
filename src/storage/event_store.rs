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
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[async_trait]
pub trait EventStore: Send + Sync {
    async fn append_event(&self, stream: &str, event: Event) -> Result<(), EventStoreError>;
    async fn fetch_stream(&self, stream: &str) -> Result<Vec<Event>, EventStoreError>;
}
