-- Add priority system for stocks
CREATE TYPE stock_priority AS ENUM ('high', 'medium', 'low');

-- Add priority column to stocks table
ALTER TABLE stocks 
ADD COLUMN priority stock_priority DEFAULT 'low',
ADD COLUMN last_price_update TIMESTAMP WITH TIME ZONE,
ADD COLUMN price_update_interval_seconds INTEGER DEFAULT 900;

-- Create index for priority queries
CREATE INDEX idx_stocks_priority ON stocks(priority, is_active);
CREATE INDEX idx_stocks_last_price_update ON stocks(last_price_update);

-- Update existing stocks to have appropriate priorities
-- Stocks with recent market data activity get medium priority
UPDATE stocks 
SET priority = 'medium' 
WHERE symbol IN (
    SELECT DISTINCT symbol 
    FROM market_data 
    WHERE timestamp > NOW() - INTERVAL '7 days'
);

-- Create a function to automatically set high priority for watchlist stocks
CREATE OR REPLACE FUNCTION update_stock_priority_for_watchlist()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        -- Set high priority when stock is added to any watchlist
        UPDATE stocks 
        SET priority = 'high', 
            price_update_interval_seconds = 60
        WHERE symbol = NEW.symbol;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        -- Check if stock is still in any watchlist, if not reduce priority
        IF NOT EXISTS (SELECT 1 FROM watchlist WHERE symbol = OLD.symbol) THEN
            UPDATE stocks 
            SET priority = 'medium',
                price_update_interval_seconds = 300
            WHERE symbol = OLD.symbol;
        END IF;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create triggers to automatically manage priorities based on watchlist changes
CREATE TRIGGER trigger_watchlist_priority_insert
    AFTER INSERT ON watchlist
    FOR EACH ROW
    EXECUTE FUNCTION update_stock_priority_for_watchlist();

CREATE TRIGGER trigger_watchlist_priority_delete
    AFTER DELETE ON watchlist
    FOR EACH ROW
    EXECUTE FUNCTION update_stock_priority_for_watchlist();
