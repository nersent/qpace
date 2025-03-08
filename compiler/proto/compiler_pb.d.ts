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

export class BuildConfigEvent extends jspb.Message { 
    getQpcConfig(): string;
    setQpcConfig(value: string): BuildConfigEvent;

    hasTarget(): boolean;
    clearTarget(): void;
    getTarget(): string | undefined;
    setTarget(value: string): BuildConfigEvent;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): BuildConfigEvent.AsObject;
    static toObject(includeInstance: boolean, msg: BuildConfigEvent): BuildConfigEvent.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: BuildConfigEvent, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): BuildConfigEvent;
    static deserializeBinaryFromReader(message: BuildConfigEvent, reader: jspb.BinaryReader): BuildConfigEvent;
}

export namespace BuildConfigEvent {
    export type AsObject = {
        qpcConfig: string,
        target?: string,
    }
}

export class BuildRequestEvent extends jspb.Message { 

    hasConfig(): boolean;
    clearConfig(): void;
    getConfig(): BuildConfigEvent | undefined;
    setConfig(value?: BuildConfigEvent): BuildRequestEvent;

    hasFile(): boolean;
    clearFile(): void;
    getFile(): FileEvent | undefined;
    setFile(value?: FileEvent): BuildRequestEvent;

    hasEnd(): boolean;
    clearEnd(): void;
    getEnd(): EndEvent | undefined;
    setEnd(value?: EndEvent): BuildRequestEvent;

    getKindCase(): BuildRequestEvent.KindCase;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): BuildRequestEvent.AsObject;
    static toObject(includeInstance: boolean, msg: BuildRequestEvent): BuildRequestEvent.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: BuildRequestEvent, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): BuildRequestEvent;
    static deserializeBinaryFromReader(message: BuildRequestEvent, reader: jspb.BinaryReader): BuildRequestEvent;
}

export namespace BuildRequestEvent {
    export type AsObject = {
        config?: BuildConfigEvent.AsObject,
        file?: FileEvent.AsObject,
        end?: EndEvent.AsObject,
    }

    export enum KindCase {
        KIND_NOT_SET = 0,
        CONFIG = 1,
        FILE = 2,
        END = 3,
    }

}

export class LogEvent extends jspb.Message { 
    getMessage(): string;
    setMessage(value: string): LogEvent;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): LogEvent.AsObject;
    static toObject(includeInstance: boolean, msg: LogEvent): LogEvent.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: LogEvent, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): LogEvent;
    static deserializeBinaryFromReader(message: LogEvent, reader: jspb.BinaryReader): LogEvent;
}

export namespace LogEvent {
    export type AsObject = {
        message: string,
    }
}

export class FileEvent extends jspb.Message { 

    hasFile(): boolean;
    clearFile(): void;
    getFile(): File | undefined;
    setFile(value?: File): FileEvent;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): FileEvent.AsObject;
    static toObject(includeInstance: boolean, msg: FileEvent): FileEvent.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: FileEvent, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): FileEvent;
    static deserializeBinaryFromReader(message: FileEvent, reader: jspb.BinaryReader): FileEvent;
}

export namespace FileEvent {
    export type AsObject = {
        file?: File.AsObject,
    }
}

export class EndEvent extends jspb.Message { 
    getStatus(): Status;
    setStatus(value: Status): EndEvent;

    hasMessage(): boolean;
    clearMessage(): void;
    getMessage(): string | undefined;
    setMessage(value: string): EndEvent;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): EndEvent.AsObject;
    static toObject(includeInstance: boolean, msg: EndEvent): EndEvent.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: EndEvent, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): EndEvent;
    static deserializeBinaryFromReader(message: EndEvent, reader: jspb.BinaryReader): EndEvent;
}

export namespace EndEvent {
    export type AsObject = {
        status: Status,
        message?: string,
    }
}

export class BuildResponseEvent extends jspb.Message { 

    hasLog(): boolean;
    clearLog(): void;
    getLog(): LogEvent | undefined;
    setLog(value?: LogEvent): BuildResponseEvent;

    hasFile(): boolean;
    clearFile(): void;
    getFile(): FileEvent | undefined;
    setFile(value?: FileEvent): BuildResponseEvent;

    hasEnd(): boolean;
    clearEnd(): void;
    getEnd(): EndEvent | undefined;
    setEnd(value?: EndEvent): BuildResponseEvent;

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
        log?: LogEvent.AsObject,
        file?: FileEvent.AsObject,
        end?: EndEvent.AsObject,
    }

    export enum KindCase {
        KIND_NOT_SET = 0,
        LOG = 1,
        FILE = 2,
        END = 3,
    }

}

export enum Status {
    OK = 0,
    ERROR = 1,
}
