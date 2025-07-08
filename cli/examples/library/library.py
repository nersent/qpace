import qpace as qp
import __QPC_PYTHON_PACKAGE__ as pine

ohlcv_path = "btc.csv"
ohlcv = qp.Ohlcv.read_csv(ohlcv_path)
ohlcv.timeframe = qp.Timeframe.Days(1)
sym = qp.Sym.BTC_USD()
ctx = qp.Ctx(ohlcv, sym)

ma = pine.library.ma(ctx.copy(), ohlcv.close, 30, 14)
print(ma[0:90])
