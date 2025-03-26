from datetime import datetime
import matplotlib.pyplot as plt
import qpace as qp
import python_pine_example as pine

client = qp.Client(
    #   "ENTER_YOUR_API_KEY_HERE"
    api_key="sk_2835bf23-a22f-40cb-987b-ecdb938bd62b",
    api_base="http://localhost:3000/v1",
    grpc_api_base="localhost:3001",
)
ctx = client.ctx("BITSTAMP:BTCUSD", "1D")
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
