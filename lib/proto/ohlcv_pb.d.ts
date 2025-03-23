// package: ohlcv
// file: ohlcv.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as sym_pb from "./sym_pb";

export class Filter extends jspb.Message { 
    getSymId(): string;
    setSymId(value: string): Filter;

    hasTimeframe(): boolean;
    clearTimeframe(): void;
    getTimeframe(): string | undefined;
    setTimeframe(value: string): Filter;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Filter.AsObject;
    static toObject(includeInstance: boolean, msg: Filter): Filter.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Filter, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Filter;
    static deserializeBinaryFromReader(message: Filter, reader: jspb.BinaryReader): Filter;
}

export namespace Filter {
    export type AsObject = {
        symId: string,
        timeframe?: string,
    }
}

export class Query extends jspb.Message { 

    hasFilter(): boolean;
    clearFilter(): void;
    getFilter(): Filter | undefined;
    setFilter(value?: Filter): Query;

    hasLimit(): boolean;
    clearLimit(): void;
    getLimit(): number | undefined;
    setLimit(value: number): Query;

    hasOffset(): boolean;
    clearOffset(): void;
    getOffset(): number | undefined;
    setOffset(value: number): Query;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Query.AsObject;
    static toObject(includeInstance: boolean, msg: Query): Query.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Query, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Query;
    static deserializeBinaryFromReader(message: Query, reader: jspb.BinaryReader): Query;
}

export namespace Query {
    export type AsObject = {
        filter?: Filter.AsObject,
        limit?: number,
        offset?: number,
    }
}

export class GetRequest extends jspb.Message { 

    hasQuery(): boolean;
    clearQuery(): void;
    getQuery(): Query | undefined;
    setQuery(value?: Query): GetRequest;

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
        query?: Query.AsObject,
    }
}

export class GetResponse extends jspb.Message { 
    clearBarsList(): void;
    getBarsList(): Array<OhlcvBar>;
    setBarsList(value: Array<OhlcvBar>): GetResponse;
    addBars(value?: OhlcvBar, index?: number): OhlcvBar;
    getTotalBars(): number;
    setTotalBars(value: number): GetResponse;

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
        totalBars: number,
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
