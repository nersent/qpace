import * as qp from "qpace/node";
import * as pine from "__QPC_PYTHON_PACKAGE__/node";

const ohlcvPath = "btc.csv";
const ohlcv = qp.Ohlcv.readCsv(ohlcvPath);
ohlcv.timeframe = qp.Timeframe.Days(1);
const sym = qp.Sym.BTC_USD();
const ctx = qp.Ctx(ohlcv, sym);

res = pine.indicator.main(ctx.fork(), "close", 30, 14);
console.log(res["locals"]["ma"].slice(0, 90));
