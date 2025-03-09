// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var ohlcv_pb = require('./ohlcv_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');

function serialize_ohlcv_GetOhlcvRequest(arg) {
  if (!(arg instanceof ohlcv_pb.GetOhlcvRequest)) {
    throw new Error('Expected argument of type ohlcv.GetOhlcvRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_ohlcv_GetOhlcvRequest(buffer_arg) {
  return ohlcv_pb.GetOhlcvRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_ohlcv_GetOhlcvResponse(arg) {
  if (!(arg instanceof ohlcv_pb.GetOhlcvResponse)) {
    throw new Error('Expected argument of type ohlcv.GetOhlcvResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_ohlcv_GetOhlcvResponse(buffer_arg) {
  return ohlcv_pb.GetOhlcvResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


var OhlcvApiService = exports.OhlcvApiService = {
  get: {
    path: '/ohlcv.OhlcvApi/Get',
    requestStream: false,
    responseStream: false,
    requestType: ohlcv_pb.GetOhlcvRequest,
    responseType: ohlcv_pb.GetOhlcvResponse,
    requestSerialize: serialize_ohlcv_GetOhlcvRequest,
    requestDeserialize: deserialize_ohlcv_GetOhlcvRequest,
    responseSerialize: serialize_ohlcv_GetOhlcvResponse,
    responseDeserialize: deserialize_ohlcv_GetOhlcvResponse,
  },
};

exports.OhlcvApiClient = grpc.makeGenericClientConstructor(OhlcvApiService);
