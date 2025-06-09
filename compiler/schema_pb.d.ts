// package: compiler
// file: schema.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";

export class InfoRequest extends jspb.Message { 

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): InfoRequest.AsObject;
    static toObject(includeInstance: boolean, msg: InfoRequest): InfoRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: InfoRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): InfoRequest;
    static deserializeBinaryFromReader(message: InfoRequest, reader: jspb.BinaryReader): InfoRequest;
}

export namespace InfoRequest {
    export type AsObject = {
    }
}

export class InfoResponse extends jspb.Message { 
    getVersion(): string;
    setVersion(value: string): InfoResponse;

    hasBuildTime(): boolean;
    clearBuildTime(): void;
    getBuildTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setBuildTime(value?: google_protobuf_timestamp_pb.Timestamp): InfoResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): InfoResponse.AsObject;
    static toObject(includeInstance: boolean, msg: InfoResponse): InfoResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: InfoResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): InfoResponse;
    static deserializeBinaryFromReader(message: InfoResponse, reader: jspb.BinaryReader): InfoResponse;
}

export namespace InfoResponse {
    export type AsObject = {
        version: string,
        buildTime?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }
}

export class File extends jspb.Message { 
    getPath(): string;
    setPath(value: string): File;

    hasData(): boolean;
    clearData(): void;
    getData(): Uint8Array | string;
    getData_asU8(): Uint8Array;
    getData_asB64(): string;
    setData(value: Uint8Array | string): File;

    hasUrl(): boolean;
    clearUrl(): void;
    getUrl(): string | undefined;
    setUrl(value: string): File;
    getFlags(): number;
    setFlags(value: number): File;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): File.AsObject;
    static toObject(includeInstance: boolean, msg: File): File.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: File, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): File;
    static deserializeBinaryFromReader(message: File, reader: jspb.BinaryReader): File;
}

export namespace File {
    export type AsObject = {
        path: string,
        data: Uint8Array | string,
        url?: string,
        flags: number,
    }
}

export class CheckRequest extends jspb.Message { 
    getQpcConfig(): string;
    setQpcConfig(value: string): CheckRequest;
    clearFilesList(): void;
    getFilesList(): Array<File>;
    setFilesList(value: Array<File>): CheckRequest;
    addFiles(value?: File, index?: number): File;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): CheckRequest.AsObject;
    static toObject(includeInstance: boolean, msg: CheckRequest): CheckRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: CheckRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): CheckRequest;
    static deserializeBinaryFromReader(message: CheckRequest, reader: jspb.BinaryReader): CheckRequest;
}

export namespace CheckRequest {
    export type AsObject = {
        qpcConfig: string,
        filesList: Array<File.AsObject>,
    }
}

export class CheckResponse extends jspb.Message { 
    getOk(): boolean;
    setOk(value: boolean): CheckResponse;

    hasMessage(): boolean;
    clearMessage(): void;
    getMessage(): string | undefined;
    setMessage(value: string): CheckResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): CheckResponse.AsObject;
    static toObject(includeInstance: boolean, msg: CheckResponse): CheckResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: CheckResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): CheckResponse;
    static deserializeBinaryFromReader(message: CheckResponse, reader: jspb.BinaryReader): CheckResponse;
}

export namespace CheckResponse {
    export type AsObject = {
        ok: boolean,
        message?: string,
    }
}

export class BuildRequest extends jspb.Message { 
    getQpcConfig(): string;
    setQpcConfig(value: string): BuildRequest;

    hasTarget(): boolean;
    clearTarget(): void;
    getTarget(): string | undefined;
    setTarget(value: string): BuildRequest;
    clearFilesList(): void;
    getFilesList(): Array<File>;
    setFilesList(value: Array<File>): BuildRequest;
    addFiles(value?: File, index?: number): File;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): BuildRequest.AsObject;
    static toObject(includeInstance: boolean, msg: BuildRequest): BuildRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: BuildRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): BuildRequest;
    static deserializeBinaryFromReader(message: BuildRequest, reader: jspb.BinaryReader): BuildRequest;
}

export namespace BuildRequest {
    export type AsObject = {
        qpcConfig: string,
        target?: string,
        filesList: Array<File.AsObject>,
    }
}

export class BuildEvent extends jspb.Message { 

    hasCheckEnd(): boolean;
    clearCheckEnd(): void;
    getCheckEnd(): CheckEnd | undefined;
    setCheckEnd(value?: CheckEnd): BuildEvent;

    hasBuildStart(): boolean;
    clearBuildStart(): void;
    getBuildStart(): BuildStart | undefined;
    setBuildStart(value?: BuildStart): BuildEvent;

    hasBuildEnd(): boolean;
    clearBuildEnd(): void;
    getBuildEnd(): BuildEnd | undefined;
    setBuildEnd(value?: BuildEnd): BuildEvent;

    getKindCase(): BuildEvent.KindCase;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): BuildEvent.AsObject;
    static toObject(includeInstance: boolean, msg: BuildEvent): BuildEvent.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: BuildEvent, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): BuildEvent;
    static deserializeBinaryFromReader(message: BuildEvent, reader: jspb.BinaryReader): BuildEvent;
}

export namespace BuildEvent {
    export type AsObject = {
        checkEnd?: CheckEnd.AsObject,
        buildStart?: BuildStart.AsObject,
        buildEnd?: BuildEnd.AsObject,
    }

    export enum KindCase {
        KIND_NOT_SET = 0,
        CHECK_END = 1,
        BUILD_START = 2,
        BUILD_END = 3,
    }

}

export class CheckEnd extends jspb.Message { 

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): CheckEnd.AsObject;
    static toObject(includeInstance: boolean, msg: CheckEnd): CheckEnd.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: CheckEnd, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): CheckEnd;
    static deserializeBinaryFromReader(message: CheckEnd, reader: jspb.BinaryReader): CheckEnd;
}

export namespace CheckEnd {
    export type AsObject = {
    }
}

export class BuildStart extends jspb.Message { 

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): BuildStart.AsObject;
    static toObject(includeInstance: boolean, msg: BuildStart): BuildStart.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: BuildStart, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): BuildStart;
    static deserializeBinaryFromReader(message: BuildStart, reader: jspb.BinaryReader): BuildStart;
}

export namespace BuildStart {
    export type AsObject = {
    }
}

export class BuildEnd extends jspb.Message { 
    getOk(): boolean;
    setOk(value: boolean): BuildEnd;

    hasMessage(): boolean;
    clearMessage(): void;
    getMessage(): string | undefined;
    setMessage(value: string): BuildEnd;
    clearFilesList(): void;
    getFilesList(): Array<File>;
    setFilesList(value: Array<File>): BuildEnd;
    addFiles(value?: File, index?: number): File;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): BuildEnd.AsObject;
    static toObject(includeInstance: boolean, msg: BuildEnd): BuildEnd.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: BuildEnd, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): BuildEnd;
    static deserializeBinaryFromReader(message: BuildEnd, reader: jspb.BinaryReader): BuildEnd;
}

export namespace BuildEnd {
    export type AsObject = {
        ok: boolean,
        message?: string,
        filesList: Array<File.AsObject>,
    }
}

export enum FileFlag {
    FILE_FLAG_NONE = 0,
    FILE_FLAG_PYTHON_WHEEL = 1,
}
