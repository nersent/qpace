import qpace as qp
import __QPC_PYTHON_PACKAGE__ as pine

ohlcv = qp.Ohlcv.read_csv("btc.csv", qp.Timeframe.Days(1))
ctx = qp.Ctx(ohlcv, qp.Sym.BTC_USD())

ma = pine.library.ma(ctx.copy(), ohlcv.close, 30, 14)
print(ma[0:90])
