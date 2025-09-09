import * as grpc from "@grpc/grpc-js";
import axios, { AxiosInstance, AxiosResponse } from "axios";
import {
  version as VERSION,
  coreVersion as CORE_VERSION,
} from "../../package.json";
import { type Timeframe, type Sym } from "./index";

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

class UserClient {
  constructor(private readonly client: Client) {}

  public async me(): Promise<{ id: string; firstName?: string }> {
    const {
      data: { user },
    } = await this.client.http.get(`/api-keys/${this.client.config.apiKey}`);
    return user;
  }
}

type SymFilter = {
  id?: string;
  tickerId?: string;
  timeframe?: Timeframe | string;
  limit?: number;
  offset?: number;
};

class SymClient {
  constructor(private readonly client: Client) {}

  public async get(query: SymFilter | string): Promise<Sym> {
    const _query: SymFilter = {
      ...(typeof query === "string" ? { id: query, tickerId: query } : query),
      limit: 1,
    };
    const syms = await this.syms(_query);
    if (syms.length === 0) {
      throw new Error("Symbol not found");
    }
    return syms[0];
  }

  public async syms(query: SymFilter = {}): Promise<Sym[]> {
    const timeframe = query?.timeframe?.toString();
    const { data } = await this.client.http.get(`/symbols`, {
      params: {
        id_pat: query.id,
        ticker_id_pat: query.tickerId,
        limit: query?.limit,
        offset: query?.offset,
        timeframe,
      },
    });
    const { Sym } = require("./index");
    return data["symbols"].map((r: any) => Sym.fromJSON(r));
  }
}

export class Client {
  private _clientInfo: Record<string, any> = {};
  private _http!: AxiosInstance;
  private _grpcMetadata: grpc.Metadata | undefined;
  private _grpcOptions: grpc.ClientOptions;
  public readonly user: UserClient;
  public readonly sym: SymClient;

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
    this.user = new UserClient(this);
    this.sym = new SymClient(this);
  }

  public get http(): AxiosInstance {
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

  public get grpcMetadata(): grpc.Metadata {
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
}
