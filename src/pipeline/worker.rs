use crate::domain::events::event::Event;
use crate::pipeline::command::PipelineCommand;
use crate::storage::event_store::EventStore;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct Worker {
    _id: usize,
    store: Arc<dyn EventStore + Send + Sync>,
}

impl Worker {
    pub fn new(_id: usize, store: Arc<dyn EventStore + Send + Sync>) -> Self {
        Self { _id, store }
    }

    pub async fn run(self, mut rx: mpsc::Receiver<PipelineCommand>) {
        while let Some(cmd) = rx.recv().await {
            match cmd {
                PipelineCommand::Append {
                    stream_id,
                    mut events,
                    expected_version,
                    resp_tx,
                } => {
                    let res = self
                        .handle_append(&stream_id, &mut events, expected_version)
                        .await;
                    let _ = resp_tx.send(res);
                }
            }
        }
    }

    async fn handle_append(
        &self,
        stream_id: &str,
        events: &mut Vec<Event>,
        expected_version: i64,
    ) -> Result<bool, String> {
        // 1. Fetch current stream to check version if needed
        let current_events = self
            .store
            .fetch_stream(stream_id)
            .await
            .map_err(|e| e.to_string())?;

        let current_version = if current_events.is_empty() {
            -1
        } else {
            (current_events.len() as i64) - 1
        };

        if expected_version != -1 && current_version != expected_version {
            tracing::warn!(
                "Concurrency conflict for stream {}: expected {}, got {}",
                stream_id,
                expected_version,
                current_version
            );
            return Ok(false);
        }

        // 2. Prepare events
        for event in events.iter_mut() {
            event.stream_id = stream_id.to_string();
        }

        // 3. Persist
        for event in events.drain(..) {
            self.store
                .append_event(stream_id, event)
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(true)
    }
}
