import os
from typing import Tuple
from matplotlib import pyplot as plt
import qpace as qp

# See examples/ohlcv.py for data loading examples
df_path = os.path.join("btc.csv")
ohlcv_loader: qp.OhlcvLoader = qp.OhlcvLoader.read_path(df_path)
ctx = qp.Context(ohlcv_loader=ohlcv_loader)

# Explicit sources/parameters
rsi: list[float] = qp.ta.rsi(src=ohlcv_loader.close, length=14)
aroon = qp.ta.aroon(low=ohlcv_loader.low, high=ohlcv_loader.high, length=14)

plt.title("RSI")
plt.plot(rsi)
plt.show()

# Implicit sources/parameters automatically provided by qp.Context
aroon = qp.ta.aroon(length=14, ctx=ctx.fork())
aroon_up, aroon_down = zip(*aroon)

plt.title("Aroon")
plt.plot(aroon_up)
plt.plot(aroon_down)
plt.show()
