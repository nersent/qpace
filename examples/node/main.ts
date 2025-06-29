import { resolve } from "path";
import * as qp from "qpace/node";
import * as pine from "qpace_example_lib";

const ohlcvPath = resolve(__dirname, "../btc.csv");
const ohlcv = qp.Ohlcv.read_csv(ohlcvPath);
ohlcv.timeframe = qp.Timeframe.Days(1);
const sym = qp.Sym.BTC_USD();
const ctx = new qp.Ctx(ohlcv, sym);

const x = pine.xd.cum(ctx.copy(), ctx.ohlcv.close);
console.log(x);
