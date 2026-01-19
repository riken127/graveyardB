use std::{env, time::Duration};

#[derive(Debug, Clone)]
pub struct Config {
    pub scylla_uri: Option<String>,
    pub scylla_keyspace: String,
    pub request_timeout: Duration,
    pub node_id: u64,
    pub cluster_nodes: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let scylla_uri = env::var("SCYLLA_URI").ok();

        let scylla_keyspace =
            env::var("SCYLLA_KEYSPACE").map_err(|_| "SCYLLA_KEYSPACE is undefined")?;

        let request_timeout = env::var("REQUEST_TIMEOUT_MS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_secs(3));

        let node_id = env::var("NODE_ID")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);

        let cluster_nodes = env::var("CLUSTER_NODES")
            .ok()
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_else(|| vec!["127.0.0.1:50051".to_string()]); // Default single node

        Ok(Self {
            scylla_uri,
            scylla_keyspace,
            request_timeout,
            node_id,
            cluster_nodes,
        })
    }
}
