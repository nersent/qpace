// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var sym_pb = require('./sym_pb.js');

function serialize_GetListRequest(arg) {
  if (!(arg instanceof sym_pb.GetListRequest)) {
    throw new Error('Expected argument of type GetListRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_GetListRequest(buffer_arg) {
  return sym_pb.GetListRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_GetListResponse(arg) {
  if (!(arg instanceof sym_pb.GetListResponse)) {
    throw new Error('Expected argument of type GetListResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_GetListResponse(buffer_arg) {
  return sym_pb.GetListResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_GetRequest(arg) {
  if (!(arg instanceof sym_pb.GetRequest)) {
    throw new Error('Expected argument of type GetRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_GetRequest(buffer_arg) {
  return sym_pb.GetRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_GetResponse(arg) {
  if (!(arg instanceof sym_pb.GetResponse)) {
    throw new Error('Expected argument of type GetResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_GetResponse(buffer_arg) {
  return sym_pb.GetResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


var SymApiService = exports.SymApiService = {
  get: {
    path: '/SymApi/Get',
    requestStream: false,
    responseStream: false,
    requestType: sym_pb.GetRequest,
    responseType: sym_pb.GetResponse,
    requestSerialize: serialize_GetRequest,
    requestDeserialize: deserialize_GetRequest,
    responseSerialize: serialize_GetResponse,
    responseDeserialize: deserialize_GetResponse,
  },
  getList: {
    path: '/SymApi/GetList',
    requestStream: false,
    responseStream: false,
    requestType: sym_pb.GetListRequest,
    responseType: sym_pb.GetListResponse,
    requestSerialize: serialize_GetListRequest,
    requestDeserialize: deserialize_GetListRequest,
    responseSerialize: serialize_GetListResponse,
    responseDeserialize: deserialize_GetListResponse,
  },
};

exports.SymApiClient = grpc.makeGenericClientConstructor(SymApiService);
