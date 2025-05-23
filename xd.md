Roadmap for initial public release of qPACE quant library.

We plan to release Rust crate, Python wheel and NPM package with support for platforms:

- Windows x64
- Linux x64
- MacOS x64
- MacOS ARM

# Features

## Compiler #6

## Core

### `Sym`

- [ ] `Sym.id`
- [ ] `Sym.min_tick`
- [ ] `Sym.min_qty`
- [ ] `Sym.ticker_id`
- [ ] `Sym.prefix`
- [ ] `Sym.currency`
- [ ] `Sym.base_currency`
- [ ] `Sym.ticker`
- [ ] `Sym.country`
- [ ] `Sym.kind`
- [ ] `Sym.price_scale`
- [ ] `Sym.point_value`
- [ ] `Sym.eq()`
- [ ] `Sym.btc_usd()`
- [ ] `Sym.eth_usd()`
- [ ] `Sym.doge_usd()`
- [ ] `Sym.sol_usd()`
- [ ] `Sym.msft()`
- [ ] `Sym.alphabet()`
- [ ] `Sym.meta()`
- [ ] `Sym.gold()`

### `SymKind`

- [ ] `crypto|stocks`
- [ ] `SymKind.eq`

### `Timeframe`

- [x] `Timeframe.to_string`
- [x] `Timeframe.from_string`
- [x] `Timeframe::years()`
- [x] `Timeframe::months()`
- [x] `Timeframe::weeks()`
- [x] `Timeframe::days()`
- [x] `Timeframe::hours()`
- [x] `Timeframe::minutes()`
- [x] `Timeframe::seconds()`
- [x] `Timeframe::ticks()`
- [x] `Timeframe::ranges()`
- [x] `Timeframe::unknown()`
- [x] `Timeframe.years`
- [x] `Timeframe.months`
- [x] `Timeframe.weeks`
- [x] `Timeframe.days`
- [x] `Timeframe.hours`
- [x] `Timeframe.minutes`
- [x] `Timeframe.seconds`
- [x] `Timeframe.ticks`
- [x] `Timeframe.ranges`
- [x] `Timeframe.unknown`
- [x] `Timeframe.to_duration()`
- [x] `Timeframe.from_duration()`
- [x] `Timeframe.eq()`

### `OhlcvBar`

- [ ] `open_time`
- [ ] `close_time`
- [ ] `open`
- [ ] `high`
- [ ] `low`
- [ ] `close`
- [ ] `volume`
- [ ] `merge`

### `Ohlcv`

- [ ] `timeframe`
- [ ] `empty`
- [ ] `from_bars`
- [ ] `resample`
- [ ] `sort`
- [ ] `copy`
- [ ] `slice`
- [ ] `index_of`
- [ ] `find`
- [ ] `bars`
- [ ] `at`
- [ ] `reverse`
- [ ] `from_pandas`
- [ ] `to_pandas`
- [ ] `from_polars`
- [ ] `to_polars`
- [ ] `write_csv`
- [ ] `read_csv`
- [ ] `write_parquet`
- [ ] `read_parquet`
- [ ] `open_time_list`
- [ ] `close_time_list`
- [ ] `open_list`
- [ ] `high_list`
- [ ] `low_list`
- [ ] `close_list`
- [ ] `volume_list`
- [ ] `push`
- [ ] `push_many`
- [ ] `extend`
- [ ] `len`

### OHLCV utils

- [ ] `zip_ohlcv_bars`
- [ ] `unzip_ohlcv_bars`

### `SymSource`

- [ ] `qpace|yahoo`
- [ ] `sym`

### `OhlcvSource`

- [ ] `qpace|yahoo`
- [ ] `sym`

### `Ctx`

- [ ] `sym`
- [ ] `ohlcv`
- [ ] `copy`
- [ ] `reset`
- [ ] `bar_index`
- [ ] `bar`
- [ ] `len`
- [ ] `is_initialized`
- [ ] `next`
- [ ] `skip`

### `Backtest`

- [ ] `config`
- [ ] `ctx`
- [ ] `equity`
- [ ] `equity_list`
- [ ] `net_equity_list`
- [ ] `returns_list`
- [ ] `net_returns_list`
- [ ] `pnl_list`
- [ ] `open_profit`
- [ ] `net_profit`
- [ ] `gross_profit`
- [ ] `gross_loss`
- [ ] `trades`
- [ ] `instrument_size`
- [ ] `next`
- [ ] `on_bar_open`
- [ ] `on_bar_close`
- [ ] `synthetic_hold`
- [ ] `signal`
- [ ] `signal_batch`
- [ ] `skip`
- [ ] `report`

### `BacktestReport`

- [ ] `ctx`
- [ ] `trades`
- [ ] `print`
- [ ] `plot`
- [ ] `to_pine`
- [ ] `to_json`
- [ ] `to_pandas`

### `Signal`

- [ ] `Order|Size|EquityPct|Hold|Close)`
- [ ] `long`
- [ ] `short`
- [ ] `equity_pct`
- [ ] `close`

### `SignalBatch`

- [ ] `from_list`
- [ ] `from_bar_index_map`
- [ ] `from_open_time_map`

## Utils

- [ ] `hl2`
- [ ] `hlc3`
- [ ] `hlcc4`

### Order Sizing

- [ ] `round_contracts`
- [ ] `validate_contracts`
- [ ] `round_to_min_tick`
- [ ] `order_size`
- [ ] `order_size_for_equity_pct`

### Statistics

- [ ] `sum`
- [ ] `mean`
- [ ] `var_from_mean`
- [ ] `var`
- [ ] `stdev_from_var`
- [ ]  `stdev`
- [ ] `pct_change`
- [ ] `returns`

### Metrics

- [ ] `expectancy_score`
- [ ] `profit_factor`
- [ ] `win_rate`
- [ ] `avg_trade`
- [ ] `avg_winning_trade`
- [ ] `avg_losing_trade`
- [ ] `avg_win_loss_ratio`
- [ ] `omega_ratio`
- [ ] `sharpe_ratio`
- [ ] `sharpe_ratio_from_equity`
- [ ] `sortino_ratio`
- [ ] `sortino_ratio_from_equity`
- [ ] `net_profit_pct`
- [ ] `gross_profit_pct`
- [ ] `gross_loss_pct`
- [ ] `long_net_profit_pct`
- [ ] `short_net_profit_pct`
- [ ] `max_drawdown_pct`
- [ ] `avg_max_drawdown_pct`
- [ ] `max_run_up_pct`
- [ ] `kelly_criterion_gambling`
- [ ] `kelly_criterion_Investment`
- [ ] `accuracy`
- [ ] `recall`
- [ ] `precision`
- [ ] `f1`
- [ ] `auc`
- [ ] `scale_proba`
- [ ] `step_proba`

## TA

- [ ] incremental functions
- [ ] vectorized functions

### Content

- [ ] `accdist`
- [ ] `cum`
- [ ] `change`
- [ ] `barssince`
- [ ] `roc`
- [ ] `crossover`
- [ ] `crossunder`
- [ ] `cross`
- [ ] `highestbars`
- [ ] `lowestbars`
- [ ] `highest`
- [ ] `lowest`
- [ ] `swma`
- [ ] `sma`
- [ ] `ema`
- [ ] `rma`
- [ ] `wma`
- [ ] `lwma`
- [ ] `hma`
- [ ] `vwma`
- [ ] `dev`
- [ ] `tr`
- [ ] `atr`
- [ ] `rsi`
- [ ] `cci`
- [ ] `aroon`
- [ ] `supertrend`
- [ ] `awesome_oscillator`
- [ ] `balance_of_power`
- [ ] `bollinger_bands_pct_b`
- [ ] `bollinger_bands_width`
- [ ] `bollinger_bands`
- [ ] `chaikin_money_flow`
- [ ] `chande_kroll_stop`
- [ ] `choppiness_index`
- [ ] `coppock_curve`
- [ ] `donchian_channel`
- [ ] `macd`
- [ ] `price_oscillator`
- [ ] `relative_vigor_index`
- [ ] `relative_volatility_index`
- [ ] `ultimate_oscillator`
- [ ] `volume_oscillator`
- [ ] `vortex_indicator`
- [ ] `williams_pct_r`

## Optimization

- [ ] grid search (user model, fixed params)
- [ ] genetic algorithm (user model, fixed params)
- [ ] neuroevolution algorithm (dynamic model, dynamic params)
- [ ] symbolic evolution (dynamic model, fixed params)

## Plotting

- [ ] API definitions
- [ ] UI

## qPACE Remote

- [ ] client
- [ ] sym
- [ ] ohlcv
- [ ] predictions
- [ ] customer dashboard

## CLI

- [ ] Integrate with qPACE Remote API
- [ ] Compiler
- [ ] Plot data
- [ ] Backtest (plotting, exporting ect)
