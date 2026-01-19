use std::collections::HashMap;
use std::sync::RwLock;
use tonic::async_trait;

use crate::{
    domain::events::event::Event,
    storage::event_store::{EventStore, EventStoreError},
};

#[derive(Debug, Default)]
pub struct InMemoryEventStore {
    // Key: stream_id, Value: List of events
    store: RwLock<HashMap<String, Vec<Event>>>,
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl EventStore for InMemoryEventStore {
    async fn append_event(&self, stream: &str, event: Event) -> Result<(), EventStoreError> {
        let mut store = self
            .store
            .write()
            .map_err(|_| EventStoreError::Unknown("Lock poison".to_string()))?;
        
        store
            .entry(stream.to_string())
            .or_insert_with(Vec::new)
            .push(event);
            
        Ok(())
    }

    async fn fetch_stream(&self, stream: &str) -> Result<Vec<Event>, EventStoreError> {
        let store = self
            .store
            .read()
            .map_err(|_| EventStoreError::Unknown("Lock poison".to_string()))?;

        match store.get(stream) {
            Some(events) => Ok(events.clone()),
            None => Ok(Vec::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::events::event_kind::{EventKind, EventPayload};

    #[tokio::test]
    async fn test_append_and_load() {
        let store = InMemoryEventStore::new();
        let payload = EventPayload(vec![1, 2, 3]);
        let event = Event::new("stream-1", EventKind::Internal, payload);

        store.append_event("stream-1", event.clone()).await.expect("Append failed");

        let loaded = store.fetch_stream("stream-1").await.expect("Load failed");
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id.0, event.id.0);
    }
    
    #[tokio::test]
    async fn test_load_empty() {
        let store = InMemoryEventStore::new();
        let loaded = store.fetch_stream("non-existent").await.expect("Load failed");
        assert!(loaded.is_empty());
    }
}
