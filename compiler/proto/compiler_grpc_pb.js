// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var compiler_pb = require('./compiler_pb.js');

function serialize_compiler_BuildRequestEvent(arg) {
  if (!(arg instanceof compiler_pb.BuildRequestEvent)) {
    throw new Error('Expected argument of type compiler.BuildRequestEvent');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_compiler_BuildRequestEvent(buffer_arg) {
  return compiler_pb.BuildRequestEvent.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_compiler_BuildResponseEvent(arg) {
  if (!(arg instanceof compiler_pb.BuildResponseEvent)) {
    throw new Error('Expected argument of type compiler.BuildResponseEvent');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_compiler_BuildResponseEvent(buffer_arg) {
  return compiler_pb.BuildResponseEvent.deserializeBinary(new Uint8Array(buffer_arg));
}


var CompilerService = exports.CompilerService = {
  build: {
    path: '/compiler.Compiler/Build',
    requestStream: true,
    responseStream: true,
    requestType: compiler_pb.BuildRequestEvent,
    responseType: compiler_pb.BuildResponseEvent,
    requestSerialize: serialize_compiler_BuildRequestEvent,
    requestDeserialize: deserialize_compiler_BuildRequestEvent,
    responseSerialize: serialize_compiler_BuildResponseEvent,
    responseDeserialize: deserialize_compiler_BuildResponseEvent,
  },
};

exports.CompilerClient = grpc.makeGenericClientConstructor(CompilerService);
