pub mod command;
pub mod worker;

use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::storage::event_store::EventStore;
use crate::domain::events::event::Event;
use crate::pipeline::command::PipelineCommand;
use crate::pipeline::worker::Worker;
use crate::cluster::NodeSelector;
use crate::cluster::client::ClusterClient;

const NUM_WORKERS: usize = 32;

pub struct EventPipeline {
    storage: Arc<dyn EventStore + Send + Sync>,
    workers: Vec<mpsc::Sender<PipelineCommand>>,
    node_selector: NodeSelector,
    cluster_client: ClusterClient,
    self_addr: String,
}

impl EventPipeline {
    pub fn new(
        storage: Arc<dyn EventStore + Send + Sync>,
        cluster_nodes: Vec<String>,
        self_node_id: u64,
    ) -> Self {
        let mut workers = Vec::with_capacity(NUM_WORKERS);

        for id in 0..NUM_WORKERS {
            let (tx, rx) = mpsc::channel::<PipelineCommand>(1024);
            let store = storage.clone();
            let worker = Worker::new(id, store);

            tokio::spawn(async move {
                worker.run(rx).await;
            });
            workers.push(tx);
        }

        let node_selector = NodeSelector::new(cluster_nodes.clone());
        // Determine self address based on ID, safe fallback if config is weird
        let sorted_nodes = node_selector.get_all_nodes();
        let self_addr = if (self_node_id as usize) < sorted_nodes.len() {
            sorted_nodes[self_node_id as usize].clone()
        } else {
            // Fallback for single node dev mode if config mismatch, assuming first one
            sorted_nodes.first().cloned().unwrap_or_else(|| "127.0.0.1:50051".to_string())
        };
        
        let cluster_client = ClusterClient::new();

        Self { 
            storage, 
            workers,
            node_selector,
            cluster_client,
            self_addr
        }
    }

    #[tracing::instrument(skip(self, events), fields(stream_id = %stream_id, event_count = events.len()))]
    pub async fn append_event(
        &self,
        stream_id: &str,
        events: Vec<Event>,
        expected_version: i64,
    ) -> Result<bool, String> {
        // 1. Determine Owner
        let target_node = self.node_selector.get_node_for_stream(stream_id);
        
        // 2. Route
        if target_node == self.self_addr {
            // Local Processing via Sharded Workers
            let mut hasher = DefaultHasher::new();
            stream_id.hash(&mut hasher);
            let hash = hasher.finish();
            let worker_idx = (hash as usize) % self.workers.len();
            
            let (resp_tx, resp_rx) = oneshot::channel();
            
            let cmd = PipelineCommand::Append {
                stream_id: stream_id.to_string(),
                events,
                expected_version,
                resp_tx,
            };
            
            self.workers[worker_idx].send(cmd).await.map_err(|e| e.to_string())?;
            
            resp_rx.await.map_err(|e| e.to_string())?
        } else {
            // Remote Forwarding

            self.cluster_client.forward_append(&target_node, stream_id, events, expected_version).await
        }
    }

    pub async fn fetch_stream(&self, stream_id: &str) -> Result<Vec<Event>, String> {
        self.storage.fetch_stream(stream_id).await.map_err(|e| e.to_string())
    }

    pub async fn upsert_schema(&self, schema: crate::domain::schema::model::Schema) -> Result<(), String> {
        self.storage.upsert_schema(schema).await.map_err(|e| e.to_string())
    }

    pub async fn get_schema(&self, name: &str) -> Result<Option<crate::domain::schema::model::Schema>, String> {
        self.storage.get_schema(name).await.map_err(|e| e.to_string())
    }
}
