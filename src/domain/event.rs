use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: Uuid,
    pub event_type: String,
    pub payload: Vec<u8>,
    pub timestamp: u64,
}