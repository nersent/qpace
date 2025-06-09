// package: compiler
// file: schema.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as schema_pb from "./schema_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";

interface ICompilerApiService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    info: ICompilerApiService_IInfo;
    check: ICompilerApiService_ICheck;
    build: ICompilerApiService_IBuild;
}

interface ICompilerApiService_IInfo extends grpc.MethodDefinition<schema_pb.InfoRequest, schema_pb.InfoResponse> {
    path: "/compiler.CompilerApi/Info";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<schema_pb.InfoRequest>;
    requestDeserialize: grpc.deserialize<schema_pb.InfoRequest>;
    responseSerialize: grpc.serialize<schema_pb.InfoResponse>;
    responseDeserialize: grpc.deserialize<schema_pb.InfoResponse>;
}
interface ICompilerApiService_ICheck extends grpc.MethodDefinition<schema_pb.CheckRequest, schema_pb.CheckResponse> {
    path: "/compiler.CompilerApi/Check";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<schema_pb.CheckRequest>;
    requestDeserialize: grpc.deserialize<schema_pb.CheckRequest>;
    responseSerialize: grpc.serialize<schema_pb.CheckResponse>;
    responseDeserialize: grpc.deserialize<schema_pb.CheckResponse>;
}
interface ICompilerApiService_IBuild extends grpc.MethodDefinition<schema_pb.BuildRequest, schema_pb.BuildEvent> {
    path: "/compiler.CompilerApi/Build";
    requestStream: false;
    responseStream: true;
    requestSerialize: grpc.serialize<schema_pb.BuildRequest>;
    requestDeserialize: grpc.deserialize<schema_pb.BuildRequest>;
    responseSerialize: grpc.serialize<schema_pb.BuildEvent>;
    responseDeserialize: grpc.deserialize<schema_pb.BuildEvent>;
}

export const CompilerApiService: ICompilerApiService;

export interface ICompilerApiServer extends grpc.UntypedServiceImplementation {
    info: grpc.handleUnaryCall<schema_pb.InfoRequest, schema_pb.InfoResponse>;
    check: grpc.handleUnaryCall<schema_pb.CheckRequest, schema_pb.CheckResponse>;
    build: grpc.handleServerStreamingCall<schema_pb.BuildRequest, schema_pb.BuildEvent>;
}

export interface ICompilerApiClient {
    info(request: schema_pb.InfoRequest, callback: (error: grpc.ServiceError | null, response: schema_pb.InfoResponse) => void): grpc.ClientUnaryCall;
    info(request: schema_pb.InfoRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: schema_pb.InfoResponse) => void): grpc.ClientUnaryCall;
    info(request: schema_pb.InfoRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: schema_pb.InfoResponse) => void): grpc.ClientUnaryCall;
    check(request: schema_pb.CheckRequest, callback: (error: grpc.ServiceError | null, response: schema_pb.CheckResponse) => void): grpc.ClientUnaryCall;
    check(request: schema_pb.CheckRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: schema_pb.CheckResponse) => void): grpc.ClientUnaryCall;
    check(request: schema_pb.CheckRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: schema_pb.CheckResponse) => void): grpc.ClientUnaryCall;
    build(request: schema_pb.BuildRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<schema_pb.BuildEvent>;
    build(request: schema_pb.BuildRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<schema_pb.BuildEvent>;
}

export class CompilerApiClient extends grpc.Client implements ICompilerApiClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public info(request: schema_pb.InfoRequest, callback: (error: grpc.ServiceError | null, response: schema_pb.InfoResponse) => void): grpc.ClientUnaryCall;
    public info(request: schema_pb.InfoRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: schema_pb.InfoResponse) => void): grpc.ClientUnaryCall;
    public info(request: schema_pb.InfoRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: schema_pb.InfoResponse) => void): grpc.ClientUnaryCall;
    public check(request: schema_pb.CheckRequest, callback: (error: grpc.ServiceError | null, response: schema_pb.CheckResponse) => void): grpc.ClientUnaryCall;
    public check(request: schema_pb.CheckRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: schema_pb.CheckResponse) => void): grpc.ClientUnaryCall;
    public check(request: schema_pb.CheckRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: schema_pb.CheckResponse) => void): grpc.ClientUnaryCall;
    public build(request: schema_pb.BuildRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<schema_pb.BuildEvent>;
    public build(request: schema_pb.BuildRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<schema_pb.BuildEvent>;
}
