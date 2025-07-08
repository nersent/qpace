import matplotlib.pyplot as plt
import qpace as qp
import __QPC_PYTHON_PACKAGE__ as pine

client = qp.Client("ENTER_YOUR_API_KEY")
ctx = client.ctx("BITSTAMP:BTCUSD", "1D", pb=True)
ohlcv = ctx.ohlcv

rsi = qp.ta.rsi(ctx.fork(), ohlcv.close)
ma = pine.custom_indicator.ma(ctx.fork(), src=ohlcv.close, length=90)
fig, (ax1, ax2) = plt.subplots(2, 1)
ax1.plot(ohlcv.open_time, ohlcv.close, label="Close", color="black")
ax1.plot(ohlcv.open_time, ma, label="Pine MA", color="red")
ax2.plot(ohlcv.open_time, rsi, label="RSI", color="blue")
ax1.legend()
ax2.legend()
plt.show()
