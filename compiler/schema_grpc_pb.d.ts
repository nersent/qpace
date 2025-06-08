// package: compiler
// file: schema.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as schema_pb from "./schema_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";

interface ICompilerApiService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    version: ICompilerApiService_IVersion;
    build: ICompilerApiService_IBuild;
}

interface ICompilerApiService_IVersion extends grpc.MethodDefinition<schema_pb.VersionRequest, schema_pb.VersionResponse> {
    path: "/compiler.CompilerApi/Version";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<schema_pb.VersionRequest>;
    requestDeserialize: grpc.deserialize<schema_pb.VersionRequest>;
    responseSerialize: grpc.serialize<schema_pb.VersionResponse>;
    responseDeserialize: grpc.deserialize<schema_pb.VersionResponse>;
}
interface ICompilerApiService_IBuild extends grpc.MethodDefinition<schema_pb.BuildRequest, schema_pb.StageEvent> {
    path: "/compiler.CompilerApi/Build";
    requestStream: false;
    responseStream: true;
    requestSerialize: grpc.serialize<schema_pb.BuildRequest>;
    requestDeserialize: grpc.deserialize<schema_pb.BuildRequest>;
    responseSerialize: grpc.serialize<schema_pb.StageEvent>;
    responseDeserialize: grpc.deserialize<schema_pb.StageEvent>;
}

export const CompilerApiService: ICompilerApiService;

export interface ICompilerApiServer extends grpc.UntypedServiceImplementation {
    version: grpc.handleUnaryCall<schema_pb.VersionRequest, schema_pb.VersionResponse>;
    build: grpc.handleServerStreamingCall<schema_pb.BuildRequest, schema_pb.StageEvent>;
}

export interface ICompilerApiClient {
    version(request: schema_pb.VersionRequest, callback: (error: grpc.ServiceError | null, response: schema_pb.VersionResponse) => void): grpc.ClientUnaryCall;
    version(request: schema_pb.VersionRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: schema_pb.VersionResponse) => void): grpc.ClientUnaryCall;
    version(request: schema_pb.VersionRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: schema_pb.VersionResponse) => void): grpc.ClientUnaryCall;
    build(request: schema_pb.BuildRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<schema_pb.StageEvent>;
    build(request: schema_pb.BuildRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<schema_pb.StageEvent>;
}

export class CompilerApiClient extends grpc.Client implements ICompilerApiClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public version(request: schema_pb.VersionRequest, callback: (error: grpc.ServiceError | null, response: schema_pb.VersionResponse) => void): grpc.ClientUnaryCall;
    public version(request: schema_pb.VersionRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: schema_pb.VersionResponse) => void): grpc.ClientUnaryCall;
    public version(request: schema_pb.VersionRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: schema_pb.VersionResponse) => void): grpc.ClientUnaryCall;
    public build(request: schema_pb.BuildRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<schema_pb.StageEvent>;
    public build(request: schema_pb.BuildRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<schema_pb.StageEvent>;
}
