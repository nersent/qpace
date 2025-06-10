export {
  NodeOhlcv as Ohlcv,
  NodeOhlcvBar as OhlcvBar,
  NodeSym as Sym,
  NodeSymKind as SymKind,
  NodeCtx as Ctx,
  NodeCtxSkip as CtxSkip,
  NodeBacktest as Backtest,
  NodeBacktestSummary as BacktestSummary,
  NodeSignal as Signal,
  NodeTimeframe as Timeframe,
  NodeTrade as Trade,
  NodeTradeDirection as TradeDirection,
  NodeTradeEvent as TradeEvent,
  returns,
  stdev,
  variance,
  mean,
  sum,
  f1,
  recall,
  precision,
  accuracy,
  shortNetProfitPct,
  longNetProfitPct,
  grossLossPct,
  grossProfitPct,
  netProfitPct,
  omegaRatioFromReturns,
  omegaRatio,
  sortinoRatioFromReturns,
  sortinoRatio,
  sharpeRatioFromReturns,
  sharpeRatio,
  avgWinLossRatio,
  avgLosingTrade,
  avgWinningTrade,
  avgTrade,
  winRate,
  longNetProfitRatio,
  profitFactor,
  pnl,
  expectancyScore,
  expectancy,
  orderSizeForEquityPct,
  orderSize,
  validateContracts,
  roundContracts,
  roundToMinTick,
  zipOhlcvBars,
} from "../../core/pkg_napi";
export { Client, ClientConfig } from "./client";

import { _getCoreVersion } from "../../core/pkg_napi";
import { version } from "../../package.json";
export const VERSION = version;
export const CORE_VERSION = _getCoreVersion();

// export const VERSION = "1.0.0";
// export const CORE_VERSION = "1.0.0";

// export const node = 1;
