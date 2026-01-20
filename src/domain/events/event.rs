use crate::domain::events::event_kind::{EventId, EventKind, EventPayload, Timestamp};

use serde::{Deserialize, Serialize};

/// Represents an Event in the Graveyar_DB system.
///
/// An event is an immutable record of something that happened in the domain.
/// It contains a unique ID, a stream ID, a sequence number within that stream,
/// the event type/kind, the data payload, a timestamp, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unique identifier for the event.
    pub id: EventId,

    /// The ID of the stream this event belongs to.
    pub stream_id: String,

    /// The monotonic sequence number of this event within its stream.
    /// This is assigned by the storage engine upon persistence.
    pub sequence_number: u64,

    /// The type of the event (e.g., "UserCreated").
    pub event_type: EventKind,

    /// The binary payload of the event.
    pub payload: EventPayload,

    /// The wall-clock time when the event was created/ingested.
    pub timestamp: Timestamp,

    /// Additional context key-value pairs (e.g., Tracing info, Saga state).
    /// This allows evolution of process logic without changing the payload schema.
    pub metadata: std::collections::HashMap<String, String>,
}

impl Event {
    /// Creates a new `Event` instance with a generated ID and timestamp.
    /// The `sequence_number` is initialized to 0 and should be set during persistence.
    pub fn new(stream_id: impl Into<String>, event_type: EventKind, payload: EventPayload) -> Self {
        Self {
            id: EventId::new(),
            stream_id: stream_id.into(),
            sequence_number: 0, // Assigned by storage
            event_type,
            payload,
            timestamp: Timestamp::now(),
            metadata: std::collections::HashMap::new(),
        }
    }
}
