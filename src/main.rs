use graveyar_db::config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = config::settings::Config::from_env()?;

    // Log "bootstrap OK" as requested
    println!("bootstrap OK");
    println!("Loaded config: {:?}", config);

    Ok(())
}
