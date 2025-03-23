import * as grpc from "@grpc/grpc-js";
import axios, { AxiosInstance, AxiosResponse } from "axios";

import {
  ClientTelemetry,
  DEFAULT_GRPC_ENDPOINT,
  DEFAULT_REST_ENDPOINT,
  GetTeamMeRequest,
  GetTeamMeResponse,
  OhlcvFilter,
  ohlcvFilterToProto,
  OhlcvQuery,
  protoToOhlcvBar,
  protoToSym,
  SymFilter,
  symFilterToProto,
  SymQuery,
  symQueryToProto,
} from "./internal";
import { CompilerApiClient } from "./proto/compiler_grpc_pb";
import { OhlcvApiClient } from "./proto/ohlcv_grpc_pb";
import * as ohlcvApi from "./proto/ohlcv_pb";
import { SymApiClient } from "./proto/sym_grpc_pb";
import * as symApi from "./proto/sym_pb";

import * as qp from "./";

export interface ClientConfig {
  apiBase?: string;
  grpcApiBase?: string;
  apiKey: string;
  timeout?: number;
}

export class Client {
  public readonly http: AxiosInstance;
  public readonly compilerClient: CompilerApiClient;
  public readonly symClient: SymApiClient;
  public readonly ohlcvClient: OhlcvApiClient;
  private _grpcMetadata: grpc.Metadata | undefined;
  private telemetry?: ClientTelemetry;

  constructor(private readonly config: ClientConfig) {
    const apiBase = config.apiBase ?? DEFAULT_REST_ENDPOINT;
    const grpApiBase = config.grpcApiBase ?? DEFAULT_GRPC_ENDPOINT;
    const grpcCredentials = grpc.ChannelCredentials.createInsecure();
    const grpcOptions = {
      "grpc.max_receive_message_length": -1,
    };

    this.http = axios.create({
      baseURL: apiBase,
      withCredentials: true,
      headers: {
        "Content-Type": "application/json",
        "x-api-key": this.config.apiKey,
      },
    });

    this.compilerClient = new CompilerApiClient(
      grpApiBase,
      grpcCredentials,
      grpcOptions,
    );
    this.symClient = new SymApiClient(grpApiBase, grpcCredentials, grpcOptions);
    this.ohlcvClient = new OhlcvApiClient(
      grpApiBase,
      grpcCredentials,
      grpcOptions,
    );

    this.telemetry ??= {};
    this.telemetry.qpaceCoreVersion ??= qp.getVersion();
    this.telemetry.qpaceVersion ??= "0.0.1";
  }

  private createGrpcMetadata(): grpc.Metadata {
    if (this._grpcMetadata == null) {
      const metadata = new grpc.Metadata();
      metadata.set("x-api-key", `${this.config.apiKey}`);
      if (this.telemetry != null) {
        metadata.set("x-qpace-telemetry", JSON.stringify(this.telemetry));
      }
      this._grpcMetadata = metadata;
    }
    return this._grpcMetadata.clone();
  }

  public async getMe(): Promise<{ team: { id: string; name: string } }> {
    const res = await this.http.get<
      GetTeamMeRequest,
      AxiosResponse<GetTeamMeResponse>
    >("/team/me");
    return res.data;
  }

  public async sym(filter: SymFilter | string | qp.Sym): Promise<qp.Sym> {
    if (filter instanceof qp.Sym) {
      return filter;
    }
    if (typeof filter === "string") {
      filter = { id: filter, tickerId: filter };
    }
    const syms = await this.syms({ ...filter, limit: 1 });
    if (syms.length === 0) {
      throw new Error("No matching symbol found");
    }
    return syms[0];
  }

  public async syms(opts: SymQuery): Promise<qp.Sym[]> {
    const req = new symApi.GetRequest().setQuery(symQueryToProto(opts));
    const res = await new Promise<symApi.GetResponse>((_resolve, _reject) => {
      this.symClient.get(req, this.createGrpcMetadata(), (err, res) => {
        if (err) return _reject(err);
        _resolve(res);
      });
    });
    return res.getSymsList()!.map((s) => protoToSym(s));
  }

  public async bars(
    _sym: SymFilter | string | qp.Sym,
    opts: Omit<OhlcvQuery, "sym"> = {},
  ): Promise<qp.OhlcvBar[]> {
    const sym = await this.sym(_sym);
    if (sym.id == null) {
      throw new Error(`Symbol has no id ${JSON.stringify(sym)}`);
    }
    const query = new ohlcvApi.Query().setFilter(
      ohlcvFilterToProto({ ...opts, sym: sym.id }),
    );
    if (opts?.limit != null) query.setLimit(opts.limit);
    if (opts?.offset != null) query.setOffset(opts.offset);
    const req = new ohlcvApi.GetRequest().setQuery(query);
    const res = await new Promise<ohlcvApi.GetResponse>((_resolve, _reject) => {
      this.ohlcvClient.get(req, this.createGrpcMetadata(), (err, res) => {
        if (err) return _reject(err);
        _resolve(res);
      });
    });
    return res.getBarsList().map(protoToOhlcvBar);
  }

  public async ohlcv(
    _sym: SymFilter | string | qp.Sym,
    query: Omit<OhlcvQuery, "sym"> | qp.Ohlcv = {},
  ): Promise<qp.Ohlcv> {
    if (query instanceof qp.Ohlcv) {
      return query;
    }
    const bars = await this.bars(_sym, query);
    return qp.Ohlcv.fromBars(bars);
  }

  public async ctx(
    _sym: SymFilter | string | qp.Sym,
    query: Omit<OhlcvQuery, "sym"> | qp.Ohlcv = {},
  ): Promise<qp.Ctx> {
    const sym = await this.sym(_sym);
    const ohlcv = await this.ohlcv(sym, query);
    return new qp.Ctx(ohlcv, sym);
  }

  // public async bt(
  //   _sym: SymFilter | string | qp.Sym,
  //   query: Omit<OhlcvQuery, "sym"> & {
  //     initialCapital?: number;
  //     processOrdersOnClose?: boolean;
  //   } = {},
  // ): Promise<qp.Backtest> {
  //   const ctx = await this.ctx(_sym, query);
  //   const btConfig = new qp.BacktestConfig();
  //   if (query.initialCapital != null) {
  //     btConfig.initialCapital = query.initialCapital;
  //   }
  //   if (query.processOrdersOnClose != null) {
  //     btConfig.processOrdersOnClose = query.processOrdersOnClose;
  //   }
  //   const bt = new qp.Backtest(ctx, btConfig);
  //   return bt;
  // }
}
