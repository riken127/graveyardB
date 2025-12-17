use std::{env, time::Duration};


#[derive(Debug, Clone)]
pub struct Config {
    pub scylla_uri: String,
    pub scylla_keyspace: String,
    pub request_timeout: Duration
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let scylla_uri =
            env::var("SCYLLA_URI").map_err(|_| "SCYLLA_URI is undefined" )?;

        let scylla_keyspace =
            env::var("SCYLLA_KEYSPACE").map_err(|_| "SCYLLA_KEYSPACE is undefined")?;

        let request_timeout = env::var("REQUEST_TIMEOUT_MS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_secs(3));

        Ok(Self {
            scylla_uri,
            scylla_keyspace,
            request_timeout,
        })
    }
}