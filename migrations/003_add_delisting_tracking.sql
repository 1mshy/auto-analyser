-- Add additional fields to track delisted stocks and error messages
ALTER TABLE stocks ADD COLUMN IF NOT EXISTS delisting_reason TEXT;
ALTER TABLE stocks ADD COLUMN IF NOT EXISTS last_error_at TIMESTAMPTZ;
ALTER TABLE stocks ADD COLUMN IF NOT EXISTS last_error_message TEXT;

-- Create index for delisted stocks
CREATE INDEX IF NOT EXISTS idx_stocks_delisting_reason ON stocks(delisting_reason);
CREATE INDEX IF NOT EXISTS idx_stocks_last_error_at ON stocks(last_error_at);

-- Update existing inactive stocks to have a delisting reason
UPDATE stocks 
SET delisting_reason = 'unknown', updated_at = NOW() 
WHERE is_active = false AND delisting_reason IS NULL;
