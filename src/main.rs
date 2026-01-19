use graveyar_db::{
    api::event_store_server::EventStoreServer,
    config,
    grpc::GrpcService,
    pipeline::EventPipeline,
    storage::{
        event_store::EventStore, hybrid::HybridEventStore, rocksdb::event_store::RocksEventStore,
        scylla::session::ScyllaStore,
    },
};
use std::sync::Arc;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = config::settings::Config::from_env()?;

    println!("bootstrap OK");
    println!("Loaded config: {:?}", config);

    // 1. Storage Initialization
    let rocks_store = Arc::new(RocksEventStore::new(&config.db_path)?);

    let storage: Arc<dyn EventStore> = if let Some(scylla_uri) = &config.scylla_uri {
        println!("Initializing ScyllaDB at {}...", scylla_uri);
        match ScyllaStore::new(scylla_uri, &config.scylla_keyspace).await {
            Ok(scylla) => {
                println!("ScyllaDB connected. Using Hybrid Storage (Primary: Scylla, Fallback: RocksDB).");
                Arc::new(HybridEventStore::new(Arc::new(scylla), rocks_store))
            }
            Err(e) => {
                eprintln!(
                    "Failed to connect to ScyllaDB: {}. Falling back to RocksDB only.",
                    e
                );
                rocks_store
            }
        }
    } else {
        println!("No SCYLLA_URI configured. Using RocksDB only.");
        rocks_store
    };

    // 2. Pipeline
    let pipeline = Arc::new(EventPipeline::new(
        storage,
        config.cluster_nodes.clone(),
        config.node_id,
    ));

    // 3. gRPC Service
    let service = GrpcService::new(pipeline);
    let addr = format!("0.0.0.0:{}", config.port).parse()?;

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(EventStoreServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
