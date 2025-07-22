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

- Cross-ecosystem - interoperable with Pandas, Polars, and more.

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

## Pine from Python/JavaScript

We designed and developed in-house Pine Script compiler that takes your original Pine Script code and compiles it to efficient rust code that is later exposed to Python, Node.js and Web/WASM with type hints. Easy interface and practically no hustle from your side. Our compiler supports any technical analysis indicator and strategy, while having extreme performance.

- bot automation
- machine learning
- backtesting
- parameter optimization
- and much more

[Get started](https://qpace.dev)

![Compiler gif](/static/compiler.gif)

`script.pine`

```pine
//@version=5
library("MyLibrary")

export custom_ma(series float src, int length) =>
    ta.ema(ta.change(src) * volume, length)
```

Python:

```python
import qpace as qp
import my_library as pine

ctx = qp.Ctx(ohlcv, qp.Sym.BTC_USD())
custom_ma = pine.script.custom_ma(ctx.copy(), ohlcv.close, 14)
print(custom_ma) # [1.0, 2.0, ...]
```

Node.js:

```javascript
import * as qp from "qpace/node";
import * as pine from "my_library"; 

const ctx = new qp.Ctx(ohlcv, qp.Sym.BTC_USD());
const customMa = pine.script.custom_ma(ctx.copy(), ohlcv.close, 14);
console.log(customMa); // [1.0, 2.0, ...]
```

## Suite

**qPACE Suite**: Free collection of the best indicators and strategies.

Python:

```bash
pip install qpace_suite
```

JavaScript:

```bash
npm install @qpace/suite
```

### _Jdehorty_

- [_Machine Learning: Lorentzian Classification_](https://www.tradingview.com/v/WhBzgfDu/)
- [_WaveTrend 3D_](https://www.tradingview.com/v/clUzC70G/)
- [_Nadaraya-Watson: Envelope_](https://www.tradingview.com/v/WeLssFxl/)
- [_MLExtensions_](https://www.tradingview.com/v/ia5ozyMF/)
- [_KernelFunctions_](https://www.tradingview.com/v/e0Ek9x99/)

### _AlgoAlpha_

- [_Adaptive Schaff Trend Cycle (STC)_](https://www.tradingview.com/v/yOxili7R/)
- [_Amazing Oscillator_](https://www.tradingview.com/v/g9j9piQE/)
- [_Donchian Trend Ranges_](https://www.tradingview.com/v/td0irJcf/)
- [_Exponential Trend_](https://www.tradingview.com/v/CDb3oR6A/)
- [_Supertrended RSI_](https://www.tradingview.com/v/tjP35RG5/)
- [_Triple Smoothed Signals_](https://www.tradingview.com/v/FoMINXVf/)

## TA

Built-in TA functions.

```python
import qpace as qp
rsi = qp.ta.rsi(ctx.copy(), src=ohlcv.close, length=14)
```

```javascript
import * as qp from "qpace/node";
const rsi = qp.ta.rsi(ctx.copy(), ohlcv.close, 14);
```

[Roadmap](https://github.com/nersent/qpace/issues/7)

> Every TA indicator was compiled using Pine to Python/JavaScript compiler.

### _Momentum_ (17)

- _Awesome Oscillator_ **AO**
- _Absolute Price Oscillator_ **APO**
- _Balance of Power_ **BOP**
- _Commodity Channel Index_ **CCI**
- _Coppock Curve_
- _KST Oscillator_ **KST**
- _Moving Average Convergence Divergence_ **MACD**
- _Momentum_ **MOM**
- _Price Oscillator_ **PO**
- _Rate of Change_ **ROC**
- _Relative Strength Index_ **RSI**
- _Relative Vigor Index_ **RVGI**
- _Stochastic RSI_ **STOCHRSI**
- _Trix_ **TRIX**
- _True Strength Index_ **TSI**
- _Ultimate Oscillator_ **UO**
- _Williams %R_ **W%R**

---

### _Overlap_ (11)

- _Double Exponential MA_ **DEMA**
- _Exponential MA_ **EMA**
- _Fibonacci Weighted MA_ **FMWA**
- _Hull MA_ **HMA**
- _Linear Weighted MA_ **LWMA**
- _Relative MA_ **RMA**
- _Simple MA_ **SMA**
- _Symmetrically Weighted MA_ **SWMA**
- _Triple Exponential MA_ **TEMA**
- _Volume-Weighted MA_ **VWMA**
- _Weighted MA_ **WMA**

---

### _Trend_ (8)

- _Advance/Decline Ratio_ **ADR**
- _Aroon_ **AROON**
- _Bull/Bear Power_ **BBP**
- _Chande-Kroll Stop_ **CKS**
- _Choppiness Index_ **CHOP**
- _Detrended Price Oscillator_ **DPO**
- _Supertrend_ **ST**
- _Vortex Indicator_ **VI**

---

### _Volatility_ (7)

- _Average True Range_ **ATR**
- _Bollinger Bands_ **BB**
- _Bollinger %B_ **%B**
- _Bollinger Width_ **BBW**
- _Donchian Channel_ **DC**
- _Relative Volatility Index_ **RVI**
- _True Range_ **TR**

---

### _Volume_ (6)

- _Accumulation/Distribution (Williams)_ **ACCDIST**
- _Chaikin Money Flow_ **CMF**
- _Elder’s Force Index_ **EFI**
- _Ease of Movement_ **EOM**
- _Money Flow Index_ **MFI**
- _Volume Oscillator_ **VO**

---

### _Statistics_ (1)

- _Standard Deviation_ **STD**

---

### _Utilities & Helpers_ (11)

- _Bars Since_
- _Change_
- _Cross_
- _Cross-Over_
- _Cross-Under_
- _Cumulative Sum_ **CUM**
- _Highest_
- _Highest Bars_
- _Lowest_
- _Lowest Bars_
- _Rate of Change_ **ROC**

## Community

Become a part of the qPACE community and connect with like-minded individuals who are passionate about trading, finance, and technology!

[Join Discord](https://qpace.dev/discord)
