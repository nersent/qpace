// package: compiler
// file: compiler.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";

export class File extends jspb.Message { 
    getPath(): string;
    setPath(value: string): File;
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

    hasCheckOnly(): boolean;
    clearCheckOnly(): void;
    getCheckOnly(): boolean | undefined;
    setCheckOnly(value: boolean): BuildRequest;
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
        checkOnly?: boolean,
        filesList: Array<File.AsObject>,
    }
}

export class StageEvent extends jspb.Message { 

    hasCheckStart(): boolean;
    clearCheckStart(): void;
    getCheckStart(): CheckStart | undefined;
    setCheckStart(value?: CheckStart): StageEvent;

    hasCheckEnd(): boolean;
    clearCheckEnd(): void;
    getCheckEnd(): CheckEnd | undefined;
    setCheckEnd(value?: CheckEnd): StageEvent;

    hasEmitStart(): boolean;
    clearEmitStart(): void;
    getEmitStart(): EmitStart | undefined;
    setEmitStart(value?: EmitStart): StageEvent;

    hasEmitEnd(): boolean;
    clearEmitEnd(): void;
    getEmitEnd(): EmitEnd | undefined;
    setEmitEnd(value?: EmitEnd): StageEvent;

    hasBuildStart(): boolean;
    clearBuildStart(): void;
    getBuildStart(): BuildStart | undefined;
    setBuildStart(value?: BuildStart): StageEvent;

    hasBuildEnd(): boolean;
    clearBuildEnd(): void;
    getBuildEnd(): BuildEnd | undefined;
    setBuildEnd(value?: BuildEnd): StageEvent;

    hasMessage(): boolean;
    clearMessage(): void;
    getMessage(): string;
    setMessage(value: string): StageEvent;

    getKindCase(): StageEvent.KindCase;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): StageEvent.AsObject;
    static toObject(includeInstance: boolean, msg: StageEvent): StageEvent.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: StageEvent, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): StageEvent;
    static deserializeBinaryFromReader(message: StageEvent, reader: jspb.BinaryReader): StageEvent;
}

export namespace StageEvent {
    export type AsObject = {
        checkStart?: CheckStart.AsObject,
        checkEnd?: CheckEnd.AsObject,
        emitStart?: EmitStart.AsObject,
        emitEnd?: EmitEnd.AsObject,
        buildStart?: BuildStart.AsObject,
        buildEnd?: BuildEnd.AsObject,
        message: string,
    }

    export enum KindCase {
        KIND_NOT_SET = 0,
        CHECK_START = 1,
        CHECK_END = 2,
        EMIT_START = 3,
        EMIT_END = 4,
        BUILD_START = 5,
        BUILD_END = 6,
        MESSAGE = 7,
    }

}

export class CheckStart extends jspb.Message { 

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): CheckStart.AsObject;
    static toObject(includeInstance: boolean, msg: CheckStart): CheckStart.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: CheckStart, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): CheckStart;
    static deserializeBinaryFromReader(message: CheckStart, reader: jspb.BinaryReader): CheckStart;
}

export namespace CheckStart {
    export type AsObject = {
    }
}

export class CheckEnd extends jspb.Message { 
    getOk(): boolean;
    setOk(value: boolean): CheckEnd;

    hasMessage(): boolean;
    clearMessage(): void;
    getMessage(): string | undefined;
    setMessage(value: string): CheckEnd;

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
        ok: boolean,
        message?: string,
    }
}

export class EmitStart extends jspb.Message { 

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): EmitStart.AsObject;
    static toObject(includeInstance: boolean, msg: EmitStart): EmitStart.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: EmitStart, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): EmitStart;
    static deserializeBinaryFromReader(message: EmitStart, reader: jspb.BinaryReader): EmitStart;
}

export namespace EmitStart {
    export type AsObject = {
    }
}

export class EmitEnd extends jspb.Message { 
    getOk(): boolean;
    setOk(value: boolean): EmitEnd;

    hasMessage(): boolean;
    clearMessage(): void;
    getMessage(): string | undefined;
    setMessage(value: string): EmitEnd;
    clearFilesList(): void;
    getFilesList(): Array<File>;
    setFilesList(value: Array<File>): EmitEnd;
    addFiles(value?: File, index?: number): File;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): EmitEnd.AsObject;
    static toObject(includeInstance: boolean, msg: EmitEnd): EmitEnd.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: EmitEnd, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): EmitEnd;
    static deserializeBinaryFromReader(message: EmitEnd, reader: jspb.BinaryReader): EmitEnd;
}

export namespace EmitEnd {
    export type AsObject = {
        ok: boolean,
        message?: string,
        filesList: Array<File.AsObject>,
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

    hasWheel(): boolean;
    clearWheel(): void;
    getWheel(): File | undefined;
    setWheel(value?: File): BuildEnd;

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
        wheel?: File.AsObject,
    }
}
