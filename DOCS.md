# qPACE Documentation

**qPACE**: The quant SDK for Python and JavaScript, written in Rust. Everything you need in one place - from research to production.

Get started at **<https://qpace.dev>**

<a href="https://qpace.dev/discord"><img src="https://cdn.nersent.com/public/badges/discord.svg" alt="Discord" /></a>

> Note: LLM friendly docs. Copy-paste into GPT.

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Usage](#usage)
- [Pine Script Compiler](#pine-script-compiler)
- [Symbol](#symbol)
  - [Symbol Kind](#symbol-kind)
  - [Example](#symbol-example)
- [Timeframe](#timeframe)
  - [Example](#timeframe-example)
- [OHLCV](#ohlcv)
  - [Python integration with Pandas](#ohlcv-python-integration-with-pandas)
  - [Reading/Writing files](#ohlcv-readingwriting-files)
  - [OHLCV bars](#ohlcv-bars)
  - [OHLCV DataFrame Ops](#ohlcv-dataframe-ops)
- [Context](#context)
  - [Example](#context-example)
  - [Iteration](#context-iteration)
  - [Sharing Context](#sharing-context)
- [Backtesting](#backtesting)
  - [Signal](#backtesting-signal)
  - [Example](#backtesting-example)
  - [Exporting to Pine Script](#exporting-to-pine-script)
- [Technical Analysis](#technical-analysis)
  - [Example](#technical-analysis-example)
  - [Indicators](#technical-analysis-indicators)
- [Plotting](#plotting)

## Introduction

Examples with actual setup are available [here](https://github.com/nersent/qpace-examples).

## Installation

Python:

```bash
pip install qpace
```

Javascript (Node.js and Browser/WASM):

```bash
npm install qpace
```

## Usage

Python:

```python
import qpace as qp

btc = qp.Sym.BTC_USD()
print(btc.min_tick)
```

Node.js:

```javascript
import * as qp from "qpace/node";

btc = qp.Sym.BTC_USD();
console.log(btc.minTick);
```

Browser / WASM:

[Here](https://github.com/nersent/qpace-examples/tree/main/web) is concrete example of how to use it in a browser. There may be some issues with webpack ect.

```javascript
import * as qp from "qpace/web";

window.onload = async () => {
  await qp.init();
  await qp.ta.init(); // qp.ta is a seperate WASM Module, so it needs be to initialized separately

  const btc = qp.Sym.BTC_USD();
  console.log(btc.minTick);
}
```

### Pine Script Compiler

We designed and developed in-house Pine Script compiler that takes your original Pine Script code and compiles it to efficient rust code that is later exposed to Python, Node.js and Web/WASM with type hints. Easy interface and practically no hustle from your side. Our compiler supports any technical analysis indicator and strategy, while having extreme performance. This can be used for backtesting, parameter optimization, bot automation, machine learning and much more.

1. Install qPACE CLI

```bash
npm install qpace -g
```

2. Initialize qPACE project

```bash
qp init
```

3. Copy-paste your Pine Script code into the project folder

```pine
//@version=5
library("MyLib")

export my_function(series float x) =>
    x * 2
```

## Symbol

A symbol is a financial instrument, such as BTC/USD or ETH/USD. In qPACE, symbols are represented by the `Sym` class.

Built-in symbols:

- `BTC_USD`
- `ETH_USD`
- `SOL_USD`
- `DOGE_USD`

### Symbol Kind

- `qp.SymKind.Stock()`
- `qp.SymKind.Future()`
- `qp.SymKind.Option()`
- `qp.SymKind.Forex()`
- `qp.SymKind.Crypto()`
- `qp.SymKind.Unknown()`
- `qp.SymKind.Other(data: String)`

### Symbol Example

Python:

```python
import qpace as qp

btc = qp.Sym.BTC_USD() 

other = qp.Sym()
other.id = "..." # arbitrary id, can be your internal id
other.ticker_id = "BINANCE:ETHUSD"
other.min_tick = 0.1
other.min_qty = 0.0001
other.prefix = "BINANCE"
other.currency = "USD"
other.base_currency = "ETH"
other.ticker = "ETHUSD"
other.kind = qp.SymKind.Crypto()
other.price_scale = 10.0
other.point_value = 1.0
```

Node.js:

```javascript
import * as qp from "qpace/node";

btc = qp.Sym.BTC_USD();

other = new qp.Sym();
other.id = "..."; 
other.tickerId = "BINANCE:ETHUSD";
other.minTick = 0.1;
other.minQty = 0.0001;
// ... same as Python
```

Browser / WASM:

```javascript
import * as qp from "qpace/web";

btc = qp.Sym.BTC_USD();
// ... same as Node.js
```

## Timeframe

A timeframe is a period of time over which data is aggregated. In qPACE, timeframes are represented by the `Timeframe` class.

- `qp.Timeframe.Years(years: number)`
- `qp.Timeframe.Months(months: number)`
- `qp.Timeframe.Weeks(weeks: number)`
- `qp.Timeframe.Days(days: number)`
- `qp.Timeframe.Hours(hours: number)`
- `qp.Timeframe.Minutes(minutes: number)`
- `qp.Timeframe.Seconds(seconds: number)`
- `qp.Timeframe.Ticks(seconds: number)`
- `qp.Timeframe.Ranges(seconds: number)`
- `qp.Timeframe.Unknown()`

### Timeframe Example

Python:

```python
timeframe = qp.Timeframe.Days(1)
print(timeframe.to_string())  # "1D"

timeframe = qp.Timeframe.from_str("1D")
```

Node.js / Web:

```javascript
timeframe = qp.Timeframe.Days(1);
console.log(timeframe.toString());  // "1D"

timeframe = qp.Timeframe.fromString("1D");
```

## OHLCV

OHLCV stands for Open, High, Low, Close, Volume. qPACE stores OHLCV similarly to how pandas stores DataFrames.

Python:

```python
ohlcv = qp.Ohlcv()
ohlcv.timeframe = qp.Timeframe.Days(1)  # optional: Set the timeframe to 1 day

print(len(ohlcv), str(ohlcv.timeframe))  # 0, "1D"
```

Node.js / Web:

```javascript
const ohlcv = new qp.Ohlcv();
ohlcv.timeframe = qp.Timeframe.Days(1);  // optional: Set the timeframe to 1 day

console.log(ohlcv.length, ohlcv.timeframe.toString());  // 0, "1D"
```

### OHLCV Python integration with Pandas

```python
import pandas as pd

df = pd.read_csv("btc.csv")
df["open_time"] = pd.to_datetime(df["open_time"], utc=True, unit='s')
df["close_time"] = pd.to_datetime(df["close_time"], utc=True, unit='s')

ohlcv = qp.Ohlcv.from_pandas(df)
ohlcv[0].volume # ...

df: pd.DataFrame = ohlcv.to_pandas()
```

### OHLCV Reading/Writing files

qPACE expects columns:

- *`time` (in seconds)
- *`open_time` (in seconds)
- *`close_time` (in seconds)
- `open` (float)
- `high` (float)
- `low` (float)
- `close` (float)
- `volume` (float)

*: We try to gracefully handle cases, where `open_time` and `close_time` are not present, but `time` is. It's best to have only `open_time` and `close_time`.

We also gracefully handle missing columns and replace then with `NaN` values.

Python:

```python
ohlcv = qp.Ohlcv.read_csv("btc.csv")
ohlcv = qp.Ohlcv.read_parquet("btc.parquet")

ohlcv.write_csv("btc.csv")
ohlcv.write_parquet("btc.parquet")
```

Node.js:

```javascript
ohlcv = qp.Ohlcv.readCsv("btc.csv");
ohlcv = qp.Ohlcv.readParquet("btc.parquet");

ohlcv.writeCsv("btc.csv");
ohlcv.writeParquet("btc.parquet");
```

Web / WASM:

We don't support reading/writing OHLCV at the moment.

### OHLCV bars

qPACE can easily operate on whole OHLCV or individual bars.
Python:

```python
from datetime import datetime

bar = qp.OhlcvBar(
  open_time=datetime.fromisoformat("2023-01-01T00:00:00"),
  close_time=datetime.fromisoformat("2023-01-01T00:01:00"),
  open=100.0,
  high=105.0,
  low=95.0,
  close=102.0,
  volume=1000.0
)

ohlcv = qp.Ohlcv()
ohlcv.push(bar)
print(len(ohlcv))  # 1

bars = ohlcv.bars()

# # 2023-01-01T00:00:00
print(bars[0].open_time)
print(ohlcv[0].open_time)
print(ohlcv.open_time[0]) 
```

Node.js / Web:

```javascript
const bar = new qp.OhlcvBar(
  new Date("2023-01-01T00:00:00Z"),
  new Date("2023-01-01T00:01:00Z"),
  100.0,
  105.0,
  95.0,
  102.0,
  1000.0
);

const ohlcv = new qp.Ohlcv();
ohlcv.push(bar);
console.log(ohlcv.length);  // 1
```

### OHLCV DataFrame Ops

Python:

```python
ohlcv = qp.Ohlcv()

ohlcv.bars # Access to all bars
ohlcv[index] # Access a bar by index
ohlcv.push(ohlcv_bar)  # Add a bar
ohlcv.push_many([ohlcv_bar, ohlcv_bar])  # Add multiple bars
ohlcv.pop()  # Remove the last bar
ohlcv.shift() # Remove the first bar
ohlcv.clear()  # Clear all bars
ohlcv.sort(ascending: bool)  # Sort bars by open time
ohlcv.reverse()
ohlcv.copy() # Create a copy of the OHLCV
ohlcv.head(n: int)  # Returns OHLCV with the first n bars
ohlcv.tail(n: int)  # Returns OHLCV with the last n bars
ohlcv.slice(start: int, end: int)  # Returns OHLCV with bars
```

Node.js / Web:

```javascript
const ohlcv = new qp.Ohlcv();

ohlcv.bars; // Access to all bars
ohlcv.at(index); // Access a bar by index
ohlcv.push(ohlcvBar); // Add a bar
ohlcv.pushMany([ohlcvBar, ohlcvBar]); // Add multiple bars
ohlcv.pop(); // Remove the last bar
ohlcv.shift(); // Remove the first bar
ohlcv.clear(); // Clear all bars
ohlcv.sort(ascending: bool); // Sort bars by open time
ohlcv.reverse();
ohlcv.copy(); // Create a copy of the OHLCV
ohlcv.head(10); // Returns OHLCV with the first 10 bars
ohlcv.tail(10); // Returns OHLCV with the last 10 bars
ohlcv.slice(0, 10); // Returns OHLCV with bars from index
```

## Context

A context combines Symbol, OHLCV and Timeframe into uniform structure, that can be easily copied/shared and passed around.

Think of it as a chart you see on a TradingView that you apply indicators to.

## Context Example

Python:

```python
import qpace as qp  

ctx = qp.Ctx()

#

ohlcv = qp.Ohlcv.read_csv("btc.csv")
ohlcv.timeframe = qp.Timeframe.Days(1)
sym = qp.Sym.BTC_USD()
ctx = qp.Ctx(ohlcv, sym)

#

print(ctx.bar_index()) # 0
print(ctx.bar().open_time) # 2023-01-01T00:00:00
print(ctx.ohlcv.timeframe.to_string()) # "1D"
print(ctx.sym.ticker_id) # "BINANCE:BTCUSD"
```

Node.js / Web:

```javascript
const ctx = new qp.Ctx();

// 

const ohlcv = qp.Ohlcv.readCsv("btc.csv"); // note: web doesn't support reading from disk
ohlcv.timeframe = qp.Timeframe.Days(1);
const sym = qp.Sym.BTC_USD();
const ctx = new qp.Ctx(ohlcv, sym);

//

console.log(ctx.barIndex()); // 0
console.log(ctx.bar().openTime); // 2023-01-01T00:00:00.000Z
console.log(ctx.ohlcv.timeframe.toString()); // "1D"
console.log(ctx.sym.tickerId); // "BINANCE:BTCUSD"
```

### Context Iteration

To advance context to next bar, you can use `ctx.next()` method or use iterator.

Python:

```python
for bar_index in ctx:
    print(ctx.bar_index(), ctx.bar().open_time)

#
while True:
    bar_index = ctx.next()
    if bar_index is None:
        break
    print(bar_index, ctx.bar().open_time)
```

Node.js / Web:

```javascript
for (const barIndex of ctx) {
    console.log(ctx.barIndex(), ctx.bar().openTime);
}

while (true) {
    const barIndex = ctx.next();
    if (barIndex === null) break;
    console.log(barIndex, ctx.bar().openTime);
}
```

### Sharing Context

- \`ctx.copy()\` - makes a new copy (same symbol/ohlcv/timeframe), but with disconnected bar position.

- \`ctx.ref()\` - makes a reference to the same context (same symbol/ohlcv/timeframe), but with the same bar position. This is useful when you want to share context between multiple components in your code.

Python:

```python
src_ctx = qp.Ctx(ohlcv, sym)
ctx_ref = src_ctx.ref()
ctx_copy = src_ctx.copy()

print(src_ctx.bar_index())  # 0
src_ctx.next()
print(src_ctx.bar_index())  # 1
print(ctx_ref.bar_index())  # 1 (same bar position)
print(ctx_copy.bar_index())  # 0 (different bar position)
```

Node.js / Web:

```javascript
const srcCtx = new qp.Ctx(ohlcv, sym);
const ctxRef = srcCtx.ref();
const ctxCopy = srcCtx.copy();

console.log(srcCtx.barIndex()); // 0
srcCtx.next();
console.log(srcCtx.barIndex()); // 1
console.log(ctxRef.barIndex()); // 1 (same bar position)
console.log(ctxCopy.barIndex()); // 0 (different bar position)
```

## Backtesting

qPACE backtesting engine is written in Rust and allows very fast realization of signals.

### Backtesting Signal

- `qp.Signal.Hold()` # does nothing
- `qp.Signal.Size(size: float)` # order size
- `qp.Signal.EquityPct(pct: float)` # total % of equity allocated to the asset
- `qp.Signal.CloseAll()` # close all positions
- `qp.Signal.Long()` # open long position with 100% equity
- `qp.Signal.Short()` # open short position with 100% equity

### Backtesting Example

Python:

```python
import qpace as qp

ctx = qp.Ctx(ohlcv, sym)
bt = qp.Backtest(
  ctx,
  initial_capital = 1000.0, # default
  process_orders_on_close = False, # default
)

for bar_index in bt:
    if bar_index == 100:
        bt.signal(qp.Signal.EquityPct(0.5)) # long 50%
    elif bar_index == 200:
        bt.signal(qp.Signal.EquityPct(0.1)) # short 10%
    elif bar_index == 300:
        bt.signal(qp.Signal.CloseAll()) # close all positions

bt.display() # displays result table and plots chart
```

Node.js / Web:

```javascript
const ctx = new qp.Ctx(ohlcv, sym);
const bt = new qp.Backtest(
  ctx,
  1000.0, // initial capital
  false, // process orders on close
);

for (const barIndex of bt) {
    if (barIndex === 100) {
        bt.signal(qp.Signal.EquityPct(0.5)); // long 50%
    } else if (barIndex === 200) {
        bt.signal(qp.Signal.EquityPct(0.1)); // short 10%
    } else if (barIndex === 300) {
        bt.signal(qp.Signal.CloseAll()); // close all positions
    }
}

bt.display(); // displays result table and plots chart
```

### Exporting to Pine Script

Python:

```python
print(bt.to_pine())
```

Node.js / Web:

```javascript
console.log(bt.toPine());
```

It generates a script with exact entries and exits so, you can copy and paste it into TradingView editor. It's advised to use the same symbol and timeframe you did backtest on.

### Technical Analysis

Technical Analysis (TA) indicators are generated by our Pine compiler from [ta.pine file](/content/ta.pine). Bindings for Python, Node.js and Web are automatically generated.

### Technical Analysis Example

Python:

```python
import qpace as qp
ctx = qp.Ctx(ohlcv, sym)
rsi = qp.ta.rsi(ctx.copy(), 14)
macd = qp.ta.macd(ctx.copy(), 12, 26)
```

Node.js:

```javascript
import * as qp from "qpace/node";

const ctx = new qp.Ctx(ohlcv, sym);
const rsi = qp.ta.rsi(ctx.copy(), 14);
const macd = qp.ta.macd(ctx.copy(), 12, 26);
```

Browser / WASM:

```javascript
import * as qp from "qpace/web";

window.onload = async () => {
  await qp.init();
  await qp.ta.init(); // qp.ta is a separate WASM Module, so it needs to be initialized separately

  const ctx = new qp.Ctx(ohlcv, sym);
  const rsi = qp.ta.rsi(ctx.copy(), 14);
  const macd = qp.ta.macd(ctx.copy(), 12, 26);
}
```

### Technical Analysis Indicators

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

### Plotting

We are working on a plotting library with UI/dashboard. More on our [Discord](https://qpace.dev/discord).

---

This is a section for LLMs.

Implementation details:

Node.js, WASM and CLI are the same npm package.

```json
  "exports": {
    "./node": {
      "import": "./lib/node/index.js",
      "require": "./lib/node/index.js",
      "default": "./lib/node/index.js"
    },
    "./web": {
      "import": "./lib/web/index.js",
      "require": "./lib/web/index.js",
      "default": "./lib/web/index.js"
    },
    "./compiler/*": {
      "import": "./compiler/*.js",
      "require": "./compiler/*.js",
      "types": "./compiler/*.d.ts"
    },
    "./package.json": "./package.json"
  },
  "typesVersions": {
    "*": {
      "node": [
        "lib/node/index.d.ts"
      ],
      "web": [
        "lib/web/index.d.ts"
      ],
      "compiler": [
        "index.d.ts",
        "schema_grpc_pb.d.ts",
        "schema_pb.d.ts"
      ]
    }
  },
```

WASM is exposed as following:

```javascript
export {
  Ohlcv,
  // ... other items
} from "../../core/pkg/qpace_core";

import init from "../../core/pkg/qpace_core";
export { init };

import * as _ta from "../../content/web/ta";
import { init as initTa } from "../../content/web";
export const ta = { ..._ta, init: initTa } as typeof _ta & {
  init: typeof initTa;
};
```

WASM may have problems with webpack. [webpack config that works](https://github.com/nersent/qpace-examples/blob/main/web/webpack.config.js)

WASM `ta` must be initialized after `qp.init()` and is optional unless you use `qp.ta` methods
