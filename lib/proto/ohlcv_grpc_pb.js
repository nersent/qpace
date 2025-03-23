// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var ohlcv_pb = require('./ohlcv_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');
var sym_pb = require('./sym_pb.js');

function serialize_ohlcv_GetRequest(arg) {
  if (!(arg instanceof ohlcv_pb.GetRequest)) {
    throw new Error('Expected argument of type ohlcv.GetRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_ohlcv_GetRequest(buffer_arg) {
  return ohlcv_pb.GetRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_ohlcv_GetResponse(arg) {
  if (!(arg instanceof ohlcv_pb.GetResponse)) {
    throw new Error('Expected argument of type ohlcv.GetResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_ohlcv_GetResponse(buffer_arg) {
  return ohlcv_pb.GetResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


var OhlcvApiService = exports.OhlcvApiService = {
  get: {
    path: '/ohlcv.OhlcvApi/Get',
    requestStream: false,
    responseStream: false,
    requestType: ohlcv_pb.GetRequest,
    responseType: ohlcv_pb.GetResponse,
    requestSerialize: serialize_ohlcv_GetRequest,
    requestDeserialize: deserialize_ohlcv_GetRequest,
    responseSerialize: serialize_ohlcv_GetResponse,
    responseDeserialize: deserialize_ohlcv_GetResponse,
  },
};

exports.OhlcvApiClient = grpc.makeGenericClientConstructor(OhlcvApiService);
