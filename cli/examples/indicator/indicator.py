import qpace as qp
import __QPC_PYTHON_PACKAGE__ as pine

ohlcv = qp.Ohlcv.read_csv("btc.csv", qp.Timeframe.Days(1))
ctx = qp.Ctx(ohlcv, qp.Sym.BTC_USD())

res = pine.indicator.main(ctx.copy(), src="close", slow_length=30, fast_length=14)
print(res["locals"]["ma"][0:90])
