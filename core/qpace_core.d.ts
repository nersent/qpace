/* tslint:disable */
/* eslint-disable */
export function zipOhlcvBars(open_time?: Date[] | null, close_time?: Date[] | null, open?: Float64Array | null, high?: Float64Array | null, low?: Float64Array | null, close?: Float64Array | null, volume?: Float64Array | null): OhlcvBar[];
export function getVersion(): string;
/**
 * https://pmc.ncbi.nlm.nih.gov/articles/PMC4614595/
 */
export function specificity(fp_count: number, tn_count: number): number;
/**
 * https://pmc.ncbi.nlm.nih.gov/articles/PMC4614595/
 */
export function sensitivity(tp_count: number, fp_count: number): number;
/**
 * https://pmc.ncbi.nlm.nih.gov/articles/PMC4614595/
 */
export function accuracy(tp_count: number, fp_count: number, fn_count: number, tn_count: number): number;
/**
 * https://python.plainenglish.io/the-kelly-criterion-maximizing-returns-through-optimal-betting-32781a768ffb
 */
export function kellyCriterion(win_prob: number, profit_factor: number): number;
export function maxRunUpPct(max_run_up: number, bar_equity_max: number): number;
export function maxDrawdownPct(max_dd: number, net_equity_max: number): number;
export function shortNetProfitPct(short_net_profit: number, initial_capital: number): number;
export function longNetProfitPct(long_net_profit: number, initial_capital: number): number;
export function grossLossPct(gross_loss: number, initial_capital: number): number;
export function grossProfitPct(gross_profit: number, initial_capital: number): number;
export function netProfitPct(net_profit: number, initial_capital: number): number;
export function sortinoRatioFromEquity(equity: Float64Array, risk_free_rate: number): number;
export function sortinoRatio(mean_returns: number, negative_returns_stdev: number, risk_free_rate: number): number;
export function sharpeRatioFromEquity(equity: Float64Array, risk_free_rate: number): number;
export function sharpeRatio(mean_returns: number, std_returns: number, risk_free_rate: number): number;
export function omegaRatio(positive_returns_sum: number, negative_returns_sum: number, risk_free_rate: number): number;
export function avgWinLossRatio(avg_winning_trade: number, avg_losing_trade: number): number;
export function avgLosingTrade(gross_loss: number, losing_trades: number): number;
export function avgWinningTrade(gross_profit: number, winning_trades: number): number;
export function avgTrade(net_profit: number, closed_trades: number): number;
export function winRate(profitable_trades: number, total_trades: number): number;
export function longNetProfitRatio(long_net_profit: number, short_net_profit: number): number;
export function profitFactor(gross_profit: number, gross_loss: number): number;
export function pnl(qty: number, entry_price: number, current_price: number): number;
export function expectancy(pnl_series: Float64Array): number;
export function expectancyScore(expectancy: number, opportunity_bars: number): number;
/**
 *
 * Calculates returns from equity (% change)
 * Returns without first item, because it would be NAN.
 * Example: [1.0, 2.0] -> [2.0] // 200%
 */
export function returns(equity: Float64Array, pad: boolean): Float64Array;
export function pctChange(current: number, previous: number): number;
export function stdev(values: Float64Array): number;
export function stdevFromVar(_var: number): number;
export function variance(values: Float64Array): number;
export function varFromMean(values: Float64Array, mean: number): number;
export function mean(values: Float64Array): number;
export function sum(values: Float64Array): number;
export function orderSizeForEquityPct(equity_pct: number, equity: number, current_position: number, instrument_price: number, point_value: number, exchange_rate: number): number;
/**
 *
 * the calculated order size, rounded down to the smallest trade quantity. For stocks, futures, CFDs, and forex that minimum quantity is 1. For Bitcoin (BTCUSD) it's 0.000001 and Ethereum (ETHUSD) uses 0.0001.
 *
 * # Parameters
 *
 * * `equity_pct` - A `f64` representing the percentage of the current strategy equity to invest in each order. This percentage is derived from either the `default_qty_value` setting or the manual 'Order size' option within the strategy's settings window.
 *
 * * `equity` - A `f64` representing the strategy's current equity. This is the sum of the initial capital, closed net profit, and open position profit. Note that this includes unrealized profits/losses, which may affect the calculated order size if the open position's result changes significantly when it's closed.
 *
 * * `exchange_rate` - A `f64` used for currency conversion, if necessary. If the strategy currency and the instrument currency are the same, this should be 1. Otherwise, provide the conversion rate between the two currencies.
 *
 * * `instrument_price` - A `f64` representing the last available price at the time the order is generated. This is typically the close price of the bar on which the order is generated, unless using options like 'Recalculate After Order Filled' or 'Recalculate On Every Tick', which may use a different price within the bar.
 *
 * * `point_value` - A `f64` that denotes the currency amount of one full point of price movement for the instrument. For example, it is 1 for stocks and 20 for the E-mini Nasdaq 100 futures.
 *
 * # Returns
 *
 * Returns a `f64` representing the calculated order size, rounded down to the smallest trade quantity based on the instrument type.
 */
export function orderSize(equity_pct: number, equity: number, exchange_rate: number, instrument_price: number, point_value: number): number;
export function roundToMinTick(value: number, min_tick: number): number;
/**
 * Checks if `size` is a valid order quantity by comparing it to the minimum order quantity.
 */
export function validateContracts(size: number, min_qty: number): boolean;
/**
 * Rounds `size` to the nearest multiple of the minimum order quantity.
 */
export function roundContracts(size: number, min_qty: number): number;
export function hlcc4(high: number, low: number, close: number): number;
export function hlc3(high: number, low: number, close: number): number;
export function hl2(high: number, low: number): number;
export enum TradeDirection {
  Long = 1,
  Short = -1,
}
export class Backtest {
  free(): void;
  constructor(js_ctx: Ctx, config: BacktestConfig);
  signal(signal: Signal): void;
  /**
   * Processes multiple signals at once. `signals` must be aligned with all bars. `signals: [bar_index_0_signal, bar_index_1_signal, ...Signal[]]`.
   */
  signalBatch(signals: Array<any>): void;
  /**
   * Processes multiple signals at once. `signals: Map<bar_index: number, Signal>`.
   */
  signalBatchMap(signals: Map<any, any>): void;
  skipRemainingBars(): void;
  skipToBar(bar_index: number): void;
  skipBars(bars: number): void;
  toPine(): string;
  readonly ctx: Ctx;
  readonly config: BacktestConfig;
  /**
   * `initial capital + net profit + open profit`
   */
  readonly equity: number;
  /**
   * `initial_capital + net_profit`
   */
  readonly netEquity: number;
  readonly equitySeries: Float64Array;
  readonly netEquitySeries: Float64Array;
  readonly equityReturns: Float64Array;
  readonly netEquityReturns: Float64Array;
  readonly pnlSeries: Float64Array;
  readonly openProfit: number;
  /**
   * Overall profit or loss.
   */
  readonly netProfit: number;
  /**
   * Total value of all completed winning trades.
   */
  readonly grossProfit: number;
  /**
   * Total value of all completed losing trades.
   */
  readonly grossLoss: number;
  /**
   * Total number of winning trades.
   */
  readonly winningTrades: number;
  /**
   * Total number of losing trades.
   */
  readonly losingTrades: number;
  /**
   * Direction and size of the current market position. If the value is > 0, the market position is long. If the value is < 0, the market position is short. The absolute value is the number of contracts/shares/lots/units in trade (position size).
   */
  readonly positionSize: number;
  readonly openTrades: Trade[];
  readonly closedTrades: Trade[];
  readonly trades: Trade[];
  readonly openLongs: number;
  readonly openShorts: number;
  readonly closedLongs: number;
  readonly closedShorts: number;
  readonly totalTongs: number;
  readonly totalShorts: number;
  readonly totalTrades: number;
  readonly instrumentPrice: number;
  readonly winRate: number;
  readonly profitFactor: number;
  readonly length: number;
}
export class BacktestConfig {
  free(): void;
  constructor(initial_capital?: number | null, process_orders_on_close?: boolean | null);
  readonly initialCapital: number;
  readonly processOrdersOnClose: boolean;
}
export class Ctx {
  private constructor();
  free(): void;
  next(): number | undefined;
  readonly barIndex: number;
  readonly bar: OhlcvBar;
  readonly isInitialized: boolean;
  readonly sym: Sym;
  readonly length: number;
}
export class OhlcvBar {
  free(): void;
  constructor(open_time?: Date | null, close_time?: Date | null, open?: number | null, high?: number | null, low?: number | null, close?: number | null, volume?: number | null);
  toString(): string;
  readonly openTime: Date;
  readonly closeTime: Date;
  readonly openTimeMs: number;
  readonly closeTimeMs: number;
  readonly open: number;
  readonly high: number;
  readonly low: number;
  readonly close: number;
  readonly volume: number;
  readonly hl2: number;
  readonly hlc3: number;
  readonly hlcc4: number;
}
export class Order {
  private constructor();
  free(): void;
}
export class OrderBook {
  private constructor();
  free(): void;
}
export class OrderConfig {
  private constructor();
  free(): void;
}
export class Signal {
  private constructor();
  free(): void;
  hold(): Signal;
  size(size: number): Signal;
  equityPct(equity_pct: number): Signal;
  closeAll(): Signal;
  long(): Signal;
  short(): Signal;
  readonly kind: SignalKind;
  get id(): string | undefined;
  set id(value: string | null | undefined);
}
export class SignalKind {
  private constructor();
  free(): void;
}
export class Sym {
  private constructor();
  free(): void;
  static btc_usd(): Sym;
  static eth_usd(): Sym;
  static sol_usd(): Sym;
  readonly minTick: number;
  readonly minQty: number;
}
export class Timeframe {
  private constructor();
  free(): void;
  toString(): string;
  static years(value: number): Timeframe;
  static months(value: number): Timeframe;
  static weeks(value: number): Timeframe;
  static days(value: number): Timeframe;
  static hours(value: number): Timeframe;
  static minutes(value: number): Timeframe;
  static seconds(value: number): Timeframe;
  static ticks(value: number): Timeframe;
  static ranges(value: number): Timeframe;
}
export class Trade {
  private constructor();
  free(): void;
  readonly size: number;
  readonly entry: TradeEvent | undefined;
  readonly exit: TradeEvent | undefined;
  readonly pnl: number;
  readonly direction: TradeDirection;
  readonly isActive: boolean;
  readonly isClosed: boolean;
}
export class TradeEvent {
  private constructor();
  free(): void;
  readonly id: string | undefined;
  readonly orderBarIndex: number;
  readonly fillBarIndex: number;
  readonly price: number;
  readonly comment: string | undefined;
}
