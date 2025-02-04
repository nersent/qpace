import os
import pandas as pd
import qpace as qp

# See examples/ohlcv.py for data loading examples
df_path = os.path.join("btc.csv")
ohlcv_loader: qp.OhlcvLoader = qp.OhlcvLoader.read_path(df_path)
sym: qp.Symbol = qp.Symbol.btc_usd()  # default
ctx: qp.Context = qp.Context(ohlcv_loader, sym)

aroon_up, aroon_down = zip(*qp.ta.aroon(ctx=ctx.fork(), length=14))

bt: qp.Backtest = qp.Backtest(
    ctx.fork(),
    config=qp.BacktestConfig(
        initial_capital=1000.0,  # default,
        process_orders_on_close=False,  # default, processes on next bar open
    ),
)

for bar_index in bt:
    if aroon_up[bar_index] > aroon_down[bar_index]:
        bt.order(qp.Order(size=1.0, tag="long"))  # buy 1.0 BTC
    elif aroon_up[bar_index] < aroon_down[bar_index]:
        bt.order(qp.Order(size=-1.0, tag="short"))  # sell 1.0 BTC

print(bt.equity)
