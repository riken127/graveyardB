use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum ScyllaError {
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Query error: {0}")]
    QueryError(String),
}

pub struct ScyllaStore {
    session: Session,
    keyspace: String,
}

impl ScyllaStore {
    pub async fn new(uri: &str, keyspace: &str) -> Result<Self, ScyllaError> {
        let session = SessionBuilder::new()
            .known_node(uri)
            .connection_timeout(Duration::from_secs(5))
            .build()
            .await
            .map_err(|e| ScyllaError::ConnectionError(e.to_string()))?;

        let store = Self {
            session,
            keyspace: keyspace.to_string(),
        };

        store.init_schema().await?;

        Ok(store)
    }

    async fn init_schema(&self) -> Result<(), ScyllaError> {
        // Create keyspace
        let create_keyspace = format!(
            "CREATE KEYSPACE IF NOT EXISTS {} \
             WITH replication = {{'class': 'SimpleStrategy', 'replication_factor': 1}}",
            self.keyspace
        );
        self.session
            .query_unpaged(create_keyspace, &[])
            .await
            .map_err(|e| ScyllaError::QueryError(e.to_string()))?;

        // Create table
        // stream_id is partition key, id (uuid v7) is clustering key for time ordering
        let create_table = format!(
            "CREATE TABLE IF NOT EXISTS {}.events ( \
             stream_id text, \
             id uuid, \
             event_type text, \
             payload blob, \
             timestamp bigint, \
             PRIMARY KEY (stream_id, id))",
            self.keyspace
        );
        self.session
            .query_unpaged(create_table, &[])
            .await
            .map_err(|e| ScyllaError::QueryError(e.to_string()))?;

        Ok(())
    }
    
    pub fn get_session(&self) -> &Session {
        &self.session
    }
}
