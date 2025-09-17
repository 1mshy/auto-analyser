use sqlx::{PgPool, Row};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "postgresql://postgres:password@localhost:5432/equity_analyser";
    
    println!("Attempting to connect to database...");
    
    // Try to connect with minimal configuration
    let pool = PgPool::connect(database_url).await?;
    
    println!("Connected successfully!");
    
    // Try a simple query
    let row = sqlx::query("SELECT 1 as test")
        .fetch_one(&pool)
        .await?;
    
    let value: i32 = row.get("test");
    println!("Query result: {}", value);
    
    Ok(())
}
