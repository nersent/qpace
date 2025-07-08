import * as qp from "qpace/node";
import * as pine from "__QPC_PYTHON_PACKAGE__/node";

const ohlcv = qp.Ohlcv.readCsv("btc.csv", qp.Timeframe.Days(1));
const ctx = qp.Ctx(ohlcv, qp.Sym.BTC_USD());

res = pine.indicator.main(ctx.fork(), "close", 30, 14);
console.log(res["locals"]["ma"].slice(0, 90));
