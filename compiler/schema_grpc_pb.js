// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var schema_pb = require('./schema_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');

function serialize_compiler_BuildEvent(arg) {
  if (!(arg instanceof schema_pb.BuildEvent)) {
    throw new Error('Expected argument of type compiler.BuildEvent');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_compiler_BuildEvent(buffer_arg) {
  return schema_pb.BuildEvent.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_compiler_BuildRequest(arg) {
  if (!(arg instanceof schema_pb.BuildRequest)) {
    throw new Error('Expected argument of type compiler.BuildRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_compiler_BuildRequest(buffer_arg) {
  return schema_pb.BuildRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_compiler_CheckRequest(arg) {
  if (!(arg instanceof schema_pb.CheckRequest)) {
    throw new Error('Expected argument of type compiler.CheckRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_compiler_CheckRequest(buffer_arg) {
  return schema_pb.CheckRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_compiler_CheckResponse(arg) {
  if (!(arg instanceof schema_pb.CheckResponse)) {
    throw new Error('Expected argument of type compiler.CheckResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_compiler_CheckResponse(buffer_arg) {
  return schema_pb.CheckResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_compiler_InfoRequest(arg) {
  if (!(arg instanceof schema_pb.InfoRequest)) {
    throw new Error('Expected argument of type compiler.InfoRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_compiler_InfoRequest(buffer_arg) {
  return schema_pb.InfoRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_compiler_InfoResponse(arg) {
  if (!(arg instanceof schema_pb.InfoResponse)) {
    throw new Error('Expected argument of type compiler.InfoResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_compiler_InfoResponse(buffer_arg) {
  return schema_pb.InfoResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


var CompilerApiService = exports.CompilerApiService = {
  info: {
    path: '/compiler.CompilerApi/Info',
    requestStream: false,
    responseStream: false,
    requestType: schema_pb.InfoRequest,
    responseType: schema_pb.InfoResponse,
    requestSerialize: serialize_compiler_InfoRequest,
    requestDeserialize: deserialize_compiler_InfoRequest,
    responseSerialize: serialize_compiler_InfoResponse,
    responseDeserialize: deserialize_compiler_InfoResponse,
  },
  check: {
    path: '/compiler.CompilerApi/Check',
    requestStream: false,
    responseStream: false,
    requestType: schema_pb.CheckRequest,
    responseType: schema_pb.CheckResponse,
    requestSerialize: serialize_compiler_CheckRequest,
    requestDeserialize: deserialize_compiler_CheckRequest,
    responseSerialize: serialize_compiler_CheckResponse,
    responseDeserialize: deserialize_compiler_CheckResponse,
  },
  build: {
    path: '/compiler.CompilerApi/Build',
    requestStream: false,
    responseStream: true,
    requestType: schema_pb.BuildRequest,
    responseType: schema_pb.BuildEvent,
    requestSerialize: serialize_compiler_BuildRequest,
    requestDeserialize: deserialize_compiler_BuildRequest,
    responseSerialize: serialize_compiler_BuildEvent,
    responseDeserialize: deserialize_compiler_BuildEvent,
  },
};

exports.CompilerApiClient = grpc.makeGenericClientConstructor(CompilerApiService, 'CompilerApi');
