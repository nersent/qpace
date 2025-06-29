export {
  Ohlcv,
  OhlcvBar,
  Sym,
  SymKind,
  Ctx,
  CtxSkip,
  Backtest,
  Signal,
  Timeframe,
  Trade,
  TradeDirection,
  TradeEvent,
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
} from "../../core/pkg/qpace_core";
import init from "../../core/pkg/qpace_core";
export { init };

import { version, coreVersion } from "../../package.json";
export const VERSION = version;
export const CORE_VERSION = coreVersion;

import * as _ta from "../../content/web/ta";
import { init as initTa } from "../../content/web";
export const ta = { ..._ta, init: initTa } as typeof _ta & {
  init: typeof initTa;
};
