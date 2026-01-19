use crate::api::event_store_client::EventStoreClient;
use crate::api::{AppendEventRequest, Event as ProtoEvent};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::transport::Channel;

#[derive(Clone)]
pub struct ClusterClient {
    clients: Arc<RwLock<HashMap<String, EventStoreClient<Channel>>>>,
}

impl ClusterClient {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for ClusterClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ClusterClient {
    pub async fn get_client(&self, addr: &str) -> Result<EventStoreClient<Channel>, String> {
        // Fast path: read lock
        {
            let map = self.clients.read().await;
            if let Some(client) = map.get(addr) {
                return Ok(client.clone());
            }
        }

        // Slow path: connect and write lock
        let mut map = self.clients.write().await;
        // Check again in case of race
        if let Some(client) = map.get(addr) {
            return Ok(client.clone());
        }

        let uri = format!("http://{}", addr); // Assume HTTP/2 without TLS for internal cluster for MVP
        let channel = Channel::from_shared(uri)
            .map_err(|e| e.to_string())?
            .connect()
            .await
            .map_err(|e| format!("Failed to connect to peer {}: {}", addr, e))?;

        let client = EventStoreClient::new(channel);
        map.insert(addr.to_string(), client.clone());

        Ok(client)
    }

    pub async fn forward_append(
        &self,
        target_node: &str,
        stream_id: &str,
        events: Vec<crate::domain::events::event::Event>,
        expected_version: i64,
    ) -> Result<bool, String> {
        let mut client = self.get_client(target_node).await?;

        // Convert Domain Events to Proto Events
        let proto_events: Vec<ProtoEvent> = events.into_iter().map(|e| e.into()).collect();

        let req = AppendEventRequest {
            stream_id: stream_id.to_string(),
            events: proto_events,
            expected_version: expected_version as u64, // Potential type mismatch if i64 vs u64, need to check proto
        };

        let resp = client
            .append_event(req)
            .await
            .map_err(|e| e.to_string())?
            .into_inner();

        Ok(resp.success)
    }
}
