use crate::domain::events::event::Event;
use crate::domain::schema::model::Schema;
use crate::storage::event_store::{EventStore, EventStoreError};
use std::sync::Arc;
use tonic::async_trait;
use tracing::warn;

pub struct HybridEventStore {
    primary: Arc<dyn EventStore>,
    fallback: Arc<dyn EventStore>,
}

impl HybridEventStore {
    pub fn new(primary: Arc<dyn EventStore>, fallback: Arc<dyn EventStore>) -> Self {
        Self { primary, fallback }
    }
}

#[async_trait]
impl EventStore for HybridEventStore {
    async fn append_event(&self, stream: &str, event: Event) -> Result<(), EventStoreError> {
        // Try Primary
        match self.primary.append_event(stream, event.clone()).await {
            Ok(_) => Ok(()),
            Err(e) => {
                warn!("Primary Storage failed during append: {}. Falling back to Secondary.", e);
                // Try Fallback
                self.fallback.append_event(stream, event).await
            }
        }
    }

    async fn fetch_stream(&self, stream: &str) -> Result<Vec<Event>, EventStoreError> {
        // Try Primary
        match self.primary.fetch_stream(stream).await {
            Ok(events) => Ok(events),
            Err(e) => {
                warn!("Primary Storage failed during fetch: {}. Falling back to Secondary.", e);
                self.fallback.fetch_stream(stream).await
            }
        }
    }

    async fn upsert_schema(&self, schema: Schema) -> Result<(), EventStoreError> {
        // For Schema, we might want dual-write or failover.
        // For robustness MVP, we'll do failover to ensure the system keeps working.
        // Ideally, we should attempt to write to both to keep them in sync if possible, 
        // but that complicates partial failure handling.
        
        match self.primary.upsert_schema(schema.clone()).await {
            Ok(_) => {
                // Determine if we should also write to fallback to keep it warm?
                // For now, let's keep it simple: Failover only.
                // If primary writes, we assume primary is source of truth.
                Ok(())
            },
            Err(e) => {
                warn!("Primary Storage failed during upsert_schema: {}. Falling back to Secondary.", e);
                self.fallback.upsert_schema(schema).await
            }
        }
    }

    async fn get_schema(&self, name: &str) -> Result<Option<Schema>, EventStoreError> {
        match self.primary.get_schema(name).await {
            Ok(res) => Ok(res),
            Err(e) => {
                warn!("Primary Storage failed during get_schema: {}. Falling back to Secondary.", e);
                self.fallback.get_schema(name).await
            }
        }
    }
}
