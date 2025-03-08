import { ChannelCredentials } from "@grpc/grpc-js";
import axios, { AxiosInstance } from "axios";
import { CompilerClient } from "~/compiler/proto/compiler_grpc_pb";

export const DEFAULT_REST_ENDPOINT = "http://0.0.0.0:3000/v1";
export const DEFAULT_GRPC_ENDPOINT = "0.0.0.0:3001";

export interface ClientConfig {
  restEndpoint?: string;
  grpcEndpoint?: string;
  token?: string;
}

export class Client {
  public readonly http: AxiosInstance;
  public readonly compilerClient: CompilerClient;
  public readonly grpcCredentials: ChannelCredentials;

  constructor(options: ClientConfig) {
    const restEndpoint = options.restEndpoint ?? DEFAULT_REST_ENDPOINT;
    const grpcEndpoint = options.grpcEndpoint ?? DEFAULT_GRPC_ENDPOINT;
    this.grpcCredentials = ChannelCredentials.createInsecure();

    this.http = axios.create({
      baseURL: restEndpoint,
      withCredentials: true,
      headers: {
        "Content-Type": "application/json",
      },
    });
    this.compilerClient = new CompilerClient(
      grpcEndpoint,
      this.grpcCredentials,
      {
        "grpc.max_receive_message_length": -1,
      },
    );
  }
}
