create index
    if not exists stock_prices_ticker_idx on stock_prices (ticker);

with value_ranges as (
        select
            1.0 / max(sp."close") - min(sp."close") as close_range,
            max(sp."open") - min(sp."open") as open_range,
            max(sp."high") - min(sp."high") as high_range,
            max(sp."low") - min(sp."low") as low_range,
            sp.ticker as ticker
        from stock_prices sp
        group by sp.ticker
    )
update stock_prices as sp
set
    close_normalized = sp."close" / vr.close_range,
    open_normalized = sp."open" / vr.open_range,
    low_normalized = sp."low" / vr.low_range,
    high_normalized = sp."high" / vr.high_range
from value_ranges vr
where sp.ticker = vr.ticker;

drop index stock_prices_ticker_idx;