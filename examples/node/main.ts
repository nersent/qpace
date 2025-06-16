import { resolve } from "path";
import * as qp from "qpace/node";
import * as pine from "qpace_example_lib";

const ohlcvPath = resolve(__dirname, "../btc.csv");
const sym = qp.Sym.BTC_USD();
const ohlcv = qp.Ohlcv.read_csv(ohlcvPath);
ohlcv.timeframe = qp.Timeframe.Days(1);
const ctx = new qp.Ctx(ohlcv, sym);

console.log(pine.xd.hma(ctx.copy(), ctx.ohlcv.close));
console.log(
  pine.xd.crossover(
    ctx.copy(),
    ctx.ohlcv.close,
    pine.xd.sma(ctx.copy(), ctx.ohlcv.open, 14),
  ),
);
// console.log(
//   pine.xd.gowno(ctx.copy(), [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 69.0, 420.0]),
// );
// import * as qp from "qpace/web";
// qp.init(() => {
//   console.log(qp.Timeframe.Days(1));
// });
