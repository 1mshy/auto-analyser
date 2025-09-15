-- Create stocks table to store all known US stocks
CREATE TABLE stocks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    symbol VARCHAR(10) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    exchange VARCHAR(50) NOT NULL,
    sector VARCHAR(100),
    industry VARCHAR(100),
    market_cap BIGINT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_stocks_symbol ON stocks(symbol);
CREATE INDEX idx_stocks_exchange ON stocks(exchange);
CREATE INDEX idx_stocks_sector ON stocks(sector);
CREATE INDEX idx_stocks_is_active ON stocks(is_active);

-- Add a last_updated column to track when we last fetched data for each stock
ALTER TABLE market_data ADD COLUMN IF NOT EXISTS last_updated TIMESTAMPTZ DEFAULT NOW();
CREATE INDEX idx_market_data_last_updated ON market_data(last_updated);
