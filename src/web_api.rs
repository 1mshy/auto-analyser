use axum::{
    extract::{Query, State, WebSocketUpgrade},
    extract::ws::{Message, WebSocket},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::{broadcast, RwLock};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;
use futures::{sink::SinkExt, stream::StreamExt};

use crate::{StockAnalyzer, StockFilter, TickerInfo};
use crate::cache::CacheManager;
use crate::database::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRequest {
    pub filter: StockFilter,
    pub max_tickers: Option<usize>,
    pub max_analysis: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockAnalysisResult {
    pub ticker: String,
    pub name: String,
    pub current_price: Option<f64>,
    pub rsi: Option<f64>,
    pub sma_20: Option<f64>,
    pub sma_50: Option<f64>,
    pub macd: Option<f64>,
    pub macd_signal: Option<f64>,
    pub macd_histogram: Option<f64>,
    pub volume: Option<u64>,
    pub pct_change: Option<f64>,
    pub market_cap: Option<String>,
    pub is_opportunity: bool,
    pub signals: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisStatus {
    pub session_id: String,
    pub status: String, // "running", "completed", "error"
    pub progress: f64,  // 0.0 to 1.0
    pub analyzed_count: usize,
    pub total_count: usize,
    pub opportunities_found: usize,
    pub error_message: Option<String>,
    pub results: Vec<StockAnalysisResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterStats {
    pub total_tickers: usize,
    pub filtered_tickers: usize,
    pub sectors: HashMap<String, usize>,
    pub countries: HashMap<String, usize>,
    pub price_ranges: HashMap<String, usize>,
}

#[derive(Clone)]
pub struct AppState {
    pub sessions: Arc<RwLock<HashMap<String, AnalysisStatus>>>,
    pub broadcast_tx: broadcast::Sender<AnalysisStatus>,
    pub all_results: Arc<RwLock<Vec<StockAnalysisResult>>>,
    pub continuous_analysis_status: Arc<RwLock<ContinuousAnalysisStatus>>,
    pub cache: CacheManager,
    pub database: Option<Arc<Database>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuousAnalysisStatus {
    pub is_running: bool,
    pub current_cycle: usize,
    pub progress: f64,
    pub analyzed_count: usize,
    pub total_count: usize,
    pub opportunities_found: usize,
    pub last_update: chrono::DateTime<chrono::Utc>,
    pub error_message: Option<String>,
}

impl Default for ContinuousAnalysisStatus {
    fn default() -> Self {
        Self {
            is_running: false,
            current_cycle: 0,
            progress: 0.0,
            analyzed_count: 0,
            total_count: 0,
            opportunities_found: 0,
            last_update: chrono::Utc::now(),
            error_message: None,
        }
    }
}

impl AppState {
    pub async fn new() -> Self {
        let (broadcast_tx, _) = broadcast::channel(100);
        let cache = CacheManager::new();
        
        // Try to initialize database, but continue without it if it fails
        let database = match Database::new("sqlite:analysis.db").await {
            Ok(db) => {
                if let Err(e) = db.initialize_tables().await {
                    tracing::warn!("Failed to initialize database tables: {}", e);
                    None
                } else {
                    tracing::info!("Database initialized successfully");
                    Some(Arc::new(db))
                }
            }
            Err(e) => {
                tracing::warn!("Failed to connect to database: {}", e);
                None
            }
        };
        
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            broadcast_tx,
            all_results: Arc::new(RwLock::new(Vec::new())),
            continuous_analysis_status: Arc::new(RwLock::new(ContinuousAnalysisStatus::default())),
            cache,
            database,
        }
    }
    
    pub async fn start_continuous_analysis(&self) {
        let state = self.clone();
        tokio::spawn(async move {
            run_continuous_analysis(state).await;
        });
    }
}

pub async fn create_router() -> Router {
    let state = AppState::new().await;
    
    // Start continuous analysis
    state.start_continuous_analysis().await;

    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/tickers", get(get_tickers))
        .route("/api/filter-stats", post(get_filter_stats))
        .route("/api/analysis", post(start_analysis))
        .route("/api/analysis/:session_id", get(get_analysis_status))
        .route("/api/analysis/:session_id/results", get(get_analysis_results))
        .route("/api/continuous-status", get(get_continuous_status))
        .route("/api/filtered-results", post(get_filtered_results))
        .route("/api/cache-stats", get(get_cache_stats))
        .route("/api/database-stats", get(get_database_stats))
        .route("/api/clear-cache", post(clear_cache))
        .route("/ws", get(websocket_handler))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}

async fn get_continuous_status(
    State(state): State<AppState>,
) -> Result<Json<ContinuousAnalysisStatus>, StatusCode> {
    let status = state.continuous_analysis_status.read().await;
    Ok(Json(status.clone()))
}

async fn get_filtered_results(
    State(state): State<AppState>,
    Json(filter): Json<StockFilter>,
) -> Result<Json<Vec<StockAnalysisResult>>, StatusCode> {
    // Try to get from database first if available
    if let Some(ref db) = state.database {
        match db.get_latest_results(None).await {
            Ok(db_results) => {
                let filtered_results = filter_results(&db_results, &filter);
                return Ok(Json(filtered_results));
            }
            Err(e) => {
                tracing::warn!("Failed to get results from database: {}", e);
            }
        }
    }
    
    // Fallback to in-memory results
    let all_results = state.all_results.read().await;
    let filtered_results = filter_results(&all_results, &filter);
    Ok(Json(filtered_results))
}

async fn get_cache_stats(
    State(state): State<AppState>,
) -> Result<Json<crate::cache::CacheStats>, StatusCode> {
    let stats = state.cache.get_cache_stats().await;
    Ok(Json(stats))
}

async fn get_database_stats(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if let Some(ref db) = state.database {
        match db.get_analysis_stats().await {
            Ok(stats) => Ok(Json(serde_json::to_value(stats).unwrap())),
            Err(e) => {
                tracing::error!("Failed to get database stats: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        Ok(Json(serde_json::json!({
            "error": "Database not available"
        })))
    }
}

async fn clear_cache(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    state.cache.clear_cache().await;
    tracing::info!("Cache cleared via API request");
    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "Cache cleared successfully"
    })))
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> axum::response::Response {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

async fn handle_websocket(socket: WebSocket, state: AppState) {
    tracing::info!("🔌 New WebSocket connection established");
    let (mut sender, mut receiver) = socket.split();
    let mut broadcast_rx = state.broadcast_tx.subscribe();
    
    // Send current status immediately
    let status = state.continuous_analysis_status.read().await.clone();
    let msg = serde_json::to_string(&status).unwrap_or_default();
    if let Err(e) = sender.send(Message::Text(msg)).await {
        tracing::warn!("Failed to send initial status: {}", e);
        return;
    }
    
    tracing::info!("📡 WebSocket ready to receive broadcasts");
    
    // Handle incoming messages and broadcast updates
    loop {
        tokio::select! {
            // Handle incoming messages from client (mostly pings/pongs)
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        tracing::debug!("Received WebSocket message: {}", text);
                        // Echo back or handle client messages if needed
                    },
                    Some(Ok(Message::Binary(_))) => {
                        tracing::debug!("Received binary message (ignored)");
                    },
                    Some(Ok(Message::Ping(data))) => {
                        if let Err(e) = sender.send(Message::Pong(data)).await {
                            tracing::warn!("Failed to send pong: {}", e);
                            break;
                        }
                    },
                    Some(Ok(Message::Pong(_))) => {
                        tracing::debug!("Received pong");
                    },
                    Some(Ok(Message::Close(_))) => {
                        tracing::info!("🔌 WebSocket client initiated close");
                        break;
                    },
                    Some(Err(e)) => {
                        tracing::warn!("WebSocket receive error: {}", e);
                        break;
                    },
                    None => {
                        tracing::info!("🔌 WebSocket receiver stream ended");
                        break;
                    }
                }
            },
            // Handle broadcast updates from server
            status = broadcast_rx.recv() => {
                match status {
                    Ok(status_update) => {
                        let msg = serde_json::to_string(&status_update).unwrap_or_default();
                        if let Err(e) = sender.send(Message::Text(msg)).await {
                            tracing::warn!("Failed to send broadcast: {}", e);
                            break;
                        }
                    },
                    Err(e) => {
                        tracing::debug!("Broadcast channel error (normal on shutdown): {}", e);
                        break;
                    }
                }
            }
        }
    }
    
    tracing::info!("🔌 WebSocket connection closed");
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

#[derive(Deserialize)]
struct TickerQuery {
    limit: Option<usize>,
}

async fn get_tickers(State(state): State<AppState>, Query(params): Query<TickerQuery>) -> Result<Json<Vec<TickerInfo>>, StatusCode> {
    let _limit = params.limit.unwrap_or(0); // 0 means fetch all - but we'll fetch all anyway
    
    let analyzer = StockAnalyzer::new_with_cache(state.cache.clone());
    match analyzer.fetch_all_tickers_cached().await {
        Ok(tickers) => Ok(Json(tickers)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_filter_stats(
    State(state): State<AppState>,
    Json(filter): Json<StockFilter>,
) -> Result<Json<FilterStats>, StatusCode> {
    let analyzer = StockAnalyzer::new_with_cache(state.cache.clone());
    match analyzer.fetch_all_tickers_cached().await {
        Ok(all_tickers) => {
            let filtered_tickers = StockAnalyzer::filter_tickers(&all_tickers, &filter);
            
            let mut sectors = HashMap::new();
            let mut countries = HashMap::new();
            let mut price_ranges = HashMap::new();
            
            for ticker in &filtered_tickers {
                if let Some(sector) = &ticker.sector {
                    *sectors.entry(sector.clone()).or_insert(0) += 1;
                }
                if let Some(country) = &ticker.country {
                    *countries.entry(country.clone()).or_insert(0) += 1;
                }
                
                if let Some(price_str) = &ticker.last_sale {
                    if let Ok(price) = price_str.replace('$', "").parse::<f64>() {
                        let range = match price {
                            p if p < 10.0 => "Under $10",
                            p if p < 50.0 => "$10-$50",
                            p if p < 100.0 => "$50-$100",
                            p if p < 500.0 => "$100-$500",
                            _ => "Over $500",
                        };
                        *price_ranges.entry(range.to_string()).or_insert(0) += 1;
                    }
                }
            }
            
            Ok(Json(FilterStats {
                total_tickers: all_tickers.len(),
                filtered_tickers: filtered_tickers.len(),
                sectors,
                countries,
                price_ranges,
            }))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn start_analysis(
    State(state): State<AppState>,
    Json(request): Json<AnalysisRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let session_id = Uuid::new_v4().to_string();
    
    let initial_status = AnalysisStatus {
        session_id: session_id.clone(),
        status: "running".to_string(),
        progress: 0.0,
        analyzed_count: 0,
        total_count: 0,
        opportunities_found: 0,
        error_message: None,
        results: Vec::new(),
    };
    
    // Store initial status
    state.sessions.write().await.insert(session_id.clone(), initial_status.clone());
    
    // Spawn background task for analysis
    let state_clone = state.clone();
    let session_id_clone = session_id.clone();
    tokio::spawn(async move {
        run_analysis(state_clone, session_id_clone, request).await;
    });
    
    Ok(Json(serde_json::json!({
        "session_id": session_id,
        "status": "started"
    })))
}

async fn get_analysis_status(
    State(state): State<AppState>,
    axum::extract::Path(session_id): axum::extract::Path<String>,
) -> Result<Json<AnalysisStatus>, StatusCode> {
    let sessions = state.sessions.read().await;
    match sessions.get(&session_id) {
        Some(status) => Ok(Json(status.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_analysis_results(
    State(state): State<AppState>,
    axum::extract::Path(session_id): axum::extract::Path<String>,
) -> Result<Json<Vec<StockAnalysisResult>>, StatusCode> {
    let sessions = state.sessions.read().await;
    match sessions.get(&session_id) {
        Some(status) => Ok(Json(status.results.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn run_analysis(state: AppState, session_id: String, request: AnalysisRequest) {
    let mut analyzer = StockAnalyzer::new_with_cache(state.cache.clone());
    
    // Update status to show we're starting
    let mut current_status = {
        let sessions = state.sessions.read().await;
        sessions.get(&session_id).unwrap().clone()
    };
    
    // Fetch tickers with caching
    let all_tickers = match analyzer.fetch_all_tickers_cached().await {
        Ok(tickers) => tickers,
        Err(e) => {
            current_status.status = "error".to_string();
            current_status.error_message = Some(format!("Failed to fetch tickers: {}", e));
            state.sessions.write().await.insert(session_id, current_status.clone());
            let _ = state.broadcast_tx.send(current_status);
            return;
        }
    };
    
    // Apply filters
    let filtered_tickers = StockAnalyzer::filter_tickers(&all_tickers, &request.filter);
    let max_analysis = request.max_analysis.unwrap_or(filtered_tickers.len()).min(filtered_tickers.len());
    
    current_status.total_count = max_analysis;
    state.sessions.write().await.insert(session_id.clone(), current_status.clone());
    let _ = state.broadcast_tx.send(current_status.clone());
    
    // Analyze each ticker
    for (i, ticker_info) in filtered_tickers.iter().take(max_analysis).enumerate() {
        let ticker = &ticker_info.symbol;
        
        match analyzer.fetch_stock_data_cached(ticker).await {
            Ok(stock_data) => {
                if !stock_data.is_empty() {
                    let indicators = analyzer.calculate_indicators_cached(ticker, &stock_data).await;
                    
                    if let Some(latest_indicator) = indicators.last() {
                        let current_price = stock_data.last().map(|quote| quote.close);
                        let is_opportunity = latest_indicator.rsi.map_or(false, |rsi| {
                            rsi <= request.filter.oversold_rsi_threshold.unwrap_or(30.0) ||
                            rsi >= request.filter.overbought_rsi_threshold.unwrap_or(70.0)
                        });
                        
                        let mut signals = Vec::new();
                        if let Some(rsi) = latest_indicator.rsi {
                            if rsi <= 30.0 {
                                signals.push("Oversold - Potential Buy".to_string());
                            } else if rsi >= 70.0 {
                                signals.push("Overbought - Potential Sell".to_string());
                            }
                        }
                        
                        let (macd_value, macd_signal_value, macd_histogram_value) = 
                            latest_indicator.macd.unwrap_or((0.0, 0.0, 0.0));
                        
                        let result = StockAnalysisResult {
                            ticker: ticker.clone(),
                            name: ticker_info.name.clone(),
                            current_price,
                            rsi: latest_indicator.rsi,
                            sma_20: latest_indicator.sma_20,
                            sma_50: latest_indicator.sma_50,
                            macd: if latest_indicator.macd.is_some() { Some(macd_value) } else { None },
                            macd_signal: if latest_indicator.macd.is_some() { Some(macd_signal_value) } else { None },
                            macd_histogram: if latest_indicator.macd.is_some() { Some(macd_histogram_value) } else { None },
                            volume: stock_data.last().map(|q| q.volume),
                            pct_change: ticker_info.pct_change.as_ref().and_then(|s| {
                                s.replace('%', "").parse().ok()
                            }),
                            market_cap: ticker_info.market_cap.clone(),
                            is_opportunity,
                            signals,
                            timestamp: chrono::Utc::now(),
                        };
                        
                        current_status.results.push(result.clone());
                        if is_opportunity {
                            current_status.opportunities_found += 1;
                        }

                        // Store in database if available
                        if let Some(ref db) = state.database {
                            if let Err(e) = db.store_analysis_result(&result, &session_id).await {
                                tracing::warn!("Failed to store result in database: {}", e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Failed to analyze {}: {}", ticker, e);
            }
        }
        
        current_status.analyzed_count = i + 1;
        current_status.progress = (i + 1) as f64 / max_analysis as f64;
        
        // Update status every 5 stocks or on the last one
        if (i + 1) % 5 == 0 || i + 1 == max_analysis {
            state.sessions.write().await.insert(session_id.clone(), current_status.clone());
            let _ = state.broadcast_tx.send(current_status.clone());
        }
        
        // Remove delay to process faster
        // tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    current_status.status = "completed".to_string();
    current_status.progress = 1.0;
    state.sessions.write().await.insert(session_id.clone(), current_status.clone());
    let _ = state.broadcast_tx.send(current_status);
}

fn filter_results(results: &[StockAnalysisResult], filter: &StockFilter) -> Vec<StockAnalysisResult> {
    results.iter()
        .filter(|result| {
            // Apply RSI filter
            if let Some(min_rsi) = filter.min_rsi {
                if result.rsi.map_or(true, |rsi| rsi < min_rsi) {
                    return false;
                }
            }
            if let Some(max_rsi) = filter.max_rsi {
                if result.rsi.map_or(true, |rsi| rsi > max_rsi) {
                    return false;
                }
            }
            
            // Apply price filter
            if let Some(min_price) = filter.min_price {
                if result.current_price.map_or(true, |price| price < min_price) {
                    return false;
                }
            }
            if let Some(max_price) = filter.max_price {
                if result.current_price.map_or(true, |price| price > max_price) {
                    return false;
                }
            }
            
            // Apply volume filter
            if let Some(min_volume) = filter.min_volume {
                if result.volume.map_or(true, |vol| vol < min_volume) {
                    return false;
                }
            }
            if let Some(max_volume) = filter.max_volume {
                if result.volume.map_or(true, |vol| vol > max_volume) {
                    return false;
                }
            }
            
            // Apply percentage change filter
            if let Some(min_pct_change) = filter.min_pct_change {
                if result.pct_change.map_or(true, |pct| pct < min_pct_change) {
                    return false;
                }
            }
            if let Some(max_pct_change) = filter.max_pct_change {
                if result.pct_change.map_or(true, |pct| pct > max_pct_change) {
                    return false;
                }
            }
            
            true
        })
        .cloned()
        .collect()
}

async fn run_continuous_analysis(state: AppState) {
    tracing::info!("🔄 Starting continuous stock analysis...");
    
    let mut cycle = 0;
    loop {
        cycle += 1;
        
        // Update status to running
        {
            let mut status = state.continuous_analysis_status.write().await;
            status.is_running = true;
            status.current_cycle = cycle;
            status.progress = 0.0;
            status.analyzed_count = 0;
            status.last_update = chrono::Utc::now();
            status.error_message = None;
        }
        
        let mut analyzer = StockAnalyzer::new_with_cache(state.cache.clone());
        
        // Fetch all tickers with caching
        let all_tickers = match analyzer.fetch_all_tickers_cached().await {
            Ok(tickers) => tickers,
            Err(e) => {
                let mut status = state.continuous_analysis_status.write().await;
                status.error_message = Some(format!("Failed to fetch tickers: {}", e));
                status.is_running = false;
                tracing::error!("❌ Failed to fetch tickers: {}", e);
                
                // Wait 5 minutes before retrying
                tokio::time::sleep(Duration::from_secs(300)).await;
                continue;
            }
        };
        
        {
            let mut status = state.continuous_analysis_status.write().await;
            status.total_count = all_tickers.len();
        }
        
        let mut new_results = Vec::new();
        let mut opportunities_found = 0;
        
        // Analyze each ticker and update results immediately
        let session_id = format!("continuous_cycle_{}", cycle);
        
        for (i, ticker_info) in all_tickers.iter().enumerate() {
            let ticker = &ticker_info.symbol;
            
            match analyzer.fetch_stock_data_cached(ticker).await {
                Ok(stock_data) => {
                    if !stock_data.is_empty() {
                        let indicators = analyzer.calculate_indicators_cached(ticker, &stock_data).await;
                        
                        if let Some(latest_indicator) = indicators.last() {
                            let current_price = stock_data.last().map(|quote| quote.close);
                            let is_opportunity = latest_indicator.rsi.map_or(false, |rsi| {
                                rsi <= 30.0 || rsi >= 70.0
                            });
                            
                            let mut signals = Vec::new();
                            if let Some(rsi) = latest_indicator.rsi {
                                if rsi <= 30.0 {
                                    signals.push("Oversold - Potential Buy".to_string());
                                } else if rsi >= 70.0 {
                                    signals.push("Overbought - Potential Sell".to_string());
                                }
                            }
                            
                            let (macd_value, macd_signal_value, macd_histogram_value) = 
                                latest_indicator.macd.unwrap_or((0.0, 0.0, 0.0));
                            
                            let result = StockAnalysisResult {
                                ticker: ticker.clone(),
                                name: ticker_info.name.clone(),
                                current_price,
                                rsi: latest_indicator.rsi,
                                sma_20: latest_indicator.sma_20,
                                sma_50: latest_indicator.sma_50,
                                macd: if latest_indicator.macd.is_some() { Some(macd_value) } else { None },
                                macd_signal: if latest_indicator.macd.is_some() { Some(macd_signal_value) } else { None },
                                macd_histogram: if latest_indicator.macd.is_some() { Some(macd_histogram_value) } else { None },
                                volume: stock_data.last().map(|q| q.volume),
                                pct_change: ticker_info.pct_change.as_ref().and_then(|s| {
                                    s.replace('%', "").parse().ok()
                                }),
                                market_cap: ticker_info.market_cap.clone(),
                                is_opportunity,
                                signals,
                                timestamp: chrono::Utc::now(),
                            };
                            
                            // Add to local results
                            new_results.push(result.clone());
                            if is_opportunity {
                                opportunities_found += 1;
                            }
                            
                            // Store in database if available
                            if let Some(ref db) = state.database {
                                if let Err(e) = db.store_analysis_result(&result, &session_id).await {
                                    tracing::warn!("Failed to store result in database: {}", e);
                                }
                            }
                            
                            // Immediately update global results with this stock
                            {
                                let mut all_results = state.all_results.write().await;
                                // Remove any existing result for this ticker
                                all_results.retain(|r| r.ticker != *ticker);
                                // Add the new result
                                all_results.push(result);
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to analyze {}: {}", ticker, e);
                }
            }
            
            // Update progress every 5 stocks for more frequent updates
            if (i + 1) % 5 == 0 || i + 1 == all_tickers.len() {
                let mut status = state.continuous_analysis_status.write().await;
                status.analyzed_count = i + 1;
                status.progress = (i + 1) as f64 / all_tickers.len() as f64;
                status.opportunities_found = opportunities_found;
                status.last_update = chrono::Utc::now();
                
                // Broadcast update every 10 stocks for more frequent updates
                if (i + 1) % 10 == 0 || i + 1 == all_tickers.len() {
                    let _ = state.broadcast_tx.send(AnalysisStatus {
                        session_id: "continuous".to_string(),
                        status: "running".to_string(),
                        progress: status.progress,
                        analyzed_count: status.analyzed_count,
                        total_count: status.total_count,
                        opportunities_found: status.opportunities_found,
                        error_message: None,
                        results: new_results.clone(),
                    });
                }
            }
            
            // Small delay to avoid overwhelming the API
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        
        // Mark cycle as complete
        {
            let mut status = state.continuous_analysis_status.write().await;
            status.is_running = false;
            status.progress = 1.0;
            status.last_update = chrono::Utc::now();
            
            tracing::info!("✅ Completed analysis cycle {} - {} opportunities found", cycle, opportunities_found);
        }
        
        // Wait 1 hour before next cycle
        tracing::info!("⏱️  Waiting 1 hour before next analysis cycle...");
        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let app = create_router().await;
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;
    println!("🚀 API Server running on http://127.0.0.1:3001");
    println!("📊 Dashboard available at http://127.0.0.1:3000");
    
    axum::serve(listener, app).await?;
    Ok(())
}
