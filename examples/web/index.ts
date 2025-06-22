import * as qp from "qpace/web";
import * as pine from "qpace_example_lib";

const bars = [
  {
    open_time: "2011-08-18T00:00:00+00:00",
    close_time: "2011-08-18T00:00:00+00:00",
    open: 10.9,
    high: 10.9,
    low: 10.9,
    close: 10.9,
    volume: NaN,
  },
  {
    open_time: "2011-08-19T00:00:00+00:00",
    close_time: "2011-08-19T00:00:00+00:00",
    open: 11.85,
    high: 11.85,
    low: 11.15,
    close: 11.69,
    volume: NaN,
  },
  {
    open_time: "2011-08-20T00:00:00+00:00",
    close_time: "2011-08-20T00:00:00+00:00",
    open: 11.7,
    high: 11.7,
    low: 11.7,
    close: 11.7,
    volume: NaN,
  },
  {
    open_time: "2011-08-21T00:00:00+00:00",
    close_time: "2011-08-21T00:00:00+00:00",
    open: 11.7,
    high: 11.7,
    low: 11.7,
    close: 11.7,
    volume: NaN,
  },
  {
    open_time: "2011-08-22T00:00:00+00:00",
    close_time: "2011-08-22T00:00:00+00:00",
    open: 11.7,
    high: 11.7,
    low: 11.7,
    close: 11.7,
    volume: NaN,
  },
  {
    open_time: "2011-08-24T00:00:00+00:00",
    close_time: "2011-08-24T00:00:00+00:00",
    open: 11.5,
    high: 11.5,
    low: 10.5,
    close: 10.5,
    volume: NaN,
  },
  {
    open_time: "2011-08-25T00:00:00+00:00",
    close_time: "2011-08-25T00:00:00+00:00",
    open: 10,
    high: 10,
    low: 10,
    close: 10,
    volume: NaN,
  },
  {
    open_time: "2011-08-30T00:00:00+00:00",
    close_time: "2011-08-30T00:00:00+00:00",
    open: 8,
    high: 8,
    low: 8,
    close: 8,
    volume: NaN,
  },
  {
    open_time: "2011-09-01T00:00:00+00:00",
    close_time: "2011-09-01T00:00:00+00:00",
    open: 8,
    high: 8.31,
    low: 5.25,
    close: 8.22,
    volume: NaN,
  },
  {
    open_time: "2011-09-02T00:00:00+00:00",
    close_time: "2011-09-02T00:00:00+00:00",
    open: 8.12,
    high: 8.88,
    low: 8,
    close: 8.88,
    volume: NaN,
  },
];

window.onload = async () => {
  await qp.init();
  // console.log(qp);
  // console.log(qp.Sym.BTC_USD().toJSON());
  await pine.init();
  const ohlcv = qp.Ohlcv.fromBars(bars.map((r) => qp.OhlcvBar.fromJSON(r)));
  const ctx = new qp.Ctx(ohlcv);
  const res = pine.xd.cum(ctx.copy(), Array.from(ctx.ohlcv.close));
  console.log(res);
  // console.log(pine)
};

// import { initWasm } from "qpace/web";

// (async () => {
//   const wasm = await initWasm(); // ← returns the real exports
//   console.log("keys:", Object.keys(wasm));
//   // Must list Ctx, Backtest, … here.
// })();

// import { initWasm, Ctx } from "qpace/web";
// import * as qp from "qpace/web"; // namespace import (optional)

// console.log("qpace/web resolves to:", require.resolve("qpace/web"));

// window.onload = async () => {
//   await initWasm(); // makes sure the .wasm is live

//   // either form now works:
//   console.log(Ctx); // named import
//   console.log(qp.Ctx); // namespace import

//   const ctx = new Ctx(); // or Ctx.new() if you didn’t add #[wasm_bindgen(constructor)]
//   console.log(ctx);
// };

// import { initWasm, Ctx } from "qpace/web";

// import * as qp from "qpace/web";

// window.onload = async () => {
//   await initWasm(); // make sure the .wasm is instantiated
//   console.log(qp.Ctx);
// console.log(new qp.Ctx());
// await qp.init();
// await pine.init();

// const root = document.getElementById("root")!;
// root.textContent = `${qp.VERSION}`;
// console.log(qp.Timeframe.Days(1).toString());

// console.log(pine.xd.cum(ctx, Array.from(ctx.ohlcv.close)));
// };
