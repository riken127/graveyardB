use graveyar_db::{
    api::event_store_server::EventStoreServer,
    config,
    grpc::GrpcService,
    pipeline::EventPipeline,
    storage::{
        event_store::EventStore,
        hybrid::HybridEventStore,
        rocksdb::event_store::RocksEventStore,
        scylla::session::ScyllaStore,
    },
};
use std::sync::Arc;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Telemetry setup paused due to crate version churn. 
    // TODO: Re-enable when opentelemetry-otlp 0.27/0.31 stabilizes or with clean deps.
    /*
    // Configuration for OpenTelemetry (0.27 compatible)
    use opentelemetry::trace::TracerProvider as _;
    use opentelemetry_sdk::trace::SdkTracerProvider;
    use opentelemetry_sdk::Resource;
    use opentelemetry::KeyValue;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    let resource = Resource::new(vec![
        KeyValue::new("service.name", "graveyar_db"),
    ]);

    let exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint("http://localhost:4318/v1/traces");

    let tracer_provider = SdkTracerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter.into())
        .build();
    
    let tracer = tracer_provider.tracer("graveyar_db");

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(telemetry)
        .init();
    */
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
            },
            Err(e) => {
                eprintln!("Failed to connect to ScyllaDB: {}. Falling back to RocksDB only.", e);
                rocks_store
            }
        }
    } else {
        println!("No SCYLLA_URI configured. Using RocksDB only.");
        rocks_store
    };

    // 2. Pipeline
    let pipeline = Arc::new(EventPipeline::new(storage, config.cluster_nodes.clone(), config.node_id));

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
