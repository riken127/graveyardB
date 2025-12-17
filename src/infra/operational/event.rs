use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum OperationalEvent {
    EventAppendFailed { stream_id: String, error: String },
    RetryExhausted { operation: String, attempts: u32 },
}