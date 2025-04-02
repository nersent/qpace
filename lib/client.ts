import * as grpc from "@grpc/grpc-js";
import axios, { AxiosInstance, AxiosResponse } from "axios";

import {
  createLazyTqdmStepper,
  createTqdm,
  createTqdmStepper,
  DEFAULT_GRPC_ENDPOINT,
  DEFAULT_REST_ENDPOINT,
  protoToOhlcvBar,
  TqdmStepper,
} from "./internal";
import { CompilerApiClient } from "./proto/compiler_grpc_pb";
import { OhlcvApiClient } from "./proto/ohlcv_grpc_pb";
import * as ohlcvApi from "./proto/ohlcv_pb";

import * as qp from "./";

export interface ClientConfig {
  apiBase?: string;
  grpcApiBase?: string;
  apiKey: string;
  timeout?: number;
  grpcCredentials?: grpc.ChannelCredentials;
}

interface SymFilter {
  id?: string;
  tickerId?: string;
  timeframe?: qp.Timeframe | string;
}

type SymOpts = SymFilter & { limit?: number; offset?: number };

type Order = "asc" | "desc";

type OhlcvOpts = {
  limit?: number;
  offset?: number;
  pb?: boolean;
  order?: Order;
};

export class Client {
  private http!: AxiosInstance;
  private compilerClient!: CompilerApiClient;
  private ohlcvClient!: OhlcvApiClient;
  private _grpcMetadata: grpc.Metadata | undefined;
  private clientInfo?: Record<string, any>;

  constructor(private readonly config: ClientConfig) {
    this.clientInfo ??= {};
    this.clientInfo["qpace"] ??= qp.getVersion();
    this.clientInfo["qpaceCore"] ??= qp.getCoreVersion();
    this.clientInfo["node"] ??= process.versions.node;
    this.init();
  }

  private init(): void {
    const apiBase = this.config.apiBase ?? DEFAULT_REST_ENDPOINT;
    const grpcApiBase = this.config.grpcApiBase ?? DEFAULT_GRPC_ENDPOINT;
    this.http = axios.create({
      baseURL: apiBase,
      withCredentials: true,
      headers: {
        "Content-Type": "application/json",
        "x-api-key": this.config.apiKey,
        "x-info": JSON.stringify(this.clientInfo),
      },
    });
    const grpcCredentials =
      this.config.grpcCredentials ??
      (grpcApiBase.startsWith("0.0.0.0")
        ? grpc.ChannelCredentials.createInsecure()
        : grpc.ChannelCredentials.createSsl());
    const grpcOptions: grpc.ClientOptions = {
      "grpc.max_receive_message_length": -1,
      "grpc.max_send_message_length": -1,
    };

    this.compilerClient = new CompilerApiClient(
      grpcApiBase,
      grpcCredentials,
      grpcOptions,
    );
    this.ohlcvClient = new OhlcvApiClient(
      grpcApiBase,
      grpcCredentials,
      grpcOptions,
    );
  }

  private createGrpcMetadata(): grpc.Metadata {
    if (this._grpcMetadata == null) {
      const metadata = new grpc.Metadata();
      metadata.set("x-api-key", `${this.config.apiKey}`);
      if (this.clientInfo != null) {
        metadata.set("x-info", JSON.stringify(this.clientInfo));
      }
      this._grpcMetadata = metadata;
    }
    return this._grpcMetadata.clone();
  }

  public async sym(filter: SymFilter | string): Promise<qp.Sym> {
    let query: SymOpts = { limit: 1 };
    if (typeof filter === "string") {
      query = { ...query, id: filter, tickerId: filter };
    } else {
      query = { ...query, ...filter };
    }
    const syms = await this.syms(query);
    if (syms.length === 0) {
      throw new Error("Symbol not found");
    }
    return syms[0];
  }

  public async syms(q: SymOpts = {}): Promise<qp.Sym[]> {
    const timeframe = q?.timeframe?.toString();
    const { data } = await this.http.get(`/symbols`, {
      params: {
        id_pat: q.id,
        ticker_id_pat: q.tickerId,
        limit: q?.limit,
        offset: q?.offset,
        timeframe,
      },
    });
    return data["symbols"].map((r: any) => qp.Sym.fromJSON(r));
  }

  public async ohlcv(
    sym: SymFilter | string | qp.Sym,
    timeframe: qp.Timeframe | string,
    opts?: OhlcvOpts,
  ): Promise<qp.Ohlcv> {
    let offset = opts?.offset ?? 0;
    if (!(timeframe instanceof qp.Timeframe)) {
      timeframe = qp.Timeframe.fromString(timeframe);
    }
    let pb: TqdmStepper | undefined;
    const _bars: qp.OhlcvBar[] = [];
    while (true) {
      const res = await this._ohlcv(sym, timeframe, { ...opts, offset });
      const bars = res.getBarsList().map(protoToOhlcvBar);
      const remaining = res.getRemaining();
      _bars.push(...bars);
      offset += bars.length;
      if (remaining === 0 || opts?.limit != null) break;
      if (opts?.pb) {
        pb ??= createTqdmStepper(remaining + _bars.length);
        pb?.setProgress(_bars.length);
      }
    }
    pb?.stop();
    const ohlcv = qp.Ohlcv.fromBars(_bars);
    ohlcv.timeframe = timeframe;
    return ohlcv;
  }

  private async _ohlcv(
    sym: SymFilter | string | qp.Sym,
    timeframe: qp.Timeframe | string,
    opts?: OhlcvOpts,
  ): Promise<ohlcvApi.GetResponse> {
    if (!(sym instanceof qp.Sym)) {
      sym = await this.sym(sym);
    }
    const req = new ohlcvApi.GetRequest();
    if (sym.id == null) {
      throw new Error("Symbol has no id");
    }
    req.setSymId(sym.id);
    req.setTimeframe(timeframe.toString());
    if (opts?.limit != null) req.setLimit(opts.limit);
    if (opts?.offset != null) req.setOffset(opts.offset);
    if (opts?.order === "asc") {
      req.setOrder(ohlcvApi.Order.ASC);
    } else if (opts?.order === "desc") {
      req.setOrder(ohlcvApi.Order.DESC);
    }
    const res = await new Promise<ohlcvApi.GetResponse>((_resolve, _reject) => {
      this.ohlcvClient.get(req, this.createGrpcMetadata(), (err, res) => {
        if (err) return _reject(err);
        _resolve(res);
      });
    });
    return res;
  }

  public async ctx(
    sym: SymFilter | string | qp.Sym,
    timeframe: qp.Timeframe | string,
    opts?: OhlcvOpts,
  ): Promise<qp.Ctx> {
    let _sym: qp.Sym;
    if (sym instanceof qp.Sym) {
      _sym = sym;
    } else {
      _sym = await this.sym(sym);
    }
    const ohlcv = await this.ohlcv(sym, timeframe, opts);
    return new qp.Ctx(ohlcv, _sym);
  }
}
