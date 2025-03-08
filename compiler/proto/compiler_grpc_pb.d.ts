// package: compiler
// file: compiler.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as compiler_pb from "./compiler_pb";

interface ICompilerService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    build: ICompilerService_IBuild;
}

interface ICompilerService_IBuild extends grpc.MethodDefinition<compiler_pb.BuildRequest, compiler_pb.BuildResponseEvent> {
    path: "/compiler.Compiler/Build";
    requestStream: false;
    responseStream: true;
    requestSerialize: grpc.serialize<compiler_pb.BuildRequest>;
    requestDeserialize: grpc.deserialize<compiler_pb.BuildRequest>;
    responseSerialize: grpc.serialize<compiler_pb.BuildResponseEvent>;
    responseDeserialize: grpc.deserialize<compiler_pb.BuildResponseEvent>;
}

export const CompilerService: ICompilerService;

export interface ICompilerServer extends grpc.UntypedServiceImplementation {
    build: grpc.handleServerStreamingCall<compiler_pb.BuildRequest, compiler_pb.BuildResponseEvent>;
}

export interface ICompilerClient {
    build(request: compiler_pb.BuildRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<compiler_pb.BuildResponseEvent>;
    build(request: compiler_pb.BuildRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<compiler_pb.BuildResponseEvent>;
}

export class CompilerClient extends grpc.Client implements ICompilerClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public build(request: compiler_pb.BuildRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<compiler_pb.BuildResponseEvent>;
    public build(request: compiler_pb.BuildRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<compiler_pb.BuildResponseEvent>;
}
