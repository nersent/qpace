import * as grpc from "@grpc/grpc-js";
import axios, { AxiosInstance, AxiosResponse } from "axios";

import {
  DEFAULT_GRPC_ENDPOINT,
  DEFAULT_REST_ENDPOINT,
  GetTeamMeRequest,
  GetTeamMeResponse,
  OhlcvQuery,
  ohlcvQueryToProto,
  protoToQpOhlcvBar,
  protoToQpSym,
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
  }

  private createGrpcMetadata(): grpc.Metadata {
    const metadata = new grpc.Metadata();
    metadata.set("x-api-key", this.config.apiKey);
    return metadata;
  }

  public async getMe(): Promise<{ team: { id: string; name: string } }> {
    const res = await this.http.get<
      GetTeamMeRequest,
      AxiosResponse<GetTeamMeResponse>
    >("/team/me");
    return res.data;
  }

  public async sym(query: SymQuery): Promise<qp.Sym> {
    const req = new symApi.GetRequest().setQuery(symQueryToProto(query));
    const res = await new Promise<symApi.GetResponse>((_resolve, _reject) => {
      this.symClient.get(req, this.createGrpcMetadata(), (err, res) => {
        if (err) return _reject(err);
        _resolve(res);
      });
    });
    return protoToQpSym(res.getSym()!);
  }

  public async syms(query: SymQuery = {}): Promise<qp.Sym[]> {
    const req = new symApi.GetListRequest().setQuery(symQueryToProto(query));
    const res = await new Promise<symApi.GetListResponse>(
      (_resolve, _reject) => {
        this.symClient.getList(req, this.createGrpcMetadata(), (err, res) => {
          if (err) return _reject(err);
          _resolve(res);
        });
      },
    );
    return res.getSymsList().map((r) => protoToQpSym(r));
  }

  public async bars(query: OhlcvQuery): Promise<qp.OhlcvBar[]> {
    const req = new ohlcvApi.GetRequest().setQuery(ohlcvQueryToProto(query));
    const res = await new Promise<ohlcvApi.GetResponse>((_resolve, _reject) => {
      this.ohlcvClient.get(req, this.createGrpcMetadata(), (err, res) => {
        if (err) return _reject(err);
        _resolve(res);
      });
    });
    return res.getBarsList().map(protoToQpOhlcvBar);
  }

  public async ohlcv(query: OhlcvQuery): Promise<qp.Ohlcv> {
    const bars = await this.bars(query);
    return qp.Ohlcv.fromBars(bars);
  }

  public async ctx(
    query: SymQuery & Omit<OhlcvQuery, "symId">,
  ): Promise<qp.Ctx> {
    const sym = await this.sym(query);
    const ohlcv = await this.ohlcv(query);
    return new qp.Ctx(ohlcv, sym);
  }
}
