ALTER TABLE stock_prices
ADD
    COLUMN IF NOT EXISTS "close_normalized" REAL;

ALTER TABLE stock_prices
ADD
    COLUMN IF NOT EXISTS "open_normalized" REAL;

ALTER TABLE stock_prices
ADD
    COLUMN IF NOT EXISTS "high_normalized" REAL;

ALTER TABLE stock_prices
ADD
    COLUMN IF NOT EXISTS "low_normalized" REAL;