import qpace as qp

# `qp.Ctx` is used for sharing the same context between for example calculating indicators, backtesting and more.

client = qp.Client(api_key="ENTER_YOUR_API_KEY_HERE")

# Constructor
if True:
    sym = qp.Sym.btc_usd()
    ohlcv = qp.Ohlcv()
    ctx = qp.Ctx(ohlcv=ohlcv, sym=sym)

# Fetching
if True:
    ctx = client.ctx("BITSTAMP:BTCUSD", timeframe=qp.Timeframe.Days(1))

# `ctx.fork()` - creates a new context, seperate from the original one, but with the same symbol and ohlcv and bar position.
if True:
    new_ctx = ctx.fork()
    print(new_ctx.bar_index == ctx.bar_index)  # True

# Resetting current bar position
if True:
    ctx.reset()
    print(ctx.bar_index)  # 0
    print(ctx.is_initialized)  # false

# Moving to next bar
if True:
    bar_index = ctx.next()
    print(bar_index == ctx.bar_index)  # True

# Iterating over bars
if True:
    while ctx.next() is not None:
        print(ctx.bar_index, ctx.bar.close)
if True:
    for bar_index in ctx:
        print(bar_index, ctx.bar.close)
