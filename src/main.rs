use graveyar_db::{
    config,
    infra::operational::{emitter::OperationalEmitter, event::OperationalEvent},
};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::settings::Config::from_env()?;

    let (tx, mut rx) = mpsc::channel::<OperationalEvent>(1024);
    let _emitter = OperationalEmitter::new(tx.clone()); // TODO passon emitter to followed components.

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            // TODO Persist in a table.
            // TODO Increment metrics.
            // TODO Sent to OTEL or other collector.
            println!("Operational event: {:?}", event);
        }
    });

    println!("Loaded config: {:?}", config);
    Ok(())
}
