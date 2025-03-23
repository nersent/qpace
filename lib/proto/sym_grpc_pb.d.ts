// package: sym
// file: sym.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as sym_pb from "./sym_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";

interface ISymApiService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    get: ISymApiService_IGet;
}

interface ISymApiService_IGet extends grpc.MethodDefinition<sym_pb.GetRequest, sym_pb.GetResponse> {
    path: "/sym.SymApi/Get";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<sym_pb.GetRequest>;
    requestDeserialize: grpc.deserialize<sym_pb.GetRequest>;
    responseSerialize: grpc.serialize<sym_pb.GetResponse>;
    responseDeserialize: grpc.deserialize<sym_pb.GetResponse>;
}

export const SymApiService: ISymApiService;

export interface ISymApiServer extends grpc.UntypedServiceImplementation {
    get: grpc.handleUnaryCall<sym_pb.GetRequest, sym_pb.GetResponse>;
}

export interface ISymApiClient {
    get(request: sym_pb.GetRequest, callback: (error: grpc.ServiceError | null, response: sym_pb.GetResponse) => void): grpc.ClientUnaryCall;
    get(request: sym_pb.GetRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: sym_pb.GetResponse) => void): grpc.ClientUnaryCall;
    get(request: sym_pb.GetRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: sym_pb.GetResponse) => void): grpc.ClientUnaryCall;
}

export class SymApiClient extends grpc.Client implements ISymApiClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public get(request: sym_pb.GetRequest, callback: (error: grpc.ServiceError | null, response: sym_pb.GetResponse) => void): grpc.ClientUnaryCall;
    public get(request: sym_pb.GetRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: sym_pb.GetResponse) => void): grpc.ClientUnaryCall;
    public get(request: sym_pb.GetRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: sym_pb.GetResponse) => void): grpc.ClientUnaryCall;
}
