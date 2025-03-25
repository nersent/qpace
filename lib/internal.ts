import { Empty } from "google-protobuf/google/protobuf/empty_pb";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";
// import { minimatch } from "minimatch";
import * as pl from "nodejs-polars";

import * as ohlcvApi from "./proto/ohlcv_pb";

import * as qp from "./";

export { ChannelCredentials, Metadata } from "@grpc/grpc-js";

export const ENV_REST_ENDPOINT = "QPACE_API_BASE";
export const ENV_GRPC_ENDPOINT = "QPACE_GRPC_API_BASE";
export const ENV_API_KEY = "QPACE_API_KEY";
export const ENV_TELEMETRY = "QPACE_TELEMETRY";

// export const DEFAULT_REST_ENDPOINT = `https://api.qpace.dev/v1`;
// export const DEFAULT_GRPC_ENDPOINT = `https://api.qpace.dev/grpc`;
export const DEFAULT_REST_ENDPOINT = `http://0.0.0.0:3000/v1`;
export const DEFAULT_GRPC_ENDPOINT = `0.0.0.0:3001`;

// cross-env QPACE_API_KEY="sk_b6fc26f0-d900-4fb0-8fc1-d83abdf1837f" QPACE_REST_ENDPOINT="http://0.0.0.0:3000/v1" QPACE_GRPC_ENDPOINT="0.0.0.0:3001" pnpm bazed run //lib:cli -- -- -- sym --list
// cross-env QPACE_API_KEY="sk_b6fc26f0-d900-4fb0-8fc1-d83abdf1837f" QPACE_REST_ENDPOINT="http://0.0.0.0:3000/v1" QPACE_GRPC_ENDPOINT="0.0.0.0:3001" pnpm bazed run //lib:cli -- -- -- build --target python --cwd C:\projects\nersent\qpace-pine-example

export interface VerifyApiKeyRequest {}
export interface VerifyApiKeyResponse {
  team?: {
    id: string;
    name: string;
  };
}

export interface User {
  id: string;
  name?: string;
  email?: string;
  teams?: { id: string; name: string }[];
}

export type MeUser = User & Required<Pick<User, "name" | "teams">>;

export const getSymId = (x: string | qp.Sym): string => {
  if (typeof x === "string") return x;
  if (x.id != null) {
    return x.id;
  }
  throw new Error(`Cannot get symbol id from ${x}`);
};

const readDf = (path: string): pl.DataFrame => {
  if (path.endsWith(".csv")) return pl.readCSV(path);
  if (path.endsWith(".parquet")) return pl.readParquet(path);
  throw new Error(`Unsupported file format: ${path}`);
};

const writeDf = (path: string, df: pl.DataFrame): void => {
  if (path.endsWith(".csv")) return df.writeCSV(path);
  if (path.endsWith(".parquet")) return df.writeParquet(path);
};

export const readOhlcvBarsFromPath = (
  format: "csv" | "parquet",
  path: string,
  timeUnit: string,
  // opts?: { limit?: number; offset?: number },
): qp.OhlcvBar[] => {
  let df: pl.DataFrame;
  df = readDf(path);
  // if (opts == null) {
  //   df = readDf(path);
  // } else {
  //   if (path.endsWith(".csv")) {
  //     df = pl.readCSV(path, { startRows: opts?.offset, nRows: opts?.limit });
  //   } else if (path.endsWith(".parquet")) {
  //     df = pl.readParquet(path, {

  //     });
  //   }
  // }
  const _openTime = df.getColumn("open_time");
  const _closeTime = df.getColumn("close_time");
  const _open = df.getColumn("open");
  const _high = df.getColumn("high");
  const _low = df.getColumn("low");
  const _close = df.getColumn("close");
  const _volume = df.getColumn("volume");
  const openTime: Date[] = Array.from(_openTime.values()).map(
    (r) => new Date(timeUnit === "s" ? r * 1000 : r),
  );
  const closeTime: Date[] = Array.from(_closeTime.values()).map(
    (r) => new Date(timeUnit === "s" ? r * 1000 : r),
  );
  const open = Float64Array.from(_open.values());
  const high = Float64Array.from(_high.values());
  const low = Float64Array.from(_low.values());
  const close = Float64Array.from(_close.values());
  const volume = Float64Array.from(_volume.values());
  return qp.zipOhlcvBars(openTime, closeTime, open, high, low, close, volume);
};

export const writeOhlcvBarsToPath = (
  format: "csv" | "parquet",
  path: string,
  bars: qp.OhlcvBar[],
): void => {
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
  writeDf(path, df);
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
