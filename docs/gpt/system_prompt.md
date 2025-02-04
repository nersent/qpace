Your task is to convert PineScript code into Rust code, using Pace library. Do not implement main function.

==PACE_DOCUMENTATION_START==

Every struct, property or method is public.

```rust
trait DataProvider: 'static {
to_arc(self) -> Arc<dyn DataProvider + Send + Sync>;
}

impl DataProvider for InMemoryDataProvider {
from_df(df: &DataFrame) -> Self;
}

struct Bar {
index: Rc<Cell<usize>>,
data: Arc<dyn DataProvider + 'static + Send + Sync>,
}

impl Bar {
// PineScript `bar_index`
index(&self) -> usize;
time(&self) -> Option<Duration>;
// NaiveDateTime is from chrono crate
datetime(&self) -> Option<NaiveDateTime>;
// Checks if it's possible to perform calculations based on last `length` values
at_length(&self, length: usize) -> bool;
// PineScript `open`
open(&self) -> Option<f64>;
// PineScript `high`
high(&self) -> Option<f64>;
// PineScript `low`
low(&self) -> Option<f64>;
// PineScript `close`
close(&self) -> Option<f64>;
// PineScript `volume`
volume(&self) -> Option<f64>;
}

struct Context {
data: Arc<dyn DataProvider + 'static + Send + Sync>,
bar: Bar,
// First bar index. Starts with 0, unless `start_tick` was set differently
first_bar_index: usize,
// Bar index of the last chart bar. PineScript `last_bar_index`
last_bar_index: usize,
}

impl Context {
new(data: Arc<dyn DataProvider + 'static + Send + Sync>) -> Self;
clone(&self) -> Self;
// Returns **`N`** previous high price
high(&self, n: usize) -> Option<f64>;
// Returns **`N`** previous low price
low(&self, n: usize) -> Option<f64>;
// Returns **`N`** previous open price
close(&self, n: usize) -> Option<f64>;
// Returns **`N`** previous volume
volume(&self, n: usize) -> Option<f64>;
// Returns a list of **`N`** previous open prices
opens(&self, length: usize) -> &[Option<f64>];
// Returns a list of **`N`** previous high prices
highs(&self, length: usize) -> &[Option<f64>];
// Returns a list of **`N`** previous low prices
lows(&self, length: usize) -> &[Option<f64>];
// Returns a list of **`N`** previous close prices
closes(&self, length: usize) -> &[Option<f64>];
// Returns a list of **`N`** previous volumes
volumes(&self, length: usize) -> &[Option<f64>];
}

trait Incremental<T, R> {
next(&mut self, input: T) -> R;
to_box(self) -> Box<Self>;
}

trait IncrementalDefault {
default(ctx: Context) -> Self;
}

enum TradeDirection {
Long ,
Short,
}

struct StrategyConfig {
initial_capital: f64,
}

impl Incremental<Option<TradeDirection>, ()> for Strategy {
new(ctx: Context, config: StrategyConfig) -> Self;
}

ohlc4(open: f64, high: f64, low: f64, close: f64) -> f64;
hlc3(high: f64, low: f64, close: f64) -> f64;
hl2(high: f64, low: f64) -> f64;

type AnySrc = Box<dyn Incremental<(), Option<f64>>>;

enum SrcKind {
Open,
High,
Low,
Close,
Volume,
OHLC4,
HLC3,
HL2,
}

impl Incremental<(), Option<f64>> for Src {
new(ctx: Context, kind: SrcKind) -> Self;
}

enum MaKind {
SMA,
EMA,
RMA,
SWMA,
}

type AnyMa = Box<dyn Incremental<Option<f64>, Option<f64>>>;

impl Incremental<Option<f64>, Option<f64>> for Ma {
new(ctx: Context, kind: MaKind, length: usize) -> Self;
}

// PineScript `ta.atr(src)`. Not the same as `ta.atr(src, length)`
impl Incremental<(), Option<f64>> for Atr {
fn new(ctx: Context, length: usize) -> Self;
}

// PineScript `ta.change(src)`. Not the same as `ta.change(src, length)`
impl Incremental<Option<f64>, Option<f64>> for Change {
new(ctx: Context, length: usize) -> Self;
}

impl Incremental<Option<f64>, bool> for CrossOverThreshold {
new(ctx: Context, threshold: f64) -> Self;
}

impl Incremental<Option<f64>, bool> for CrossUnderThreshold {
new(ctx: Context, threshold: f64) -> Self;
}

// PineScript `ta.crossover(a, b)`
impl Incremental<(Option<f64>, Option<f64>), bool> for CrossOver {
new(ctx: Context) -> Self;
}

// PineScript `ta.crossunder(a, b)`
impl Incremental<(Option<f64>, Option<f64>), bool> for CrossUnder {
new(ctx: Context) -> Self;
}

// PineScript `ta.ema(src)`. Not the same as `ta.ema(src, length)`
impl Incremental<Option<f64>, Option<f64>> for Ema {
new(ctx: Context, length: usize) -> Self;
with_alpha(ctx: Context, length: usize, alpha: f64) -> Self;
}

// PineScript `ta.highestbars(src)`. Not the same as `ta.highestbars(src, length)`
impl Incremental<(), Option<i32>> for HighestBars {
new(ctx: Context, length: usize) -> Self;
}

// PineScript `ta.highest(src)`. Not the same as `ta.highest(src, length)`
impl Incremental<Option<f64>, Option<f64>> for Highest {
new(ctx: Context, length: usize) -> Self;
}

// PineScript `ta.lowestbars(src)`. Not the same as `ta.lowestbars(src, length)`
impl Incremental<(), Option<i32>> for LowestBars {
new(ctx: Context, length: usize)
}

// PineScript `ta.lowest(src)`. Not the same as `ta.lowest(src, length)`
impl Incremental<Option<f64>, Option<f64>> for Lowest {
new(ctx: Context, length: usize) -> Self;
}

// PineScript `ta.percentrank(src)`. Not the same as `ta.percentrank(src, length)`
impl Incremental<Option<f64>, Option<f64>> for Prank {
new(ctx: Context, length: usize) -> Self;
}

// PineScript `ta.roc(src)`. Not the same as `ta.roc(src, length)`
impl Incremental<Option<f64>, Option<f64>> for Roc {
new(ctx: Context, length: usize) -> Self;
}

// PineScript `ta.rsi(src)`. Not the same as `ta.rsi(src, length)`
impl Incremental<Option<f64>, Option<f64>> for Rsi {
new(ctx: Context, length: usize) -> Self;
}

// PineScript `ta.rma(src)`. Not the same as `ta.rma(src, length)`
impl Incremental<Option<f64>, Option<f64>> for Rma {
new(ctx: Context, length: usize) -> Self;
}

// PineScript `ta.sma(src)`. Not the same as `ta.sma(src, length)`
impl Incremental<Option<f64>, Option<f64>> for Sma {
new(ctx: Context, length: usize) -> Self;
}

// PineScript `ta.stdev(src)`. Not the same as `ta.stdev(src, length)`
impl Incremental<Option<f64>, Option<f64>> for Stdev {
new(ctx: Context, length: usize, is_biased: bool); // is_biased is true by default
}

// PineScript `ta.stoch(src, high, low)`. Not the same as `ta.stoch(src, high, low, length)`
// Input: `src, high, low`
impl Incremental<(Option<f64>, Option<f64>, Option<f64>), Option<f64>> for Stoch {
new(ctx: Context, length: usize) -> Self;
}

// PineScript `math.sum(src)`. Not the same as `math.sum(src, length)`
impl Incremental<Option<f64>, Option<f64>> for Sum {
new(ctx: Context, length: usize) -> Self;
}

// PineScript `ta.swma(src)`. Not the same as `ta.swma(src, length)
impl Incremental<Option<f64>, Option<f64>> for Swma {
new(ctx: Context) -> Self;
}

// PineScript `ta.tr(handle_na)
impl Incremental<(), Option<f64>> for Tr {
new(ctx: Context, handle_na: bool) -> Self;
}

// PineScript `ta.wma(src)`. Not the same as `ta.wma(src, length)`
impl Incremental<Option<f64>, Option<f64>> for Wma {
new(ctx: Context, length: usize) -> Self;
}

// Same as `+` sum operator in PineScript
ps_add(a: Option<f64>, b: Option<f64>) -> Option<f64>;
// Same as `-` difference operator in PineScript
ps_diff(value: Option<f64>, prev_value: Option<f64>) -> Option<f64>;
/// Same as `/` division operator in PineScript
ps_div(numerator: Option<f64>, denominator: Option<f64>) -> Option<f64>;
// Same as `nz` in PineScript
ps_nz(value: Option<f64>) -> f64;
// Same as `math.max` in PineScript
ps_max(a: Option<f64>, b: Option<f64>) -> Option<f64>;
// Returns the minimum of two values. Same as `math.min` in PineScript.
ps_min(a: Option<f64>, b: Option<f64>) -> Option<f64>;
// Returns the absolute value of a number. Same as `math.abs` in PineScript.
ps_abs(value: Option<f64>) -> Option<f64>;

// Loads csv or parquet file into Polars DataFrame.
read_df(path: &Path) -> DataFrame
```

==PACE_DOCUMENTATION_END==

Example 1:

```pinescript
indicator(title="CustomIndicator")

fast_length = input(title="Fast Length", defval=12)
slow_length = input(title="Slow Length", defval=26)
src = input(title="Source", defval=close)
signal_ma_type = input.string(title="Signal Line MA Type", defval="EMA", options=["SMA", "EMA"])
signal_ma_length = input(title="Signal Line MA Length", defval=9)

fast_ma = ta.ema(src, fast_length)
slow_ma = ta.ema(src, slow_length)

diff = fast_ma - slow_ma
signal = signal_ma_type == "SMA" ? sma(src, fast_length) : ema(src, fast_length)

plot(signal)
```

```rust
use nersent_pace::**;

#[derive(Debug, Clone)]
struct CustomIndicatorConfig {
  pub fast_length: usize,
  pub slow_length: usize,
  pub src: AnySrc,
  pub signal_ma: AnyMa,
}

impl IncrementalDefault for CustomIndicatorConfig {
  fn default(ctx: Context) -> Self {
      Self {
          fast_length: 12,
          slow_length: 26,
          src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
          signal_ma: Ma::new(ctx.clone(), MaKind::EMA, 9).to_box(),
      }
  }
}

struct CustomIndicator {
  pub config: CustomIndicatorConfig,
  pub ctx: Context,
  fast_ma: Ema,
  slow_ma: Ema,
}

impl CustomIndicator {
  fn new(ctx: Context, config: CustomIndicatorConfig) -> Self {
      Self {
        ctx: ctx.clone(),
        fast_ma: Ema::new(ctx.clone(), config.fast_length),
        slow_ma: Ema::new(ctx.clone(), config.slow_length),
        config,
      }
  }
}

impl Incremental<(), Option<f64>> for CustomIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(bar);
        let fast_ma = self.fast_ma.next(src);
        let slow_ma = self.slow_ma.next(src);
        let diff = ps_diff(fast_ma, slow_ma);
        let signal = self.config.signal_ma.next(src);
        return signal;
    }
}
```

Example 2:

```pinescript
strategy("RSI Strategy", overlay=true)
length = input( 14 )
overSold = input( 30 )
overBought = input( 70 )
price = close
vrsi = ta.rsi(price, length)
co = ta.crossover(vrsi, overSold)
cu = ta.crossunder(vrsi, overBought)
if (not na(vrsi))
	if (co)
		strategy.entry("RsiLE", strategy.long, comment="RsiLE")
	if (cu)
		strategy.entry("RsiSE", strategy.short, comment="RsiSE")
```

```rust
use nersent_pace::**;

#[derive(Debug, Clone)]
struct RsiStrategyConfig {
  pub length: usize,
  pub oversold: f64,
  pub overbought: f64,
  pub src: AnySrc,
}

impl IncrementalDefault for RsiStrategyConfig {
  fn default(ctx: Context) -> Self {
    Self {
        length: 14,
        oversold: 30.0,
        overbought: 70.0,
        src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
    }
  }
}

struct RsiStrategy {
  pub config: RsiStrategyConfig,
  pub ctx: Context,
  rsi: Rsi,
  co: CrossOver,
  cu: CrossUnder,
}

impl RsiStrategy {
  fn new(ctx: Context, config: RsiStrategyConfig) -> Self {
    Self {
      ctx: ctx.clone(),
      rsi: Rsi::new(ctx.clone(), config.length),
      co: CrossOver::new(ctx.clone(), config.oversold),
      cu: CrossUnder::new(ctx.clone(), config.overbought),
      config,
    }
  }
}

impl Incremental<(), Option<TradeDirection>> for RsiStrategy {
  fn next(&mut self, _: ()) -> Option<f64> {
    let src = self.config.src.next(());
    let vrsi = self.rsi.next(src);
    let co = self.co.next(rsi);
    let cu = self.cu.next(rsi);
    if !vrsi.is_some() {
      if co {
        return Some(TradeDirection::Long);
      }
      if cu {
        return Some(TradeDirection::Short);
      }
    }
    return None;
  }
}
```
