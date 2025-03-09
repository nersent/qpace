// package: ohlcv
// file: ohlcv.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";

export class GetOhlcvRequest extends jspb.Message { 
    getSymId(): string;
    setSymId(value: string): GetOhlcvRequest;

    hasTimeframe(): boolean;
    clearTimeframe(): void;
    getTimeframe(): Timeframe | undefined;
    setTimeframe(value?: Timeframe): GetOhlcvRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetOhlcvRequest.AsObject;
    static toObject(includeInstance: boolean, msg: GetOhlcvRequest): GetOhlcvRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetOhlcvRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetOhlcvRequest;
    static deserializeBinaryFromReader(message: GetOhlcvRequest, reader: jspb.BinaryReader): GetOhlcvRequest;
}

export namespace GetOhlcvRequest {
    export type AsObject = {
        symId: string,
        timeframe?: Timeframe.AsObject,
    }
}

export class GetOhlcvResponse extends jspb.Message { 
    clearBarsList(): void;
    getBarsList(): Array<OhlcvBar>;
    setBarsList(value: Array<OhlcvBar>): GetOhlcvResponse;
    addBars(value?: OhlcvBar, index?: number): OhlcvBar;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetOhlcvResponse.AsObject;
    static toObject(includeInstance: boolean, msg: GetOhlcvResponse): GetOhlcvResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetOhlcvResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetOhlcvResponse;
    static deserializeBinaryFromReader(message: GetOhlcvResponse, reader: jspb.BinaryReader): GetOhlcvResponse;
}

export namespace GetOhlcvResponse {
    export type AsObject = {
        barsList: Array<OhlcvBar.AsObject>,
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

export class Timeframe extends jspb.Message { 

    hasYears(): boolean;
    clearYears(): void;
    getYears(): number;
    setYears(value: number): Timeframe;

    hasMonths(): boolean;
    clearMonths(): void;
    getMonths(): number;
    setMonths(value: number): Timeframe;

    hasWeeks(): boolean;
    clearWeeks(): void;
    getWeeks(): number;
    setWeeks(value: number): Timeframe;

    hasDays(): boolean;
    clearDays(): void;
    getDays(): number;
    setDays(value: number): Timeframe;

    hasHours(): boolean;
    clearHours(): void;
    getHours(): number;
    setHours(value: number): Timeframe;

    hasMinutes(): boolean;
    clearMinutes(): void;
    getMinutes(): number;
    setMinutes(value: number): Timeframe;

    hasSeconds(): boolean;
    clearSeconds(): void;
    getSeconds(): number;
    setSeconds(value: number): Timeframe;

    hasTicks(): boolean;
    clearTicks(): void;
    getTicks(): number;
    setTicks(value: number): Timeframe;

    hasRanges(): boolean;
    clearRanges(): void;
    getRanges(): number;
    setRanges(value: number): Timeframe;

    hasUnknown(): boolean;
    clearUnknown(): void;
    getUnknown(): google_protobuf_empty_pb.Empty | undefined;
    setUnknown(value?: google_protobuf_empty_pb.Empty): Timeframe;

    getValueCase(): Timeframe.ValueCase;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Timeframe.AsObject;
    static toObject(includeInstance: boolean, msg: Timeframe): Timeframe.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Timeframe, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Timeframe;
    static deserializeBinaryFromReader(message: Timeframe, reader: jspb.BinaryReader): Timeframe;
}

export namespace Timeframe {
    export type AsObject = {
        years: number,
        months: number,
        weeks: number,
        days: number,
        hours: number,
        minutes: number,
        seconds: number,
        ticks: number,
        ranges: number,
        unknown?: google_protobuf_empty_pb.Empty.AsObject,
    }

    export enum ValueCase {
        VALUE_NOT_SET = 0,
        YEARS = 1,
        MONTHS = 2,
        WEEKS = 3,
        DAYS = 4,
        HOURS = 5,
        MINUTES = 6,
        SECONDS = 7,
        TICKS = 8,
        RANGES = 9,
        UNKNOWN = 10,
    }

}
