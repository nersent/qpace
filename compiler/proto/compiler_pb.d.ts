// package: compiler
// file: compiler.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";

export class File extends jspb.Message { 
    getPath(): string;
    setPath(value: string): File;
    getChecksum(): string;
    setChecksum(value: string): File;
    clearTagsList(): void;
    getTagsList(): Array<string>;
    setTagsList(value: Array<string>): File;
    addTags(value: string, index?: number): string;
    getData(): Uint8Array | string;
    getData_asU8(): Uint8Array;
    getData_asB64(): string;
    setData(value: Uint8Array | string): File;

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
        checksum: string,
        tagsList: Array<string>,
        data: Uint8Array | string,
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

    hasCheckOnly(): boolean;
    clearCheckOnly(): void;
    getCheckOnly(): boolean | undefined;
    setCheckOnly(value: boolean): BuildRequest;

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
        checkOnly?: boolean,
    }
}

export class BuildResponse extends jspb.Message { 
    clearFilesList(): void;
    getFilesList(): Array<File>;
    setFilesList(value: Array<File>): BuildResponse;
    addFiles(value?: File, index?: number): File;
    getStatus(): BuildStatus;
    setStatus(value: BuildStatus): BuildResponse;

    hasMessage(): boolean;
    clearMessage(): void;
    getMessage(): string | undefined;
    setMessage(value: string): BuildResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): BuildResponse.AsObject;
    static toObject(includeInstance: boolean, msg: BuildResponse): BuildResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: BuildResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): BuildResponse;
    static deserializeBinaryFromReader(message: BuildResponse, reader: jspb.BinaryReader): BuildResponse;
}

export namespace BuildResponse {
    export type AsObject = {
        filesList: Array<File.AsObject>,
        status: BuildStatus,
        message?: string,
    }
}

export class BuildResponseEvent extends jspb.Message { 

    hasMessage(): boolean;
    clearMessage(): void;
    getMessage(): string;
    setMessage(value: string): BuildResponseEvent;

    hasResponse(): boolean;
    clearResponse(): void;
    getResponse(): BuildResponse | undefined;
    setResponse(value?: BuildResponse): BuildResponseEvent;

    hasStage(): boolean;
    clearStage(): void;
    getStage(): BuildStage;
    setStage(value: BuildStage): BuildResponseEvent;

    getKindCase(): BuildResponseEvent.KindCase;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): BuildResponseEvent.AsObject;
    static toObject(includeInstance: boolean, msg: BuildResponseEvent): BuildResponseEvent.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: BuildResponseEvent, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): BuildResponseEvent;
    static deserializeBinaryFromReader(message: BuildResponseEvent, reader: jspb.BinaryReader): BuildResponseEvent;
}

export namespace BuildResponseEvent {
    export type AsObject = {
        message: string,
        response?: BuildResponse.AsObject,
        stage: BuildStage,
    }

    export enum KindCase {
        KIND_NOT_SET = 0,
        MESSAGE = 1,
        RESPONSE = 2,
        STAGE = 3,
    }

}

export enum BuildStatus {
    OK = 0,
    ERROR = 1,
}

export enum BuildStage {
    START = 0,
    END = 1,
    BUILD_WAITING = 2,
    BUILD_PYTHON_WHEEL_START = 3,
    BUILD_PYTHON_WHEEL_END = 4,
}
