use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventId(pub Uuid);

impl EventId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamp(pub u64);

impl Timestamp {
    pub fn now() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Self(since_the_epoch.as_millis() as u64)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPayload(pub Vec<u8>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventKind {
    Internal,
    Schematic,
    Transactional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaType(pub String);
