// package: 
// file: sym.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as sym_pb from "./sym_pb";

interface ISymApiService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    get: ISymApiService_IGet;
}

interface ISymApiService_IGet extends grpc.MethodDefinition<sym_pb.GetSymRequest, sym_pb.GetSymResponse> {
    path: "/SymApi/Get";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<sym_pb.GetSymRequest>;
    requestDeserialize: grpc.deserialize<sym_pb.GetSymRequest>;
    responseSerialize: grpc.serialize<sym_pb.GetSymResponse>;
    responseDeserialize: grpc.deserialize<sym_pb.GetSymResponse>;
}

export const SymApiService: ISymApiService;

export interface ISymApiServer extends grpc.UntypedServiceImplementation {
    get: grpc.handleUnaryCall<sym_pb.GetSymRequest, sym_pb.GetSymResponse>;
}

export interface ISymApiClient {
    get(request: sym_pb.GetSymRequest, callback: (error: grpc.ServiceError | null, response: sym_pb.GetSymResponse) => void): grpc.ClientUnaryCall;
    get(request: sym_pb.GetSymRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: sym_pb.GetSymResponse) => void): grpc.ClientUnaryCall;
    get(request: sym_pb.GetSymRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: sym_pb.GetSymResponse) => void): grpc.ClientUnaryCall;
}

export class SymApiClient extends grpc.Client implements ISymApiClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public get(request: sym_pb.GetSymRequest, callback: (error: grpc.ServiceError | null, response: sym_pb.GetSymResponse) => void): grpc.ClientUnaryCall;
    public get(request: sym_pb.GetSymRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: sym_pb.GetSymResponse) => void): grpc.ClientUnaryCall;
    public get(request: sym_pb.GetSymRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: sym_pb.GetSymResponse) => void): grpc.ClientUnaryCall;
}
