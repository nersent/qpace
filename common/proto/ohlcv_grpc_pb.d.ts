// package: ohlcv
// file: ohlcv.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as ohlcv_pb from "./ohlcv_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";

interface IOhlcvApiService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
}


export const OhlcvApiService: IOhlcvApiService;

export interface IOhlcvApiServer extends grpc.UntypedServiceImplementation {
}

export interface IOhlcvApiClient {
}

export class OhlcvApiClient extends grpc.Client implements IOhlcvApiClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
}
