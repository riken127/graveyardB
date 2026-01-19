use crate::api as proto;
use crate::domain::events::event::Event;
use crate::domain::events::event_kind::{EventId, EventKind, EventPayload, Timestamp};

impl TryFrom<proto::Event> for Event {
    type Error = String;

    fn try_from(proto_event: proto::Event) -> Result<Self, Self::Error> {
        // Parse event_type string to EventKind
        // Assuming the proto string matches the Debug output of the enum variants
        let event_type = match proto_event.event_type.as_str() {
            "Internal" => EventKind::Internal,
            "Schematic" => EventKind::Schematic,
            "Transactional" => EventKind::Transactional,
            _ => EventKind::Internal, // Default/Fallback to avoid failure on unknown types for now
        };

        use std::str::FromStr;
        Ok(Event {
            id: EventId(uuid::Uuid::from_str(&proto_event.id).map_err(|e| e.to_string())?),
            stream_id: String::new(), // Placeholder, caller context usually provides this
            event_type,
            payload: EventPayload(proto_event.payload),
            timestamp: Timestamp(proto_event.timestamp),
        })
    }
}

impl From<Event> for proto::Event {
    fn from(domain_event: Event) -> Self {
        let event_type_str = format!("{:?}", domain_event.event_type);
        proto::Event {
            id: domain_event.id.0.to_string(),
            event_type: event_type_str,
            payload: domain_event.payload.0,
            timestamp: domain_event.timestamp.0,
        }
    }
}
