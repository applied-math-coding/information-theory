create index
    if not exists stock_prices_ticker_idx on stock_prices (ticker);

with value_ranges as (
        select
            max(sp."close") - min(sp."close") as close_range,
            min(sp."close") as min_close,
            max(sp."open") - min(sp."open") as open_range,
            min(sp."open") as min_open,
            max(sp."high") - min(sp."high") as high_range,
            min(sp."high") as min_high,
            max(sp."low") - min(sp."low") as low_range,
            min(sp."low") as min_low,
            sp.ticker as ticker
        from stock_prices sp
        group by sp.ticker
    )
update stock_prices as sp
set
    close_normalized = (sp."close" - vr.min_close) / vr.close_range,
    open_normalized = (sp."open" - vr.min_open) / vr.open_range,
    low_normalized = (sp."low" - vr.min_low) / vr.low_range,
    high_normalized = (sp."high" - vr.min_high) / vr.high_range
from value_ranges vr
where sp.ticker = vr.ticker;

drop index stock_prices_ticker_idx;