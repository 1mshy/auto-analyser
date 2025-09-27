use anyhow::Result;
use auto_analyser::web_api;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,auto_analyser=debug".into())
        )
        .init();

    tracing::info!("🚀 Starting Auto Stock Analyser Web API...");
    
    if let Err(e) = web_api::start_server().await {
        tracing::error!("❌ Server error: {}", e);
        std::process::exit(1);
    }
    
    Ok(())
}
