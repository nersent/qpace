import * as qp from "qpace/web";

window.onload = async () => {
  await qp.init();
  await qp.ta.init();

  let { default: bars } = await import("../assets/btc_1d.json");
  bars = bars.map((r) => qp.OhlcvBar.fromJSON(r));
  const ohlcv = qp.Ohlcv.fromBars(bars);
  ohlcv.timeframe = qp.Timeframe.Days(1);

  const ctx = new qp.Ctx(ohlcv, qp.Sym.BTC_USD());
  console.log(qp.ta.rsi(ctx.copy(), ctx.ohlcv.close, 14));
};
