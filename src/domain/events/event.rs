use crate::domain::events::event_kind::{EventId, EventKind, EventPayload, Timestamp};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub stream_id: String,
    pub sequence_number: u64,
    pub event_type: EventKind,
    pub payload: EventPayload,
    pub timestamp: Timestamp,
}

impl Event {
    pub fn new(stream_id: impl Into<String>, event_type: EventKind, payload: EventPayload) -> Self {
        Self {
            id: EventId::new(),
            stream_id: stream_id.into(),
            sequence_number: 0, // Assigned by storage
            event_type,
            payload,
            timestamp: Timestamp::now(),
        }
    }
}
