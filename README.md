<div align="center">
  <img src="static/logo.svg" width="184">

<br />

<h1><b>qPACE</b></h1>

<a href="https://discord.gg/P7Vn4VX"><img src="https://cdn.nersent.com/public/badges/discord.svg" alt="Discord" /></a>

</div>

**qPACE**: The Technical Analysis framework for Python and JavaScript, written in Rust, designed to be extremely fast.

![backtest summary](/static/backtest_summary.png)

## Table of Contents

- [Features](#features)

- [Installation](#installation)

- [Examples](#examples)

- [Pine from Python](#pine-from-python)

- [Motivation](#motivation)

- [Contributing](#contributing)

## Features

- Simple, yet powerful API

- Comparable results to TradingView Pine

- Extremly fast backtesting - supports vectorized and standalone signals\

- Rich collection of built-in technical analysis indicators - written in [Pine](/content/ta.pine), compiled via [QPC](#pine-from-python)
  - Accumulation/Distribution (ACCDIST)
  - Relative Strength Index (RSI)
  - Moving Average Convergence Divergence (MACD)
  - Bollinger Bands (BB, BB %b, BB width)
  - Stochastic Oscillator
  - Average True Range (ATR)
  - True Range (TR)
  - Exponential Moving Average (EMA)
  - Simple Moving Average (SMA)
  - RMA
  - VWMA
  - LWMA
  - SWMA
  - Choppiness Index (CHOP)
  - Chande Kroll Stop (CKS)
  - Aroon
  - Commodity Channel Index (CCI)
  - Cross/Over/Under
  - Highest/Lowest/Bars
  - Change
  - Cumulative (CUM)
  - Bars Since
  - Rate of Chance (ROC)
  - SuperTrend
  - Chaikin Money Flow (CMF)
  - Coppock Curve
  - Doncnian Channels
  - Price Oscillator
  - Relative Vigor Index (RVGI)
  - Relative Volatility Index (RVI)
  - Ultimate Oscillator
  - Vortex Indicator
  - Williams %R

## Installation

### Python

```bash
pip install qpace
```

### JavaScript

```bash
npm install qpace
```

## Examples

## Pine from Python

QPC is specially designed compiler to translate any Pine code into efficient Rust code that is then exposed to Python and JavaScript, allowing you to run your favorite Pine scripts in the most efficient way possible.

## Motivation

## Contributing
