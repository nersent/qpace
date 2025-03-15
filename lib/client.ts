import * as grpc from "@grpc/grpc-js";
import axios, { AxiosInstance } from "axios";
import { CompilerApiClient } from "~/common/proto/compiler_grpc_pb";
import { SymApiClient } from "~/common/proto/sym_grpc_pb";
import * as symApi from "~/common/proto/sym_pb";

import * as qp from "./index";

export const DEFAULT_REST_ENDPOINT = "http://0.0.0.0:3000/v1";
export const DEFAULT_GRPC_ENDPOINT = "0.0.0.0:3001";

export interface ClientConfig {
  restEndpoint?: string;
  grpcEndpoint?: string;
  apiKey: string;
}

export interface SymQuery {
  id?: string;
  tickerId?: string;
}

const symQueryToProto = (query: SymQuery): symApi.FindQuery => {
  const proto = new symApi.FindQuery();
  if (query.id) proto.setId(query.id);
  if (query.tickerId) proto.setTickerId(query.tickerId);
  return proto;
};

// const protoSymToQp = (proto: symApi.Sym, lib: qp.Lib): qp.Sym => {};

export class Client {
  public readonly http: AxiosInstance;
  public readonly compilerApiClient: CompilerApiClient;
  public readonly symApiClient: SymApiClient;
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

    this.compilerApiClient = new CompilerApiClient(
      grpcEndpoint,
      this.grpcCredentials,
      this.grpcOptions,
    );
    this.symApiClient = new SymApiClient(
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

  public async sym(query: SymQuery): Promise<any> {
    const req = new symApi.GetRequest().setQuery(symQueryToProto(query));
    const res = await new Promise<symApi.GetResponse>((_resolve, _reject) => {
      this.symApiClient.get(req, this.createGrpcMetadata(), (err, res) => {
        if (err) return _reject(err);
        _resolve(res);
      });
    });
    console.log(res.getSym());
    return undefined as any;
  }
}
