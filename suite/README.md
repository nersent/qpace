# AlgoAlpha x QPACE

QPACE:

- [Website](https://qpace.dev)
- [Documentation](/DOCS.md)

AlgoAlpha:

- [Website](https://www.algoalpha.io/)

## Indicators

- [Adaptive Schaff Trend Cycle (STC)](https://www.tradingview.com/v/yOxili7R/)
- [Amazing Oscillator (AO)](https://www.tradingview.com/v/g9j9piQE/)
- [Exponential Trend](https://www.tradingview.com/v/CDb3oR6A/)
- [Supertrended RSI](https://www.tradingview.com/v/tjP35RG5/)
- [Triple Smoothed Signals](https://www.tradingview.com/v/FoMINXVf/)

## Python

```bash
pip install qpace algoalpha
```

```python
import qpace as qp
import algoalpha as aa

ohlcv = qp.Ohlcv.read_csv("btc_csv", qp.Timeframe.Days(1))
ctx = qp.Ctx(ohlcv, qp.Sym.BTC_USD())

res = aa.supertrended_rsi.main(ctx.copy())
print(res["locals"]["rsi_value"][0:30])
```

## JavaScript

```bash
npm add qpace algoalpha
```

Node.js:

```ts
import * as qp from "qpace/node";
import * as aa from "algoalpha/node";

const ohlcv = qp.Ohlcv.readCsv("btc_csv", qp.Timeframe.Days(1))
const ctx = qp.Ctx(ohlcv, qp.BTC_USD())

const res = aa.supertrended_rsi.main(ctx.copy())
console.log(res.locals.rsi_value.slice(0, 30))
```

Browser:

```ts
import * as qp from "qpace/web";
import * as aa from "algoalpha/web";

window.onload = async () => {
  // Initialize WASM Modules
  {
    await qp.init();
    await qp.ta.init();
    await aa.init();
  }

  const ohlcv = qp.Ohlcv.fromBars([ ... ])
  const ctx = qp.Ctx(ohlcv, qp.BTC_USD())
  
  const res = aa.supertrended_rsi.main(ctx.copy())
  console.log(res.locals.rsi_value.slice(0, 30))
}

```
