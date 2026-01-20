use crate::storage::snapshot::{Snapshot, SnapshotError, SnapshotStore};
use rocksdb::DB;
use std::sync::Arc;
use tonic::async_trait;

pub struct RocksSnapshotStore {
    db: Arc<DB>,
}

impl RocksSnapshotStore {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SnapshotStore for RocksSnapshotStore {
    async fn save_snapshot(&self, snapshot: Snapshot) -> Result<(), SnapshotError> {
        let key = format!("snapshot:{}", snapshot.stream_id);

        // We use CBOR for internal storage of the snapshot struct meta + payload
        // Actually, we can just serialize the whole Snapshot struct if we derive Serialize
        // Let's manually pack or use serde.
        // Snapshot struct defined in `storage::snapshot` needs Serialize.
        // For MVP, I'll use a simple tuple serialization or just rely on serde_cbor if I add Serialize to Snapshot.

        // For now, let's manually serialize: version + timestamp + payload
        // But better to use structured.
        // I will assume Snapshot derives Serialize in the trait file? No, I defined it without derives.
        // I should update `Snapshot` definition to derive Serialize/Deserialize.

        let mut buf = Vec::new();
        // Simple binary format: [version:8][timestamp:8][payload...]
        buf.extend_from_slice(&snapshot.version.to_be_bytes());
        buf.extend_from_slice(&snapshot.timestamp.to_be_bytes());
        buf.extend_from_slice(&snapshot.payload);

        // Write to RocksDB
        self.db
            .put(key, buf)
            .map_err(|e| SnapshotError::StorageError(e.to_string()))?;

        Ok(())
    }

    async fn get_snapshot(&self, stream_id: &str) -> Result<Option<Snapshot>, SnapshotError> {
        let key = format!("snapshot:{}", stream_id);

        match self
            .db
            .get(key)
            .map_err(|e| SnapshotError::StorageError(e.to_string()))?
        {
            Some(bytes) => {
                if bytes.len() < 16 {
                    return Ok(None); // Invalid
                }

                let (ver_bytes, rest) = bytes.split_at(8);
                let (ts_bytes, payload) = rest.split_at(8);

                let version = u64::from_be_bytes(ver_bytes.try_into().unwrap());
                let timestamp = u64::from_be_bytes(ts_bytes.try_into().unwrap());

                Ok(Some(Snapshot {
                    stream_id: stream_id.to_string(),
                    version,
                    payload: payload.to_vec(),
                    timestamp,
                }))
            }
            None => Ok(None),
        }
    }
}
