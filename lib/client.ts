import * as grpc from "@grpc/grpc-js";
import axios, { AxiosInstance } from "axios";

import {
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

export const DEFAULT_REST_ENDPOINT = "http://0.0.0.0:3000/v1";
export const DEFAULT_GRPC_ENDPOINT = "0.0.0.0:3001";

export { SymQuery };

export interface ClientConfig {
  restEndpoint?: string;
  grpcEndpoint?: string;
  apiKey: string;
}

export class Client {
  public readonly http: AxiosInstance;
  public readonly compilerClient: CompilerApiClient;
  public readonly symClient: SymApiClient;
  public readonly ohlcvClient: OhlcvApiClient;
  public readonly grpcCredentials: grpc.ChannelCredentials;
  public readonly grpcOptions: Partial<grpc.ClientOptions>;
  private readonly apiKey: string;

  constructor(config: ClientConfig) {
    this.apiKey = config.apiKey;
    const restEndpoint = config.restEndpoint ?? DEFAULT_REST_ENDPOINT;
    const grpcEndpoint = config.grpcEndpoint ?? DEFAULT_GRPC_ENDPOINT;
    this.grpcCredentials = grpc.ChannelCredentials.createInsecure();
    this.grpcOptions = {
      "grpc.max_receive_message_length": -1,
    };

    this.http = axios.create({
      baseURL: restEndpoint,
      withCredentials: true,
      headers: {
        "Content-Type": "application/json",
      },
    });

    this.compilerClient = new CompilerApiClient(
      grpcEndpoint,
      this.grpcCredentials,
      this.grpcOptions,
    );
    this.symClient = new SymApiClient(
      grpcEndpoint,
      this.grpcCredentials,
      this.grpcOptions,
    );
    this.ohlcvClient = new OhlcvApiClient(
      grpcEndpoint,
      this.grpcCredentials,
      this.grpcOptions,
    );
  }

  private createGrpcMetadata(): grpc.Metadata {
    const metadata = new grpc.Metadata();
    metadata.set("x-api-key", this.apiKey);
    return metadata;
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

  public async syms(query: SymQuery): Promise<qp.Sym[]> {
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
