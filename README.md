<div align="center">
  <img src="static/banner.svg" alt="qPACE banner" />
  <h1>qPACE</h1>
  <p><strong>The Quant SDK for Python&nbsp;·&nbsp;JavaScript&nbsp;·&nbsp;Rust</strong></p>
  <em>From research to production - all in one toolkit.</em>
  <br/>
  <br/>
  <a href="https://pypi.org/project/qpace/"><img src="https://img.shields.io/pypi/v/qpace?color=blue&label=pypi" alt="PyPI"></a>
  <a href="https://www.npmjs.com/package/qpace"><img src="https://img.shields.io/npm/v/qpace?color=red&label=npm" alt="NPM"></a>
  <a href="https://qpace.dev/discord"><img src="https://discordapp.com/api/guilds/1238782377229160498/widget.png?style=shield" alt="Discord"></a>
  <br/>
</div>

  <!-- <a href="https://qpace.dev/discord"><img src="https://cdn.nersent.com/public/badges/discord.svg" alt="Discord"></a> -->
<!-- <iframe src="https://discord.com/widget?id=1238782377229160498&theme=dark" width="350" height="500" allowtransparency="true" frameborder="0" sandbox="allow-popups allow-popups-to-escape-sandbox allow-same-origin allow-scripts"></iframe> -->
---

**qPACE**: The all-in-one quantitative toolkit powered by Rust - usable from Python, Node.js, and the browser.

- Cross‑language, cross‑platform - high‑performance Rust core with the fully typed API for Python, Node.js (NAPI), and Browser (WebAssembly).

- Extremely fast backtesting engine - millions of bars per second. Export exact trades back to Pine for one‑click visual validation.

- Technical Analysis - more than 30 indicators fully compliant with TradingView results, written in [Pine](/content/ta.pine) and compiled using our [compiler](#pine-from-pythonjavascript).

- Data layer - resampling/aggregation, zipping/unzipping, reading/writing from CSV/Parquet, and more.

- CLI + upcoming UI

## Quick Links

- [Home](https://qpace.dev)
- [Documentation](/DOCS.md)
- [Examples](/https://github.com/nersent/qpace-examples)
- [Discord](https://qpace.dev/discord)

## Installation

### Python

```bash
pip install qpace
```

### JavaScript

```bash
npm install qpace
```

## Pine from Python/JavaScript

We designed and developed in-house Pine Script compiler that takes your original Pine Script code and compiles it to efficient rust code that is later exposed to Python, Node.js and Web/WASM with type hints. Easy interface and practically no hustle from your side. Our compiler supports any technical analysis indicator and strategy, while having extreme performance. This can be used for backtesting, parameter optimization, bot automation, machine learning and much more.

[Get started](https://qpace.dev)

## Quick Example

Python

```python
import qpace as qp

ohlcv = qp.Ohlcv.read_csv("btc.csv")
ctx = qp.Ctx(ohlcv, qp.Sym.BTC_USD())
rsi = qp.ta.rsi(ctx.copy(), ohlcv.close, 14)
```

Node.js

```javascript
import * as qp from "qpace/node";

const ohlcv = qp.Ohlcv.readCsv("btc.csv");
const ctx = new qp.Ctx(ohlcv, qp.Sym.BTC_USD());
const rsi = qp.ta.rsi(ctx.copy(), ohlcv.close, 14);
```

## TA

- Accumulation/Distribution (ACCDIST)
- Relative Strength Index (RSI)
- Moving Average Convergence Divergence (MACD)
- Bollinger Bands (BB, BB %b, BB width)
- Average True Range (ATR)
- True Range (TR)
- Exponential Moving Average (EMA)
- Simple Moving Average (SMA)
- Relative Moving Average (RMA)
- Volume-Weighted Moving Average (VWMA)
- Linear Weighted Moving Average (LWMA)
- Symmetrically Weighted Moving Average (SWMA)
- Hull Moving Average (HMA)
- Awesome Oscillator (AO)
- Balance of Power (BOP)
- Choppiness Index (CHOP)
- Chande-Kroll Stop (CKS)
- Aroon
- Commodity Channel Index (CCI)
- Supertrend
- Chaikin Money Flow (CMF)
- Coppock Curve
- Donchian Channels
- Price Oscillator (PO)
- Relative Vigor Index (RVGI)
- Relative Volatility Index (RVI)
- Ultimate Oscillator (UO)
- Vortex Indicator (VI)
- Williams %R
- Rate of Change (ROC)
- Change
- Cumulative Sum (CUM)
- Bars Since
- Cross/Over/Under
- Highest/Lowest/Bars
- Standard Deviation (DEV)
- Volume Oscillator (VO)
- And more coming soon!

## Community

Become a part of the qPACE community and connect with like-minded individuals who are passionate about trading, finance, and technology!

[Join Discord](https://qpace.dev/discord)
