// package: ohlcv
// file: ohlcv.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as ohlcv_pb from "./ohlcv_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as sym_pb from "./sym_pb";

interface IOhlcvApiService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    get: IOhlcvApiService_IGet;
}

interface IOhlcvApiService_IGet extends grpc.MethodDefinition<ohlcv_pb.GetRequest, ohlcv_pb.GetResponse> {
    path: "/ohlcv.OhlcvApi/Get";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<ohlcv_pb.GetRequest>;
    requestDeserialize: grpc.deserialize<ohlcv_pb.GetRequest>;
    responseSerialize: grpc.serialize<ohlcv_pb.GetResponse>;
    responseDeserialize: grpc.deserialize<ohlcv_pb.GetResponse>;
}

export const OhlcvApiService: IOhlcvApiService;

export interface IOhlcvApiServer extends grpc.UntypedServiceImplementation {
    get: grpc.handleUnaryCall<ohlcv_pb.GetRequest, ohlcv_pb.GetResponse>;
}

export interface IOhlcvApiClient {
    get(request: ohlcv_pb.GetRequest, callback: (error: grpc.ServiceError | null, response: ohlcv_pb.GetResponse) => void): grpc.ClientUnaryCall;
    get(request: ohlcv_pb.GetRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: ohlcv_pb.GetResponse) => void): grpc.ClientUnaryCall;
    get(request: ohlcv_pb.GetRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: ohlcv_pb.GetResponse) => void): grpc.ClientUnaryCall;
}

export class OhlcvApiClient extends grpc.Client implements IOhlcvApiClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public get(request: ohlcv_pb.GetRequest, callback: (error: grpc.ServiceError | null, response: ohlcv_pb.GetResponse) => void): grpc.ClientUnaryCall;
    public get(request: ohlcv_pb.GetRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: ohlcv_pb.GetResponse) => void): grpc.ClientUnaryCall;
    public get(request: ohlcv_pb.GetRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: ohlcv_pb.GetResponse) => void): grpc.ClientUnaryCall;
}
