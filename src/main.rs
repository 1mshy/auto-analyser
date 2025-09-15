mod api;
mod config;
mod database;
mod handlers;
mod models;
mod services;
mod utils;

use anyhow::Result;
use axum::{
    middleware,
    routing::{get, post, put, delete},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::{info, Level};
use tracing_subscriber;

use crate::config::Config;
use crate::database::Database;
use crate::handlers::{auth, market_data, alerts, watchlist};
use crate::utils::auth::auth_middleware;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting Equity Analyser Service");

    // Load configuration
    let config = Config::from_env()?;
    
    // Initialize database
    let db = Database::new(&config.database_url).await?;
    db.migrate().await?;

    // Create shared application state
    let app_state = api::AppState::new(config.clone(), db);

    // Build the application router
    let protected_routes = Router::new()
        .route("/api/auth/me", get(auth::me))
        .route("/api/market/quote/:symbol", get(market_data::get_quote))
        .route("/api/market/historical/:symbol", get(market_data::get_historical))
        .route("/api/market/indicators/:symbol", get(market_data::get_indicators))
        .route("/api/watchlist", get(watchlist::get_watchlist))
        .route("/api/watchlist", post(watchlist::add_to_watchlist))
        .route("/api/watchlist/:symbol", delete(watchlist::remove_from_watchlist))
        .route("/api/alerts", get(alerts::get_alerts))
        .route("/api/alerts", post(alerts::create_alert))
        .route("/api/alerts/:id", put(alerts::update_alert))
        .route("/api/alerts/:id", delete(alerts::delete_alert))
        .layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
        // Public auth routes
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        
        // Protected routes
        .merge(protected_routes)
        
        // Health check (public)
        .route("/health", get(|| async { "OK" }))
        
        // Serve static files for the web UI
        .nest_service("/", ServeDir::new("static"))
        
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
        )
        .with_state(app_state);

    // Start the market data scheduler
    let scheduler_state = api::AppState::new(config.clone(), Database::new(&config.database_url).await?);
    tokio::spawn(async move {
        services::scheduler::start_market_data_scheduler(scheduler_state).await;
    });

    // Start the alert evaluator
    let alert_state = api::AppState::new(config.clone(), Database::new(&config.database_url).await?);
    tokio::spawn(async move {
        services::alerts::start_alert_evaluator(alert_state).await;
    });

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

