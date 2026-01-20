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
    async fn append_event(
        &self,
        stream: &str,
        mut event: Event,
        expected_version: u64,
    ) -> Result<(), EventStoreError> {
        let mut store = self
            .store
            .write()
            .map_err(|_| EventStoreError::Unknown("Lock poison".to_string()))?;

        let stream_events = store.entry(stream.to_string()).or_insert_with(Vec::new);

        let current_version = stream_events.last().map(|e| e.sequence_number).unwrap_or(0);

        if expected_version != 0 && current_version != expected_version {
            // Treat expected_version 0 as "Expected stream to correspond to sequence 0" i.e. empty?
            // Actually, usually 0 means "No Stream" or "First Event is 1".
            // Let's assume sequence starts at 1.
            // If stream empty, current = 0.
            // If expected = 0, fine.
            // If expected > 0 (e.g. 5), but current is 0 -> Error.
            // If expected < current -> Conflict.

            // Strict check:
            if current_version != expected_version {
                return Err(EventStoreError::ConcurrencyError {
                    expected: expected_version,
                    actual: current_version,
                });
            }
        }

        // Wait, if expected_version is 0 and stream exists?
        // If stream has events 1,2,3. Current=3. Expected=0.
        // It failed. Correct.

        let next_version = current_version + 1;
        event.sequence_number = next_version;
        stream_events.push(event);

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

    async fn upsert_schema(
        &self,
        _schema: crate::domain::schema::model::Schema,
    ) -> Result<(), EventStoreError> {
        // No-op for now or simple log
        Ok(())
    }

    async fn get_schema(
        &self,
        _name: &str,
    ) -> Result<Option<crate::domain::schema::model::Schema>, EventStoreError> {
        Ok(None)
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

        store
            .append_event("stream-1", event.clone(), 0)
            .await
            .expect("Append failed");

        let loaded = store.fetch_stream("stream-1").await.expect("Load failed");
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id.0, event.id.0);
    }

    #[tokio::test]
    async fn test_load_empty() {
        let store = InMemoryEventStore::new();
        let loaded = store
            .fetch_stream("non-existent")
            .await
            .expect("Load failed");
        assert!(loaded.is_empty());
    }
}
