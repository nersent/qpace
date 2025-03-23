// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var sym_pb = require('./sym_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');

function serialize_sym_GetRequest(arg) {
  if (!(arg instanceof sym_pb.GetRequest)) {
    throw new Error('Expected argument of type sym.GetRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_sym_GetRequest(buffer_arg) {
  return sym_pb.GetRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_sym_GetResponse(arg) {
  if (!(arg instanceof sym_pb.GetResponse)) {
    throw new Error('Expected argument of type sym.GetResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_sym_GetResponse(buffer_arg) {
  return sym_pb.GetResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


var SymApiService = exports.SymApiService = {
  get: {
    path: '/sym.SymApi/Get',
    requestStream: false,
    responseStream: false,
    requestType: sym_pb.GetRequest,
    responseType: sym_pb.GetResponse,
    requestSerialize: serialize_sym_GetRequest,
    requestDeserialize: deserialize_sym_GetRequest,
    responseSerialize: serialize_sym_GetResponse,
    responseDeserialize: deserialize_sym_GetResponse,
  },
};

exports.SymApiClient = grpc.makeGenericClientConstructor(SymApiService);
