import * as grpc from "@grpc/grpc-js";
import axios, { AxiosInstance, AxiosResponse } from "axios";
import {
  version as VERSION,
  coreVersion as CORE_VERSION,
} from "../../package.json";

// export const DEFAULT_REST_ENDPOINT = `http://0.0.0.0:3000/v1`;
// export const DEFAULT_GRPC_ENDPOINT = `0.0.0.0:3001`;

const DEFAULT_REST_ENDPOINT = `https://api.qpace.dev/v1`;
const DEFAULT_GRPC_ENDPOINT = `grpc.qpace.dev`;

export interface ClientConfig {
  apiKey: string;
  apiBase?: string;
  grpcApiBase?: string;
  timeout?: number;
  grpcCredentials?: grpc.ChannelCredentials;
}

export class Client {
  private _clientInfo: Record<string, any> = {};
  private _http!: AxiosInstance;
  private _grpcMetadata: grpc.Metadata | undefined;
  private _grpcOptions: grpc.ClientOptions;

  constructor(public readonly config: ClientConfig) {
    this.config.apiBase ??= DEFAULT_REST_ENDPOINT;
    this.config.grpcApiBase ??= DEFAULT_GRPC_ENDPOINT;
    this.config.grpcCredentials ??= grpc.ChannelCredentials.createInsecure();
    this._clientInfo = {
      ...this._clientInfo,
      qpace: VERSION,
      qpaceCore: CORE_VERSION,
      client: "node",
      node: process.versions.node,
      platform: process.platform,
      arch: process.arch,
    };
    this._grpcOptions = {
      "grpc.max_receive_message_length": -1,
      "grpc.max_send_message_length": -1,
    };
  }

  private get http(): AxiosInstance {
    if (this._http == null) {
      this._http = axios.create({
        baseURL: this.config.apiBase,
        withCredentials: true,
        headers: {
          "Content-Type": "application/json",
          "x-api-key": this.config.apiKey,
          "x-info": JSON.stringify(this._clientInfo),
        },
      });
    }
    return this._http;
  }

  private get grpcMetadata(): grpc.Metadata {
    if (this._grpcMetadata == null) {
      const metadata = new grpc.Metadata();
      metadata.set("x-api-key", `${this.config.apiKey}`);
      if (this._clientInfo != null) {
        metadata.set("x-info", JSON.stringify(this._clientInfo));
      }
      this._grpcMetadata = metadata;
    }
    return this._grpcMetadata.clone();
  }

  public async me(): Promise<{ id: string; name: string }> {
    const {
      data: { user },
    } = await this.http.get(`/api_keys/${this.config.apiKey}`);
    return user;
  }
}
