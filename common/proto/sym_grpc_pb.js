// GENERATED CODE -- DO NOT EDIT!
'use strict';
var grpc = require('@grpc/grpc-js');
var sym_pb = require('./sym_pb.js');
function serialize_GetSymRequest(arg) {
    if (!(arg instanceof sym_pb.GetSymRequest)) {
        throw new Error('Expected argument of type GetSymRequest');
    }
    return Buffer.from(arg.serializeBinary());
}
function deserialize_GetSymRequest(buffer_arg) {
    return sym_pb.GetSymRequest.deserializeBinary(new Uint8Array(buffer_arg));
}
function serialize_GetSymResponse(arg) {
    if (!(arg instanceof sym_pb.GetSymResponse)) {
        throw new Error('Expected argument of type GetSymResponse');
    }
    return Buffer.from(arg.serializeBinary());
}
function deserialize_GetSymResponse(buffer_arg) {
    return sym_pb.GetSymResponse.deserializeBinary(new Uint8Array(buffer_arg));
}
var SymApiService = exports.SymApiService = {
    get: {
        path: '/SymApi/Get',
        requestStream: false,
        responseStream: false,
        requestType: sym_pb.GetSymRequest,
        responseType: sym_pb.GetSymResponse,
        requestSerialize: serialize_GetSymRequest,
        requestDeserialize: deserialize_GetSymRequest,
        responseSerialize: serialize_GetSymResponse,
        responseDeserialize: deserialize_GetSymResponse
    }
};
exports.SymApiClient = grpc.makeGenericClientConstructor(SymApiService);
