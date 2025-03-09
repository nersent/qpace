// package: 
// file: sym.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";

export class GetSymRequest extends jspb.Message { 
    getId(): string;
    setId(value: string): GetSymRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetSymRequest.AsObject;
    static toObject(includeInstance: boolean, msg: GetSymRequest): GetSymRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetSymRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetSymRequest;
    static deserializeBinaryFromReader(message: GetSymRequest, reader: jspb.BinaryReader): GetSymRequest;
}

export namespace GetSymRequest {
    export type AsObject = {
        id: string,
    }
}

export class GetSymResponse extends jspb.Message { 

    hasSym(): boolean;
    clearSym(): void;
    getSym(): Sym | undefined;
    setSym(value?: Sym): GetSymResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetSymResponse.AsObject;
    static toObject(includeInstance: boolean, msg: GetSymResponse): GetSymResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetSymResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetSymResponse;
    static deserializeBinaryFromReader(message: GetSymResponse, reader: jspb.BinaryReader): GetSymResponse;
}

export namespace GetSymResponse {
    export type AsObject = {
        sym?: Sym.AsObject,
    }
}

export class Sym extends jspb.Message { 
    getId(): string;
    setId(value: string): Sym;

    hasPrefix(): boolean;
    clearPrefix(): void;
    getPrefix(): string | undefined;
    setPrefix(value: string): Sym;

    hasCurrency(): boolean;
    clearCurrency(): void;
    getCurrency(): string | undefined;
    setCurrency(value: string): Sym;

    hasBaseCurrency(): boolean;
    clearBaseCurrency(): void;
    getBaseCurrency(): string | undefined;
    setBaseCurrency(value: string): Sym;

    hasTicker(): boolean;
    clearTicker(): void;
    getTicker(): string | undefined;
    setTicker(value: string): Sym;

    hasTickerId(): boolean;
    clearTickerId(): void;
    getTickerId(): string | undefined;
    setTickerId(value: string): Sym;

    hasCountry(): boolean;
    clearCountry(): void;
    getCountry(): string | undefined;
    setCountry(value: string): Sym;
    getMinTick(): number;
    setMinTick(value: number): Sym;
    getMinQty(): number;
    setMinQty(value: number): Sym;
    getPriceScale(): number;
    setPriceScale(value: number): Sym;
    getPointValue(): number;
    setPointValue(value: number): Sym;
    clearIconsList(): void;
    getIconsList(): Array<Icon>;
    setIconsList(value: Array<Icon>): Sym;
    addIcons(value?: Icon, index?: number): Icon;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Sym.AsObject;
    static toObject(includeInstance: boolean, msg: Sym): Sym.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Sym, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Sym;
    static deserializeBinaryFromReader(message: Sym, reader: jspb.BinaryReader): Sym;
}

export namespace Sym {
    export type AsObject = {
        id: string,
        prefix?: string,
        currency?: string,
        baseCurrency?: string,
        ticker?: string,
        tickerId?: string,
        country?: string,
        minTick: number,
        minQty: number,
        priceScale: number,
        pointValue: number,
        iconsList: Array<Icon.AsObject>,
    }
}

export class Icon extends jspb.Message { 
    getUrl(): string;
    setUrl(value: string): Icon;
    getMimeType(): string;
    setMimeType(value: string): Icon;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Icon.AsObject;
    static toObject(includeInstance: boolean, msg: Icon): Icon.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Icon, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Icon;
    static deserializeBinaryFromReader(message: Icon, reader: jspb.BinaryReader): Icon;
}

export namespace Icon {
    export type AsObject = {
        url: string,
        mimeType: string,
    }
}
