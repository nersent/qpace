// package: 
// file: sym.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as sym_pb from "./sym_pb";

interface ISymApiService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    get: ISymApiService_IGet;
    getList: ISymApiService_IGetList;
}

interface ISymApiService_IGet extends grpc.MethodDefinition<sym_pb.GetRequest, sym_pb.GetResponse> {
    path: "/SymApi/Get";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<sym_pb.GetRequest>;
    requestDeserialize: grpc.deserialize<sym_pb.GetRequest>;
    responseSerialize: grpc.serialize<sym_pb.GetResponse>;
    responseDeserialize: grpc.deserialize<sym_pb.GetResponse>;
}
interface ISymApiService_IGetList extends grpc.MethodDefinition<sym_pb.GetListRequest, sym_pb.GetListResponse> {
    path: "/SymApi/GetList";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<sym_pb.GetListRequest>;
    requestDeserialize: grpc.deserialize<sym_pb.GetListRequest>;
    responseSerialize: grpc.serialize<sym_pb.GetListResponse>;
    responseDeserialize: grpc.deserialize<sym_pb.GetListResponse>;
}

export const SymApiService: ISymApiService;

export interface ISymApiServer extends grpc.UntypedServiceImplementation {
    get: grpc.handleUnaryCall<sym_pb.GetRequest, sym_pb.GetResponse>;
    getList: grpc.handleUnaryCall<sym_pb.GetListRequest, sym_pb.GetListResponse>;
}

export interface ISymApiClient {
    get(request: sym_pb.GetRequest, callback: (error: grpc.ServiceError | null, response: sym_pb.GetResponse) => void): grpc.ClientUnaryCall;
    get(request: sym_pb.GetRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: sym_pb.GetResponse) => void): grpc.ClientUnaryCall;
    get(request: sym_pb.GetRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: sym_pb.GetResponse) => void): grpc.ClientUnaryCall;
    getList(request: sym_pb.GetListRequest, callback: (error: grpc.ServiceError | null, response: sym_pb.GetListResponse) => void): grpc.ClientUnaryCall;
    getList(request: sym_pb.GetListRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: sym_pb.GetListResponse) => void): grpc.ClientUnaryCall;
    getList(request: sym_pb.GetListRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: sym_pb.GetListResponse) => void): grpc.ClientUnaryCall;
}

export class SymApiClient extends grpc.Client implements ISymApiClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public get(request: sym_pb.GetRequest, callback: (error: grpc.ServiceError | null, response: sym_pb.GetResponse) => void): grpc.ClientUnaryCall;
    public get(request: sym_pb.GetRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: sym_pb.GetResponse) => void): grpc.ClientUnaryCall;
    public get(request: sym_pb.GetRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: sym_pb.GetResponse) => void): grpc.ClientUnaryCall;
    public getList(request: sym_pb.GetListRequest, callback: (error: grpc.ServiceError | null, response: sym_pb.GetListResponse) => void): grpc.ClientUnaryCall;
    public getList(request: sym_pb.GetListRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: sym_pb.GetListResponse) => void): grpc.ClientUnaryCall;
    public getList(request: sym_pb.GetListRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: sym_pb.GetListResponse) => void): grpc.ClientUnaryCall;
}
