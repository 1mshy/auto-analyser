use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, RwLock};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use crate::{StockAnalyzer, StockFilter, TickerInfo};

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
}

impl AppState {
    pub fn new() -> Self {
        let (broadcast_tx, _) = broadcast::channel(100);
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            broadcast_tx,
        }
    }
}

pub async fn create_router() -> Router {
    let state = AppState::new();

    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/tickers", get(get_tickers))
        .route("/api/filter-stats", post(get_filter_stats))
        .route("/api/analysis", post(start_analysis))
        .route("/api/analysis/:session_id", get(get_analysis_status))
        .route("/api/analysis/:session_id/results", get(get_analysis_results))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
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

async fn get_tickers(Query(params): Query<TickerQuery>) -> Result<Json<Vec<TickerInfo>>, StatusCode> {
    let _limit = params.limit.unwrap_or(0); // 0 means fetch all - but we'll fetch all anyway
    
    match StockAnalyzer::fetch_all_tickers().await {
        Ok(tickers) => Ok(Json(tickers)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_filter_stats(
    Json(filter): Json<StockFilter>,
) -> Result<Json<FilterStats>, StatusCode> {
    match StockAnalyzer::fetch_all_tickers().await {
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
    let mut analyzer = StockAnalyzer::new();
    
    // Update status to show we're starting
    let mut current_status = {
        let sessions = state.sessions.read().await;
        sessions.get(&session_id).unwrap().clone()
    };
    
    // Fetch tickers
    let all_tickers = match StockAnalyzer::fetch_all_tickers().await {
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
        
        match analyzer.fetch_all_stock_data(ticker).await {
            Ok(stock_data) => {
                if !stock_data.is_empty() {
                    let indicators = analyzer.calculate_indicators(ticker, &stock_data);
                    
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
                        
                        current_status.results.push(result);
                        if is_opportunity {
                            current_status.opportunities_found += 1;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to analyze {}: {}", ticker, e);
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

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let app = create_router().await;
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;
    println!("🚀 API Server running on http://127.0.0.1:3001");
    println!("📊 Dashboard available at http://127.0.0.1:3000");
    
    axum::serve(listener, app).await?;
    Ok(())
}
