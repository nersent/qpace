import { Empty } from "google-protobuf/google/protobuf/empty_pb";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";
import { minimatch } from "minimatch";
import * as pl from "nodejs-polars";

import * as ohlcvApi from "./proto/ohlcv_pb";
import * as symApi from "./proto/sym_pb";

import * as qp from "./";

export const ENV_REST_ENDPOINT = "QPACE_REST_ENDPOINT";
export const ENV_GRPC_ENDPOINT = "QPACE_GRPC_ENDPOINT";
export const ENV_API_KEY = "QPACE_API_KEY";

export const LOCALHOST_REST_ENDPOINT = "http://0.0.0.0:3000/v1";
export const LOCALHOST_GRPC_ENDPOINT = "0.0.0.0:3001";

export const DEFAULT_REST_ENDPOINT = `https://api.qpace.dev/v1`;
export const DEFAULT_GRPC_ENDPOINT = `https://grpc.qpace.dev`;

// cross-env QPACE_API_KEY="sk_b6fc26f0-d900-4fb0-8fc1-d83abdf1837f" QPACE_REST_ENDPOINT="http://0.0.0.0:3000/v1" QPACE_GRPC_ENDPOINT="0.0.0.0:3001" pnpm bazed run //lib:cli -- -- -- sym --list
// cross-env QPACE_API_KEY="sk_b6fc26f0-d900-4fb0-8fc1-d83abdf1837f" QPACE_REST_ENDPOINT="http://0.0.0.0:3000/v1" QPACE_GRPC_ENDPOINT="0.0.0.0:3001" pnpm bazed run //lib:cli -- -- -- build --target python --cwd C:\projects\nersent\qpace-pine-example

export interface SymQuery {
  id?: string;
  tickerId?: string;
}

export const symQueryToProto = (query: SymQuery): symApi.GetQuery => {
  const proto = new symApi.GetQuery();
  if (query.id != null) proto.setId(query.id);
  if (query.tickerId != null) proto.setTickerId(query.tickerId);
  return proto;
};

export const protoToSymQuery = (proto: symApi.GetQuery): SymQuery => {
  const query: SymQuery = {};
  if (proto.hasId()) query.id = proto.getId();
  if (proto.hasTickerId()) query.tickerId = proto.getTickerId();
  return query;
};

export const validateSymQuery = (query: SymQuery): void => {
  if (query.id == null && query.tickerId == null) {
    throw new Error("id or tickerId is required");
  }
};

export const protoToQpSym = (proto: symApi.Sym): qp.Sym => {
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
  // if (proto.hasPriceScale()) sym.priceScale = proto.getPriceScale()!;
  // if (proto.hasPointValue()) sym.pointValue = proto.getPointValue()!;
  sym.icons.push(...proto.getIconsList().map(protoToQpSymIcon));
  return sym;
};

export const qpSymToProto = (sym: qp.Sym): symApi.Sym => {
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
  // if (sym.priceScale != null) proto.setPriceScale(sym.priceScale);
  // if (sym.pointValue != null) proto.setPointValue(sym.pointValue);
  proto.setIconsList(sym.icons.map(qpSymIconToProto));
  return proto;
};

export const protoToQpSymIcon = (proto: symApi.Icon): qp.SymIcon => {
  const icon = new qp.SymIcon();
  icon.url = proto.getUrl();
  icon.mimeType = proto.getMimeType();
  return icon;
};

export const qpSymIconToProto = (icon: qp.SymIcon): symApi.Icon => {
  const proto = new symApi.Icon();
  proto.setUrl(icon.url);
  proto.setMimeType(icon.mimeType);
  return proto;
};

export const matchSym = (
  sym: qp.Sym,
  query: SymQuery,
  allowGlob = false,
): boolean => {
  const { id, tickerId } = query;
  if (query.id != null) {
    if (id == null) return false;
    if (!allowGlob && sym.id !== id) return false;
    if (allowGlob && !minimatch(id, id)) return false;
  }
  if (query.tickerId != null) {
    if (tickerId == null) return false;
    if (!allowGlob && sym.tickerId !== tickerId) return false;
    if (allowGlob && !minimatch(tickerId, tickerId)) return false;
  }
  return true;
};

export const qpTimeframeToProto = (
  timeframe: qp.Timeframe,
): ohlcvApi.Timeframe => {
  if (timeframe.unknown) {
    return new ohlcvApi.Timeframe().setUnknown(new Empty());
  }
  if (timeframe.years != null) {
    return new ohlcvApi.Timeframe().setYears(timeframe.years);
  }
  if (timeframe.months != null) {
    return new ohlcvApi.Timeframe().setMonths(timeframe.months);
  }
  if (timeframe.days != null) {
    return new ohlcvApi.Timeframe().setDays(timeframe.days);
  }
  if (timeframe.hours != null) {
    return new ohlcvApi.Timeframe().setHours(timeframe.hours);
  }
  if (timeframe.minutes != null) {
    return new ohlcvApi.Timeframe().setMinutes(timeframe.minutes);
  }
  if (timeframe.seconds != null) {
    return new ohlcvApi.Timeframe().setSeconds(timeframe.seconds);
  }
  if (timeframe.ranges != null) {
    return new ohlcvApi.Timeframe().setRanges(timeframe.ranges);
  }
  if (timeframe.ticks != null) {
    return new ohlcvApi.Timeframe().setTicks(timeframe.ticks);
  }
  throw new Error(`Cannot map qp timeframe to proto: ${timeframe}`);
};

export const protoToQpTimeframe = (proto: ohlcvApi.Timeframe): qp.Timeframe => {
  if (proto.hasUnknown()) {
    return qp.Timeframe.unknown();
  }
  if (proto.hasYears()) {
    return qp.Timeframe.years(proto.getYears());
  }
  if (proto.hasMonths()) {
    return qp.Timeframe.months(proto.getMonths());
  }
  if (proto.hasDays()) {
    return qp.Timeframe.days(proto.getDays());
  }
  if (proto.hasHours()) {
    return qp.Timeframe.hours(proto.getHours());
  }
  if (proto.hasMinutes()) {
    return qp.Timeframe.minutes(proto.getMinutes());
  }
  if (proto.hasSeconds()) {
    return qp.Timeframe.seconds(proto.getSeconds());
  }
  if (proto.hasRanges()) {
    return qp.Timeframe.ranges(proto.getRanges());
  }
  if (proto.hasTicks()) {
    return qp.Timeframe.ticks(proto.getTicks());
  }
  throw new Error(`Cannot map proto timeframe to qp: ${proto}`);
};

export interface OhlcvQuery {
  sym: string | qp.Sym;
  timeframe?: qp.Timeframe;
  limit?: number;
  offset?: number;
}

export const getSymId = (x: string | qp.Sym): string => {
  if (typeof x === "string") return x;
  const id = x.id;
  if (id == null) throw new Error("symbol id is required");
  return id;
};

export const ohlcvQueryToProto = (query: OhlcvQuery): ohlcvApi.GetQuery => {
  const proto = new ohlcvApi.GetQuery();
  proto.setSymId(getSymId(query.sym));
  if (query.limit != null) proto.setLimit(query.limit);
  if (query.offset != null) proto.setOffset(query.offset);
  if (query.timeframe != null) {
    proto.setTimeframe(qpTimeframeToProto(query.timeframe));
  }
  return proto;
};

export const protoToOhlcvQuery = (proto: ohlcvApi.GetQuery): OhlcvQuery => {
  return {
    sym: proto.getSymId(),
    limit: proto.hasLimit() ? proto.getLimit() : undefined,
    offset: proto.hasOffset() ? proto.getOffset() : undefined,
    timeframe: proto.hasTimeframe()
      ? protoToQpTimeframe(proto.getTimeframe()!)
      : undefined,
  };
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

// res.setBarsList([
//   new ohlcvApi.OhlcvBar()
//     .setOpen(1)
//     .setClose(2)
//     .setHigh(3)
//     .setLow(4)
//     .setVolume(5)
//     .setOpenTime(Timestamp.fromDate(new Date()))
//     .setCloseTime(Timestamp.fromDate(new Date())),
// ]);

export const qpOhlcvBarToProto = (bar: qp.OhlcvBar): ohlcvApi.OhlcvBar => {
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

export const protoToQpOhlcvBar = (proto: ohlcvApi.OhlcvBar): qp.OhlcvBar => {
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
