// package: compiler
// file: compiler.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as compiler_pb from "./compiler_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";

interface ICompilerApiService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    version: ICompilerApiService_IVersion;
    build: ICompilerApiService_IBuild;
}

interface ICompilerApiService_IVersion extends grpc.MethodDefinition<compiler_pb.VersionRequest, compiler_pb.VersionResponse> {
    path: "/compiler.CompilerApi/Version";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<compiler_pb.VersionRequest>;
    requestDeserialize: grpc.deserialize<compiler_pb.VersionRequest>;
    responseSerialize: grpc.serialize<compiler_pb.VersionResponse>;
    responseDeserialize: grpc.deserialize<compiler_pb.VersionResponse>;
}
interface ICompilerApiService_IBuild extends grpc.MethodDefinition<compiler_pb.BuildRequest, compiler_pb.StageEvent> {
    path: "/compiler.CompilerApi/Build";
    requestStream: false;
    responseStream: true;
    requestSerialize: grpc.serialize<compiler_pb.BuildRequest>;
    requestDeserialize: grpc.deserialize<compiler_pb.BuildRequest>;
    responseSerialize: grpc.serialize<compiler_pb.StageEvent>;
    responseDeserialize: grpc.deserialize<compiler_pb.StageEvent>;
}

export const CompilerApiService: ICompilerApiService;

export interface ICompilerApiServer extends grpc.UntypedServiceImplementation {
    version: grpc.handleUnaryCall<compiler_pb.VersionRequest, compiler_pb.VersionResponse>;
    build: grpc.handleServerStreamingCall<compiler_pb.BuildRequest, compiler_pb.StageEvent>;
}

export interface ICompilerApiClient {
    version(request: compiler_pb.VersionRequest, callback: (error: grpc.ServiceError | null, response: compiler_pb.VersionResponse) => void): grpc.ClientUnaryCall;
    version(request: compiler_pb.VersionRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: compiler_pb.VersionResponse) => void): grpc.ClientUnaryCall;
    version(request: compiler_pb.VersionRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: compiler_pb.VersionResponse) => void): grpc.ClientUnaryCall;
    build(request: compiler_pb.BuildRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<compiler_pb.StageEvent>;
    build(request: compiler_pb.BuildRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<compiler_pb.StageEvent>;
}

export class CompilerApiClient extends grpc.Client implements ICompilerApiClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public version(request: compiler_pb.VersionRequest, callback: (error: grpc.ServiceError | null, response: compiler_pb.VersionResponse) => void): grpc.ClientUnaryCall;
    public version(request: compiler_pb.VersionRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: compiler_pb.VersionResponse) => void): grpc.ClientUnaryCall;
    public version(request: compiler_pb.VersionRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: compiler_pb.VersionResponse) => void): grpc.ClientUnaryCall;
    public build(request: compiler_pb.BuildRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<compiler_pb.StageEvent>;
    public build(request: compiler_pb.BuildRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<compiler_pb.StageEvent>;
}
