create index
    if not exists stock_prices_ticker_idx on stock_prices (ticker);

create index
    if not exists stock_prices_date_idx on stock_prices ("date");