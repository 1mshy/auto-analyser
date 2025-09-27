-- Initial schema for stock analysis results
CREATE TABLE IF NOT EXISTS analysis_results (
    id TEXT PRIMARY KEY,
    ticker TEXT NOT NULL,
    name TEXT NOT NULL,
    current_price REAL,
    rsi REAL,
    sma_20 REAL,
    sma_50 REAL,
    macd REAL,
    macd_signal REAL,
    macd_histogram REAL,
    volume INTEGER,
    pct_change REAL,
    market_cap TEXT,
    is_opportunity INTEGER NOT NULL,
    signals TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    analysis_session TEXT NOT NULL,
    UNIQUE(ticker, analysis_session)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_ticker ON analysis_results(ticker);
CREATE INDEX IF NOT EXISTS idx_timestamp ON analysis_results(timestamp);
CREATE INDEX IF NOT EXISTS idx_session ON analysis_results(analysis_session);
CREATE INDEX IF NOT EXISTS idx_opportunity ON analysis_results(is_opportunity);
CREATE INDEX IF NOT EXISTS idx_rsi ON analysis_results(rsi);