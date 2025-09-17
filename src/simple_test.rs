use sqlx::{PgPool, Row};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Testing database connection with different approaches...");
    
    // Try different connection approaches
    let connection_attempts = vec![
        "postgresql://postgres:password@localhost:5432/equity_analyser",
        "postgresql://postgres:password@localhost:5432/equity_analyser?application_name=test_app",
        "postgresql://postgres:password@localhost:5432/equity_analyser?options=--timezone%3DUTC",
        "postgresql://postgres:password@localhost:5432/equity_analyser?timezone=UTC",
    ];
    
    for (i, database_url) in connection_attempts.iter().enumerate() {
        println!("\nAttempt {}: Connecting to: {}", i + 1, database_url);
        
        match PgPool::connect(database_url).await {
            Ok(pool) => {
                println!("Connection successful!");
                
                // Try a simple query
                match sqlx::query("SELECT 1 as test").fetch_one(&pool).await {
                    Ok(result) => {
                        let value: i32 = result.get("test");
                        println!("Query result: {}", value);
                        println!("SUCCESS with connection string: {}", database_url);
                        return Ok(());
                    }
                    Err(e) => {
                        println!("Query failed: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
    
    println!("All connection attempts failed");
    Ok(())
}
