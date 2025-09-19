use anyhow::Result;
use auto_analyser::web_api;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Starting Auto Stock Analyser Web API...");
    
    if let Err(e) = web_api::start_server().await {
        eprintln!("❌ Server error: {}", e);
        std::process::exit(1);
    }
    
    Ok(())
}
