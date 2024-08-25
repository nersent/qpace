import pandas as pd
import qpace as qp

df: pd.DataFrame = pd.read_csv("btc.csv")
close: list[float] = df["close"].tolist()
high: list[float] = df["high"].tolist()
low: list[float] = df["low"].tolist()

rsi: list[float] = qp.ta.relative_strength_index(length=14, src=close)
aroon_up, aroon_down = qp.ta.aroon(length=14, high=high, low=low)

# or use DataProvider that can be reused multiple times
sym: qp.Symbol = qp.Symbol.btc_usd()
dp: qp.DataProvider = qp.DataProvider.from_pandas(df, sym)

rsi: list[float] = qp.ta.relative_strength_index(dp, length=14)
aroon_up, aroon_down = qp.ta.aroon(dp, length=14)

print(rsi[0:30])
print(aroon_up[0:30])
print(aroon_down[0:30])
