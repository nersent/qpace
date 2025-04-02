// package: ohlcv
// file: ohlcv.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";

export class GetRequest extends jspb.Message { 
    getSymId(): string;
    setSymId(value: string): GetRequest;
    getTimeframe(): string;
    setTimeframe(value: string): GetRequest;

    hasLimit(): boolean;
    clearLimit(): void;
    getLimit(): number | undefined;
    setLimit(value: number): GetRequest;

    hasOffset(): boolean;
    clearOffset(): void;
    getOffset(): number | undefined;
    setOffset(value: number): GetRequest;

    hasOrder(): boolean;
    clearOrder(): void;
    getOrder(): Order | undefined;
    setOrder(value: Order): GetRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetRequest.AsObject;
    static toObject(includeInstance: boolean, msg: GetRequest): GetRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetRequest;
    static deserializeBinaryFromReader(message: GetRequest, reader: jspb.BinaryReader): GetRequest;
}

export namespace GetRequest {
    export type AsObject = {
        symId: string,
        timeframe: string,
        limit?: number,
        offset?: number,
        order?: Order,
    }
}

export class GetResponse extends jspb.Message { 
    clearBarsList(): void;
    getBarsList(): Array<OhlcvBar>;
    setBarsList(value: Array<OhlcvBar>): GetResponse;
    addBars(value?: OhlcvBar, index?: number): OhlcvBar;
    getTotal(): number;
    setTotal(value: number): GetResponse;
    getRemaining(): number;
    setRemaining(value: number): GetResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetResponse.AsObject;
    static toObject(includeInstance: boolean, msg: GetResponse): GetResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetResponse;
    static deserializeBinaryFromReader(message: GetResponse, reader: jspb.BinaryReader): GetResponse;
}

export namespace GetResponse {
    export type AsObject = {
        barsList: Array<OhlcvBar.AsObject>,
        total: number,
        remaining: number,
    }
}

export class OhlcvBar extends jspb.Message { 

    hasOpenTime(): boolean;
    clearOpenTime(): void;
    getOpenTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setOpenTime(value?: google_protobuf_timestamp_pb.Timestamp): OhlcvBar;

    hasCloseTime(): boolean;
    clearCloseTime(): void;
    getCloseTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setCloseTime(value?: google_protobuf_timestamp_pb.Timestamp): OhlcvBar;
    getOpen(): number;
    setOpen(value: number): OhlcvBar;
    getHigh(): number;
    setHigh(value: number): OhlcvBar;
    getLow(): number;
    setLow(value: number): OhlcvBar;
    getClose(): number;
    setClose(value: number): OhlcvBar;
    getVolume(): number;
    setVolume(value: number): OhlcvBar;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): OhlcvBar.AsObject;
    static toObject(includeInstance: boolean, msg: OhlcvBar): OhlcvBar.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: OhlcvBar, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): OhlcvBar;
    static deserializeBinaryFromReader(message: OhlcvBar, reader: jspb.BinaryReader): OhlcvBar;
}

export namespace OhlcvBar {
    export type AsObject = {
        openTime?: google_protobuf_timestamp_pb.Timestamp.AsObject,
        closeTime?: google_protobuf_timestamp_pb.Timestamp.AsObject,
        open: number,
        high: number,
        low: number,
        close: number,
        volume: number,
    }
}

export enum Order {
    ASC = 0,
    DESC = 1,
}
