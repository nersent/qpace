import qpace as qp
import __QPC_PYTHON_PACKAGE__ as pine

ohlcv_path = "btc.csv"
ohlcv = qp.Ohlcv.read_csv(ohlcv_path)
ohlcv.timeframe = qp.Timeframe.Days(1)
sym = qp.Sym.BTC_USD()
ctx = qp.Ctx(ohlcv, sym)

res = pine.indicator.main(ctx.copy(), src="close", slow_length=30, fast_length=14)
print(res["locals"]["ma"][0:90])
