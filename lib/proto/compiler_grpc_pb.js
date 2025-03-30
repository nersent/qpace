// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var compiler_pb = require('./compiler_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');

function serialize_compiler_BuildRequest(arg) {
  if (!(arg instanceof compiler_pb.BuildRequest)) {
    throw new Error('Expected argument of type compiler.BuildRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_compiler_BuildRequest(buffer_arg) {
  return compiler_pb.BuildRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_compiler_StageEvent(arg) {
  if (!(arg instanceof compiler_pb.StageEvent)) {
    throw new Error('Expected argument of type compiler.StageEvent');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_compiler_StageEvent(buffer_arg) {
  return compiler_pb.StageEvent.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_compiler_VersionRequest(arg) {
  if (!(arg instanceof compiler_pb.VersionRequest)) {
    throw new Error('Expected argument of type compiler.VersionRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_compiler_VersionRequest(buffer_arg) {
  return compiler_pb.VersionRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_compiler_VersionResponse(arg) {
  if (!(arg instanceof compiler_pb.VersionResponse)) {
    throw new Error('Expected argument of type compiler.VersionResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_compiler_VersionResponse(buffer_arg) {
  return compiler_pb.VersionResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


var CompilerApiService = exports.CompilerApiService = {
  version: {
    path: '/compiler.CompilerApi/Version',
    requestStream: false,
    responseStream: false,
    requestType: compiler_pb.VersionRequest,
    responseType: compiler_pb.VersionResponse,
    requestSerialize: serialize_compiler_VersionRequest,
    requestDeserialize: deserialize_compiler_VersionRequest,
    responseSerialize: serialize_compiler_VersionResponse,
    responseDeserialize: deserialize_compiler_VersionResponse,
  },
  build: {
    path: '/compiler.CompilerApi/Build',
    requestStream: false,
    responseStream: true,
    requestType: compiler_pb.BuildRequest,
    responseType: compiler_pb.StageEvent,
    requestSerialize: serialize_compiler_BuildRequest,
    requestDeserialize: deserialize_compiler_BuildRequest,
    responseSerialize: serialize_compiler_StageEvent,
    responseDeserialize: deserialize_compiler_StageEvent,
  },
};

exports.CompilerApiClient = grpc.makeGenericClientConstructor(CompilerApiService);
