import qpace as qp

client = qp.Client(api_key="ENTER_YOUR_API_KEY_HERE")

# Constructor
if True:
    sym = qp.Sym.btc_usd()
    ohlcv = qp.Ohlcv()
    ctx = qp.Ctx(ohlcv=ohlcv, sym=sym)
    bt = qp.Backtest(ctx, initial_capital=1000.0, process_orders_on_close=False)

# Fetching
if True:
    ctx = client.ctx("BITSTAMP:BTCUSD", timeframe=qp.Timeframe.Days(1))
    bt = qp.Backtest(ctx)
    print(str(bt.ctx.timeframe), bt.ctx.sym.ticker_id, len(bt.ctx.ohlcv))

# Iterating over bars
if True:
    for bar_index in bt:
        print(bt.ctx.bar_index, bt.ctx.bar_index)

# Iterating over bars with explicit lifecycle
if True:
    for bar_index in bt.ctx:
        bt.on_bar_open()
        # if `bt.process_orders_on_close` is false, orders are processed here
        if True:
            # your signals here
            pass
        bt.on_bar_close()
        # if `bt.process_orders_on_close` is true, orders are processed here

# Iterating over bars with signals
if True:
    for bar_index in bt.ctx:
        if bar_index == 1:
            bt.signal(qp.Signal.short())
        elif bar_index == 20:
            bt.signal(qp.Signal.close_all())
        elif bar_index == 50:
            bt.signal(qp.Signal.long())
        elif bar_index == 52:
            bt.signal(qp.Signal.close_all())
    bt.print()

# Array Vectorized backtest
if True:
    signals = [
        qp.Signal.hold(),
        None,  # same as `qp.Signal.hold()`
        qp.Signal.short(),
        qp.Signal.close_all(),
        qp.Signal.long(),
        qp.Signal.long(),  # nothing happens
        qp.Signal.short(),  # flips to short
    ]
    if len(signals) != len(bt.ctx):
        raise ValueError("Signals length must be equal to ctx length")
    bt.signal_batch(signals)
    bt.print()

# Dict Vectorized backtest
if True:
    signals = {
        1: qp.Signal.short(),
        2: None,  # same as `qp.Signal.hold()`
        20: qp.Signal.close_all(),
        50: qp.Signal.long(),
        52: qp.Signal.close_all(),
    }
    bt.signal_batch_dict(signals)
    bt.print()

# Exporting backtest to Pine
if True:
    print(bt.to_pine())
    # copy-pastable to other platforms

# Skipping bars
if True:
    bt.skip()  # skips all remaining bars
    bt.skip(bars=5)  # skips 5 next bars
    bt.skip(bar_index=20)  # skips to bar index 20

# Backtest metrics
if True:
    print(bt.equity, bt.net_equity, bt.open_profit, bt.position_size)  # and more

# Equity curve
if True:
    current_equity = bt.equity
    all_equity_list = bt.equity_list
