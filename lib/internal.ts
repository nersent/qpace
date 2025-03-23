import { Empty } from "google-protobuf/google/protobuf/empty_pb";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";
// import { minimatch } from "minimatch";
import * as pl from "nodejs-polars";

import * as ohlcvApi from "./proto/ohlcv_pb";
import * as symApi from "./proto/sym_pb";

import * as qp from "./";

export const ENV_REST_ENDPOINT = "QPACE_API_BASE";
export const ENV_GRPC_ENDPOINT = "QPACE_GRPC_API_BASE";
export const ENV_API_KEY = "QPACE_API_KEY";
export const ENV_TELEMETRY = "QPACE_TELEMETRY";

export const DEFAULT_REST_ENDPOINT = `https://api.qpace.dev/v1`;
export const DEFAULT_GRPC_ENDPOINT = `https://api.qpace.dev/grpc`;
// export const DEFAULT_REST_ENDPOINT = `http://0.0.0.0:3000/v1`;
// export const DEFAULT_GRPC_ENDPOINT = `0.0.0.0:3001`;

// cross-env QPACE_API_KEY="sk_b6fc26f0-d900-4fb0-8fc1-d83abdf1837f" QPACE_REST_ENDPOINT="http://0.0.0.0:3000/v1" QPACE_GRPC_ENDPOINT="0.0.0.0:3001" pnpm bazed run //lib:cli -- -- -- sym --list
// cross-env QPACE_API_KEY="sk_b6fc26f0-d900-4fb0-8fc1-d83abdf1837f" QPACE_REST_ENDPOINT="http://0.0.0.0:3000/v1" QPACE_GRPC_ENDPOINT="0.0.0.0:3001" pnpm bazed run //lib:cli -- -- -- build --target python --cwd C:\projects\nersent\qpace-pine-example

export interface GetTeamMeRequest {}

export interface GetTeamMeResponse {
  team: {
    id: string;
    name: string;
  };
}

export interface SymFilter {
  id?: string;
  tickerId?: string;
  timeframe?: qp.Timeframe;
}
export type SymQuery = SymFilter & { limit?: number; offset?: number };

export const symFilterToProto = (filter: SymFilter): symApi.Filter => {
  const proto = new symApi.Filter();
  if (filter.id != null) proto.setId(filter.id);
  if (filter.tickerId != null) proto.setTickerIdPat(filter.tickerId);
  if (filter.timeframe != null) {
    proto.setTimeframe(filter.timeframe?.toString());
  }
  return proto;
};

export const symQueryToProto = (query: SymQuery): symApi.Query => {
  const proto = new symApi.Query();
  proto.setFilter(symFilterToProto(query));
  if (query.limit != null) proto.setLimit(query.limit);
  if (query.offset != null) proto.setOffset(query.offset);
  return proto;
};

export const protoToSym = (proto: symApi.Sym): qp.Sym => {
  const sym = new qp.Sym();
  sym.id = proto.getId()!;
  if (proto.hasTickerId()) sym.tickerId = proto.getTickerId()!;
  if (proto.hasPrefix()) sym.prefix = proto.getPrefix()!;
  if (proto.hasCurrency()) sym.currency = proto.getCurrency()!;
  if (proto.hasBaseCurrency()) sym.baseCurrency = proto.getBaseCurrency()!;
  if (proto.hasTicker()) sym.ticker = proto.getTicker()!;
  if (proto.hasCountry()) sym.country = proto.getCountry()!;
  if (proto.hasMinTick()) sym.minTick = proto.getMinTick()!;
  if (proto.hasMinQty()) sym.minQty = proto.getMinQty()!;
  if (proto.hasPriceScale()) sym.priceScale = proto.getPriceScale()!;
  if (proto.hasPointValue()) sym.pointValue = proto.getPointValue()!;
  sym.icons.push(...proto.getIconsList().map(protoToSymIcon));
  return sym;
};

export const symToProto = (sym: qp.Sym): symApi.Sym => {
  const proto = new symApi.Sym();
  if (sym.id == null) throw new Error("symbol id is required");
  proto.setId(sym.id);
  if (sym.tickerId != null) proto.setTickerId(sym.tickerId);
  if (sym.prefix != null) proto.setPrefix(sym.prefix);
  if (sym.currency != null) proto.setCurrency(sym.currency);
  if (sym.baseCurrency != null) proto.setBaseCurrency(sym.baseCurrency);
  if (sym.ticker != null) proto.setTicker(sym.ticker);
  if (sym.country != null) proto.setCountry(sym.country);
  if (sym.minTick != null) proto.setMinTick(sym.minTick);
  if (sym.minQty != null) proto.setMinQty(sym.minQty);
  if (sym.priceScale != null) proto.setPriceScale(sym.priceScale);
  if (sym.pointValue != null) proto.setPointValue(sym.pointValue);
  proto.setIconsList(sym.icons.map(symIconToProto));
  return proto;
};

export const protoToSymIcon = (proto: symApi.Icon): qp.SymIcon => {
  const icon = new qp.SymIcon();
  icon.url = proto.getUrl();
  icon.mimeType = proto.getMimeType();
  return icon;
};

export const symIconToProto = (icon: qp.SymIcon): symApi.Icon => {
  const proto = new symApi.Icon();
  proto.setUrl(icon.url);
  proto.setMimeType(icon.mimeType);
  return proto;
};

// export const matchSym = (
//   sym: qp.Sym,
//   query: SymQuery,
//   allowGlob = false,
// ): boolean => {
//   const { id, tickerId } = sym;
//   if (query.id != null) {
//     if (id == null) return false;
//     if (!allowGlob && sym.id !== id) return false;
//     if (allowGlob && !minimatch(id, query.id)) return false;
//   }
//   if (query.tickerId != null) {
//     if (tickerId == null) return false;
//     if (!allowGlob && sym.tickerId !== tickerId) return false;
//     if (allowGlob && !minimatch(tickerId, query.tickerId)) return false;
//   }
//   return true;
// };

// export const timeframeToProto = (timeframe: qp.Timeframe): symApi.Timeframe => {
//   if (timeframe.unknown) {
//     return new symApi.Timeframe().setUnknown(new Empty());
//   }
//   if (timeframe.years != null) {
//     return new symApi.Timeframe().setYears(timeframe.years);
//   }
//   if (timeframe.months != null) {
//     return new symApi.Timeframe().setMonths(timeframe.months);
//   }
//   if (timeframe.days != null) {
//     return new symApi.Timeframe().setDays(timeframe.days);
//   }
//   if (timeframe.hours != null) {
//     return new symApi.Timeframe().setHours(timeframe.hours);
//   }
//   if (timeframe.minutes != null) {
//     return new symApi.Timeframe().setMinutes(timeframe.minutes);
//   }
//   if (timeframe.seconds != null) {
//     return new symApi.Timeframe().setSeconds(timeframe.seconds);
//   }
//   if (timeframe.ranges != null) {
//     return new symApi.Timeframe().setRanges(timeframe.ranges);
//   }
//   if (timeframe.ticks != null) {
//     return new symApi.Timeframe().setTicks(timeframe.ticks);
//   }
//   throw new Error(`Cannot map qp.Timeframe to proto: ${timeframe}`);
// };

// export const protoToTimeframe = (proto: symApi.Timeframe): qp.Timeframe => {
//   if (proto.hasUnknown()) {
//     return qp.Timeframe.unknown();
//   }
//   if (proto.hasYears()) {
//     return qp.Timeframe.years(proto.getYears());
//   }
//   if (proto.hasMonths()) {
//     return qp.Timeframe.months(proto.getMonths());
//   }
//   if (proto.hasDays()) {
//     return qp.Timeframe.days(proto.getDays());
//   }
//   if (proto.hasHours()) {
//     return qp.Timeframe.hours(proto.getHours());
//   }
//   if (proto.hasMinutes()) {
//     return qp.Timeframe.minutes(proto.getMinutes());
//   }
//   if (proto.hasSeconds()) {
//     return qp.Timeframe.seconds(proto.getSeconds());
//   }
//   if (proto.hasRanges()) {
//     return qp.Timeframe.ranges(proto.getRanges());
//   }
//   if (proto.hasTicks()) {
//     return qp.Timeframe.ticks(proto.getTicks());
//   }
//   throw new Error(`Cannot map proto to qp.Timeframe: ${proto}`);
// };

export const getSymId = (x: string | qp.Sym): string => {
  if (typeof x === "string") return x;
  if (x.id != null) {
    return x.id;
  }
  throw new Error(`Cannot get symbol id from ${x}`);
};

export interface OhlcvFilter {
  sym: string | qp.Sym;
  timeframe?: qp.Timeframe;
}
export type OhlcvQuery = OhlcvFilter & { limit?: number; offset?: number };

export const ohlcvFilterToProto = (filter: OhlcvFilter): ohlcvApi.Filter => {
  const proto = new ohlcvApi.Filter();
  proto.setSymId(getSymId(filter.sym));
  if (filter.timeframe != null) {
    proto.setTimeframe(filter.timeframe.toString());
  }
  return proto;
};

const readDf = async (path: string): Promise<pl.DataFrame> => {
  if (path.endsWith(".csv")) return pl.readCSV(path);
  if (path.endsWith(".parquet")) return pl.readParquet(path);
  throw new Error(`Unsupported file format: ${path}`);
};

const writeDf = async (path: string, df: pl.DataFrame): Promise<void> => {
  if (path.endsWith(".csv")) return df.writeCSV(path);
  if (path.endsWith(".parquet")) return df.writeParquet(path);
};

export const readOhlcvBarsFromPath = async (
  path: string,
): Promise<qp.OhlcvBar[]> => {
  const df = await readDf(path);
  const _openTime = df.getColumn("open_time");
  const _closeTime = df.getColumn("close_time");
  const _open = df.getColumn("open");
  const _high = df.getColumn("high");
  const _low = df.getColumn("low");
  const _close = df.getColumn("close");
  const _volume = df.getColumn("volume");
  const openTime: Date[] = Array.from(_openTime.values()).map(
    (r) => new Date(r),
  );
  const closeTime: Date[] = Array.from(_closeTime.values()).map(
    (r) => new Date(r),
  );
  const open = Float64Array.from(_open.values());
  const high = Float64Array.from(_high.values());
  const low = Float64Array.from(_low.values());
  const close = Float64Array.from(_close.values());
  const volume = Float64Array.from(_volume.values());
  return qp.zipOhlcvBars(openTime, closeTime, open, high, low, close, volume);
};

export const writeOhlcvBarsToPath = async (
  path: string,
  bars: qp.OhlcvBar[],
): Promise<void> => {
  const openTime = bars.map((r) => r.openTime);
  const closeTime = bars.map((r) => r.closeTime);
  const open = bars.map((r) => r.open);
  const high = bars.map((r) => r.high);
  const low = bars.map((r) => r.low);
  const close = bars.map((r) => r.close);
  const volume = bars.map((r) => r.volume);
  const df = pl.DataFrame({
    open_time: openTime,
    close_time: closeTime,
    open: open,
    high: high,
    low: low,
    close: close,
    volume: volume,
  });
  await writeDf(path, df);
};

export const ohlcvBarToProto = (bar: qp.OhlcvBar): ohlcvApi.OhlcvBar => {
  const proto = new ohlcvApi.OhlcvBar();
  proto.setOpenTime(Timestamp.fromDate(bar.openTime));
  proto.setCloseTime(Timestamp.fromDate(bar.closeTime));
  proto.setOpen(bar.open);
  proto.setClose(bar.close);
  proto.setHigh(bar.high);
  proto.setLow(bar.low);
  proto.setVolume(bar.volume);
  return proto;
};

export const protoToOhlcvBar = (proto: ohlcvApi.OhlcvBar): qp.OhlcvBar => {
  return new qp.OhlcvBar(
    proto.getOpenTime()!.toDate(),
    proto.getCloseTime()!.toDate(),
    proto.getOpen(),
    proto.getHigh(),
    proto.getLow(),
    proto.getClose(),
    proto.getVolume(),
  );
};

/*
  We use telemetry to collect information about the client usage and performance.
  This information is used to improve the client and the qpace platform.
  We don't collect any personal information or sensitive data.

  You can opt-out of telemetry by setting the QPACE_TELEMETRY environment variable to false or by using `qpc telemetry disable` command.
*/
export interface ClientTelemetry {
  qpaceVersion?: string;
  qpaceCoreVersion?: string;
  deviceId?: string;
  nodeVersion?: string;
  npmVersion?: string;
  yarnVersion?: string;
  pnpmVersion?: string;
  goVersion?: string;
  pythonVersion?: string;
  python3Version?: string;
  pip3Version?: string;
  pipVersion?: string;
  rustVersion?: string;
  cargoVersion?: string;
  os?: string;
  arch?: string;
  platform?: string;
  cpu?: string;
  memory?: string;
}
