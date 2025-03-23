import qpace as qp
import python_pine_example as pine
import matplotlib.pyplot as plt

client = qp.Client(api_key="ENTER_YOUR_API_KEY_HERE")
ctx = client.ctx("BITSTAMP:BTCUSD", timeframe=qp.Timeframe.Days(1))
ohlcv = ctx.ohlcv

rsi = qp.ta.rsi(ctx.fork(), ohlcv.close)
ma = pine.my_library.ma(ctx.fork(), src=ohlcv.close, length=90)
fig, (ax1, ax2) = plt.subplots(2, 1)
ax1.plot(ohlcv.open_time, ohlcv.close, label="Close", color="black")
ax1.plot(ohlcv.open_time, ma, label="Pine MA", color="red")
ax2.plot(ohlcv.open_time, rsi, label="RSI", color="blue")
ax1.legend()
ax2.legend()
plt.show()
