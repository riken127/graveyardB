use rocksdb::{IteratorMode, Options, DB};
use tonic::async_trait;

use crate::{
    domain::events::event::Event,
    storage::event_store::{EventStore, EventStoreError},
};

pub struct RocksEventStore {
    db: DB,
}

impl RocksEventStore {
    pub fn new(path: &str) -> Result<Self, EventStoreError> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path).map_err(|e| EventStoreError::StorageError(e.to_string()))?;
        Ok(Self { db })
    }
}

#[async_trait]
impl EventStore for RocksEventStore {


    async fn append_event(&self, stream: &str, event: Event) -> Result<(), EventStoreError> {
        let key = format!("stream:{}:{}", stream, event.id.0);
        let value = serde_cbor::to_vec(&event)?;

        self.db
            .put(key, value)
            .map_err(|e| EventStoreError::StorageError(e.to_string()))?;
        Ok(())
    }

    async fn fetch_stream(&self, stream: &str) -> Result<Vec<Event>, EventStoreError> {
        let prefix = format!("stream:{}:", stream);
        let mut events = Vec::new();
        // iter works with bytes, so we need to filter keys manually if we use prefix,
        // or usage prefix_iterator if available, but here we can just iterate start and check.
        // Actually rocksdb crate has prefix_iterator but let's stick to simple iterator for now as in code
        // assuming standard iterator usage. Code was using 'prefix.deserialize()' which is wrong.

        // Correct approach with prefix scan:
        let mode = IteratorMode::From(prefix.as_bytes(), rocksdb::Direction::Forward);

        for item in self.db.iterator(mode) {
            let (key, value) = item.map_err(|e| EventStoreError::StorageError(e.to_string()))?;
            // We need to convert key to string to check prefix or check bytes
            if !key.starts_with(prefix.as_bytes()) {
                break;
            }

            let event: Event = serde_cbor::from_slice(&value)?;
            events.push(event);
        }
        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::events::event_kind::{EventKind, EventPayload};
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_rocks_persistence() {
        let temp_dir = TempDir::new().expect("failed to create temp dir");
        let db_path = temp_dir.path().to_str().unwrap();

        let payload = EventPayload(vec![1, 2, 3]);
        let event = Event::new("stream-p", EventKind::Internal, payload.clone());

        {
            let store = RocksEventStore::new(db_path).expect("failed to open db");
            store.append_event("stream-p", event.clone()).await.expect("failed to append");
        } // store dropped, db closed

        {
            let store = RocksEventStore::new(db_path).expect("failed to reopen db");
            let loaded = store.fetch_stream("stream-p").await.expect("failed to fetch");
            assert_eq!(loaded.len(), 1);
            assert_eq!(loaded[0].id.0, event.id.0);
            assert_eq!(loaded[0].payload.0, payload.0);
        }
    }

    #[tokio::test]
    async fn test_stream_ordering() {
        let temp_dir = TempDir::new().expect("failed to create temp dir");
        let db_path = temp_dir.path().to_str().unwrap();
        let store = RocksEventStore::new(db_path).expect("failed to open db");

        let event1 = Event::new("stream-o", EventKind::Internal, EventPayload(vec![1]));
        // Sleep slightly to ensure v7 time diff works if relying on time, 
        // though v7 has sub-ms precision and counter.
        // But let's just create them.
        let event2 = Event::new("stream-o", EventKind::Internal, EventPayload(vec![2]));

        store.append_event("stream-o", event1.clone()).await.expect("failed to append 1");
        store.append_event("stream-o", event2.clone()).await.expect("failed to append 2");

        let loaded = store.fetch_stream("stream-o").await.expect("failed to fetch");
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].id.0, event1.id.0);
        assert_eq!(loaded[1].id.0, event2.id.0);
    }
}
