pub mod command;
pub mod worker;

use crate::cluster::client::ClusterClient;
use crate::cluster::ClusterTopology;
use crate::domain::events::event::Event;
use crate::pipeline::command::PipelineCommand;
use crate::pipeline::worker::Worker;
use crate::storage::event_store::EventStore;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};

const NUM_WORKERS: usize = 32;

pub struct EventPipeline {
    storage: Arc<dyn EventStore + Send + Sync>,
    workers: Vec<mpsc::Sender<PipelineCommand>>,
    topology: ClusterTopology,
    cluster_client: ClusterClient,
    self_addr: String,
}

impl EventPipeline {
    pub fn new(
        storage: Arc<dyn EventStore + Send + Sync>,
        cluster_nodes: Vec<String>,
        self_node_id: u64,
        auth_token: Option<String>,
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

        // Initialize Topology with Epoch 0 (MVP Static)
        let topology = ClusterTopology::new(cluster_nodes.clone(), 0);
        
        // Determine self address based on ID, safe fallback if config is weird
        let sorted_nodes = topology.get_all_nodes();
        let self_addr = if (self_node_id as usize) < sorted_nodes.len() {
            sorted_nodes[self_node_id as usize].clone()
        } else {
            // Fallback for single node dev mode if config mismatch, assuming first one
            sorted_nodes
                .first()
                .cloned()
                .unwrap_or_else(|| "127.0.0.1:50051".to_string())
        };

        let cluster_client = ClusterClient::new(auth_token);

        Self {
            storage,
            workers,
            topology,
            cluster_client,
            self_addr,
        }
    }

    #[tracing::instrument(skip(self, events), fields(stream_id = %stream_id, event_count = events.len()))]
    /// Entry point for clients: Routes request to owner (Local or Remote)
    pub async fn append_event(
        &self,
        stream_id: &str,
        events: Vec<Event>,
        expected_version: i64,
    ) -> Result<bool, String> {
        // 1. Determine Owner
        let owner = self.topology.get_owner(stream_id);
        
        // 2. Route
        if owner.node_addr == self.self_addr {
            // Local processing (we are owner)
            self.append_event_as_owner(stream_id, events, expected_version).await
        } else {
            // Remote Forwarding
            self.cluster_client
                .forward_append(&owner.node_addr, stream_id, events, expected_version)
                .await
        }
    }

    /// Strict Entry point: Only processes if WE are the owner. 
    /// Used for forwarded requests or strict validation.
    pub async fn append_event_as_owner(
        &self,
        stream_id: &str,
        events: Vec<Event>,
        expected_version: i64,
    ) -> Result<bool, String> {
        // 1. Validate Ownership Again (Safety)
        let owner = self.topology.get_owner(stream_id);
        if owner.node_addr != self.self_addr {
            return Err(format!(
                "NotOwnerError: Node {} received write for stream {} but owner is {} (Epoch {})",
                self.self_addr, stream_id, owner.node_addr, owner.epoch
            ));
        }

        // 2. Local Processing via Sharded Workers
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

        self.workers[worker_idx]
            .send(cmd)
            .await
            .map_err(|e| e.to_string())?;

        resp_rx.await.map_err(|e| e.to_string())?
    }

    pub async fn fetch_stream(&self, stream_id: &str) -> Result<Vec<Event>, String> {
        self.storage
            .fetch_stream(stream_id)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn upsert_schema(
        &self,
        schema: crate::domain::schema::model::Schema,
    ) -> Result<(), String> {
        self.storage
            .upsert_schema(schema)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_schema(
        &self,
        name: &str,
    ) -> Result<Option<crate::domain::schema::model::Schema>, String> {
        self.storage
            .get_schema(name)
            .await
            .map_err(|e| e.to_string())
    }
}
