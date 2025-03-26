import * as grpc from "@grpc/grpc-js";
import axios, { AxiosInstance, AxiosResponse } from "axios";

import {
  DEFAULT_GRPC_ENDPOINT,
  DEFAULT_REST_ENDPOINT,
  protoToOhlcvBar,
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
}

interface SymFilter {
  id?: string;
  tickerId?: string;
  timeframe?: qp.Timeframe | string;
}

type SymOpts = SymFilter & { limit?: number; offset?: number };

type OhlcvOpts = { limit?: number; offset?: number };

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
    const grpApiBase = this.config.grpcApiBase ?? DEFAULT_GRPC_ENDPOINT;
    this.http = axios.create({
      baseURL: apiBase,
      withCredentials: true,
      headers: {
        "Content-Type": "application/json",
        "x-api-key": this.config.apiKey,
        "x-info": JSON.stringify(this.clientInfo),
      },
    });
    const grpcCredentials = grpc.ChannelCredentials.createInsecure();
    const grpcOptions = {
      "grpc.max_receive_message_length": -1,
    };

    this.compilerClient = new CompilerApiClient(
      grpApiBase,
      grpcCredentials,
      grpcOptions,
    );
    this.ohlcvClient = new OhlcvApiClient(
      grpApiBase,
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
    if (!(sym instanceof qp.Sym)) {
      sym = await this.sym(sym);
    }
    const req = new ohlcvApi.GetRequest();
    if (sym.id == null) {
      throw new Error("Symbol has no id");
    }
    req.setSymId(sym.id);
    if (!(timeframe instanceof qp.Timeframe)) {
      timeframe = qp.Timeframe.fromString(timeframe);
    }
    req.setTimeframe(timeframe.toString());
    if (opts?.limit != null) req.setLimit(opts.limit);
    if (opts?.offset != null) req.setOffset(opts.offset);
    const res = await new Promise<ohlcvApi.GetResponse>((_resolve, _reject) => {
      this.ohlcvClient.get(req, this.createGrpcMetadata(), (err, res) => {
        if (err) return _reject(err);
        _resolve(res);
      });
    });
    const ohlcv = qp.Ohlcv.fromBars(res.getBarsList().map(protoToOhlcvBar));
    ohlcv.timeframe = timeframe;
    return ohlcv;
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
