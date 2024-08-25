import pandas as pd
import qpace as qp

df: pd.DataFrame = pd.read_csv("btc.csv")
sym: qp.Symbol = qp.Symbol.btc_usd()
dp: qp.DataProvider = qp.DataProvider.from_pandas(df, sym)

aroon_up, aroon_down = qp.ta.aroon(dp, length=14)

bt: qp.Backtest = qp.Backtest(dp, initial_capital=1000.0)

for bar_index in bt:
    if aroon_up[bar_index] > aroon_down[bar_index]:
        bt.signal(qp.Signal.long())
    elif aroon_up[bar_index] < aroon_down[bar_index]:
        bt.signal(qp.Signal.short())

bt.print()
