import * as grpc from "@grpc/grpc-js";
import * as cliProgress from "cli-progress";
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

export const DEFAULT_REST_ENDPOINT = `https://api.qpace.dev/v1`;
export const DEFAULT_GRPC_ENDPOINT = `grpc.qpace.dev`;
// export const DEFAULT_REST_ENDPOINT = `http://0.0.0.0:3000/v1`;
// export const DEFAULT_GRPC_ENDPOINT = `0.0.0.0:3001`;

export interface VerifyApiKeyRequest {}
export interface VerifyApiKeyResponse {
  user?: {
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

const writeDf = (
  path: string,
  df: pl.DataFrame,
  opts?: { compression?: "brotli" },
): void => {
  if (path.endsWith(".csv")) return df.writeCSV(path);
  if (path.endsWith(".parquet")) return df.writeParquet(path);
};

const getTimeSeries = (
  df: pl.DataFrame,
  colName: string | undefined,
  timeUnit: "s" | "ms",
): (Date | undefined)[] | undefined => {
  if (!colName || !df.columns.includes(colName)) return undefined;
  const col = df.getColumn(colName);
  return Array.from(col.values()).map((val: number | null) => {
    if (val == null) return undefined;
    return new Date(timeUnit === "s" ? val * 1000 : val);
  });
};

const getNumericSeries = (
  df: pl.DataFrame,
  colName: string,
): Float64Array | undefined => {
  if (!df.columns.includes(colName)) return undefined;
  return Float64Array.from(df.getColumn(colName).values());
};

const coerceDates = (
  arr: (Date | undefined)[] | undefined,
): Date[] | undefined => {
  if (!arr) return undefined; // stay undefined if the array itself is undefined
  return arr.map((d) => d ?? new Date(0)); // replace undefined with a default Date
};

export const readOhlcvBarsFromPath = (
  format: "csv" | "parquet",
  path: string,
  timeUnit: "s" | "ms" = "s",
  // opts?: { limit?: number; offset?: number },
): qp.OhlcvBar[] => {
  // 1) Read the file into a Polars DataFrame.
  const df = readDf(path);
  const cols = df.columns;

  // 2) Figure out which column name to use for open_time, close_time.
  //    If "open_time" is missing, fall back to "time."
  const openTimeCol = cols.includes("open_time")
    ? "open_time"
    : cols.includes("time")
    ? "time"
    : undefined;
  const closeTimeCol = cols.includes("close_time")
    ? "close_time"
    : cols.includes("time")
    ? "time"
    : undefined;

  // 3) Extract numeric columns for open/high/low/close/volume.
  //    The Rust version uses unwrap(), which panics on missing columns
  //    for open/high/low/close. You can throw if you need them mandatory:
  if (!cols.includes("open")) {
    throw new Error('Missing required "open" column');
  }
  if (!cols.includes("high")) {
    throw new Error('Missing required "high" column');
  }
  if (!cols.includes("low")) {
    throw new Error('Missing required "low" column');
  }
  if (!cols.includes("close")) {
    throw new Error('Missing required "close" column');
  }

  const openSeries = getNumericSeries(df, "open") ?? new Float64Array(0);
  const highSeries = getNumericSeries(df, "high") ?? new Float64Array(0);
  const lowSeries = getNumericSeries(df, "low") ?? new Float64Array(0);
  const closeSeries = getNumericSeries(df, "close") ?? new Float64Array(0);

  // volume is optional; if missing, pass an empty array or Float64Array(…).
  const volumeSeries =
    getNumericSeries(df, "volume") ?? new Float64Array(openSeries.length);

  // 4) Convert time columns to arrays of Date objects, or undefined if absent.
  const openTime = getTimeSeries(df, openTimeCol, timeUnit);
  const closeTime = getTimeSeries(df, closeTimeCol, timeUnit);

  // 5) Finally, zip them into an array of OhlcvBar-compatible objects.
  //    If openTime / closeTime are undefined, your zip function should handle that
  //    (like the Rust code does), or you can default them to a large array of “oldest” dates.
  return qp.zipOhlcvBars(
    coerceDates(openTime),
    coerceDates(closeTime),
    openSeries,
    highSeries,
    lowSeries,
    closeSeries,
    volumeSeries,
  );
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

export type TqdmStepper = {
  setTotal: (total: number) => void;
  step: () => void;
  stop: () => void;
  setProgress: (progress: number) => void;
};

export const createTqdmStepper = (
  total: number,
  stopIfTotal = false,
): TqdmStepper => {
  let progress = 0;
  let lastProgress = 0;

  const bar = new cliProgress.Bar(
    {
      format: " {bar} {percentage}% | ETA: {eta_formatted} | {value}/{total}",
      etaBuffer: 100,
    },
    cliProgress.Presets.shades_classic,
  );
  bar.start(total, progress, { eta: 0 });

  const updateBar = (): void => {
    bar.update(progress);
    lastProgress = progress;

    if (stopIfTotal && progress === total) {
      stop();
    }
  };

  const stop = (): void => {
    bar.stop();
  };

  return {
    setTotal: (newTotal: number): void => {
      total = newTotal;
      bar.setTotal(total);
    },
    stop,
    step: (): void => {
      progress++;
      updateBar();
    },
    setProgress: (prog: number): void => {
      progress = prog;
      updateBar();
    },
  };
};

// eslint-disable-next-line @typescript-eslint/explicit-function-return-type
export const createTqdm = (total: number) => {
  const stepper = createTqdmStepper(total, true);

  return <T, A extends any[]>(cb: (...args: A) => T) => {
    return (...args: A): T => {
      const res = cb(...args);
      if (res instanceof Promise) {
        return new Promise((resolve, reject) => {
          res
            .then((x) => {
              stepper.step();
              resolve(x);
            })
            .catch(reject);
        }) as any;
      }
      stepper.step();
      return res;
    };
  };
};

export function* tqdm<T>(arr: T[]): Iterable<T> {
  const stepper = createTqdmStepper(arr.length, true);

  for (let i = 0; i < arr.length; i++) {
    yield arr[i];
    stepper.step();
  }
}

export const createLazyTqdmStepper = (): ((remaining: number) => void) => {
  let pb: TqdmStepper | undefined;

  return (remaining: number) => {
    pb ??= createTqdmStepper(remaining);
    pb.step();
  };
};

// eslint-disable-next-line @typescript-eslint/explicit-function-return-type
// export const getInfo = async ({
//   compilerClient,
//   metadata,
// }: {
//   compilerClient: CompilerApiClient;
//   metadata: grpc.Metadata;
// }) => {
//   const req = new compilerApi.VersionRequest();
//   const res = await new Promise<compilerApi.VersionResponse>(
//     (_resolve, _reject) => {
//       compilerClient.version(req, metadata, (err, res) => {
//         if (err) return _reject(err);
//         _resolve(res);
//       });
//     },
//   );
//   return {
//     qpace: qp.getVersion(),
//     qpaceCore: qp.getCoreVersion(),
//     compiler: res.getVersion(),
//     compilerBuildTime: res.getBuildTime()?.toDate(),
//   };
// };
