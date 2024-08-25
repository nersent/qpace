import pandas as pd
import qpace as qp
import numpy as np

df: pd.DataFrame = pd.read_csv("btc.csv")
sym: qp.Symbol = qp.Symbol.btc_usd()
dp: qp.DataProvider = qp.DataProvider.from_pandas(df, sym)

aroon_up, aroon_down = qp.ta.aroon(dp, length=14)
aroon_up = np.array(aroon_up)
aroon_down = np.array(aroon_down)

bt: qp.Backtest = qp.Backtest(dp, initial_capital=1000.0)

long: list[bool] = (aroon_up > aroon_down).astype(bool).tolist()
short: list[bool] = (aroon_up < aroon_down).astype(bool).tolist()

bt.signal_batch(long=long, short=short)

bt.print()
