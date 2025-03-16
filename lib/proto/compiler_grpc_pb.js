// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var compiler_pb = require('./compiler_pb.js');
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


var CompilerApiService = exports.CompilerApiService = {
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
