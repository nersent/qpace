// package: 
// file: sym.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";

export class GetQuery extends jspb.Message { 

    hasId(): boolean;
    clearId(): void;
    getId(): string | undefined;
    setId(value: string): GetQuery;

    hasTickerId(): boolean;
    clearTickerId(): void;
    getTickerId(): string | undefined;
    setTickerId(value: string): GetQuery;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetQuery.AsObject;
    static toObject(includeInstance: boolean, msg: GetQuery): GetQuery.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetQuery, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetQuery;
    static deserializeBinaryFromReader(message: GetQuery, reader: jspb.BinaryReader): GetQuery;
}

export namespace GetQuery {
    export type AsObject = {
        id?: string,
        tickerId?: string,
    }
}

export class GetRequest extends jspb.Message { 

    hasQuery(): boolean;
    clearQuery(): void;
    getQuery(): GetQuery | undefined;
    setQuery(value?: GetQuery): GetRequest;

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
        query?: GetQuery.AsObject,
    }
}

export class GetResponse extends jspb.Message { 

    hasSym(): boolean;
    clearSym(): void;
    getSym(): Sym | undefined;
    setSym(value?: Sym): GetResponse;

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
        sym?: Sym.AsObject,
    }
}

export class GetListRequest extends jspb.Message { 

    hasQuery(): boolean;
    clearQuery(): void;
    getQuery(): GetQuery | undefined;
    setQuery(value?: GetQuery): GetListRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetListRequest.AsObject;
    static toObject(includeInstance: boolean, msg: GetListRequest): GetListRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetListRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetListRequest;
    static deserializeBinaryFromReader(message: GetListRequest, reader: jspb.BinaryReader): GetListRequest;
}

export namespace GetListRequest {
    export type AsObject = {
        query?: GetQuery.AsObject,
    }
}

export class GetListResponse extends jspb.Message { 
    clearSymsList(): void;
    getSymsList(): Array<Sym>;
    setSymsList(value: Array<Sym>): GetListResponse;
    addSyms(value?: Sym, index?: number): Sym;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetListResponse.AsObject;
    static toObject(includeInstance: boolean, msg: GetListResponse): GetListResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetListResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetListResponse;
    static deserializeBinaryFromReader(message: GetListResponse, reader: jspb.BinaryReader): GetListResponse;
}

export namespace GetListResponse {
    export type AsObject = {
        symsList: Array<Sym.AsObject>,
    }
}

export class Sym extends jspb.Message { 
    getId(): string;
    setId(value: string): Sym;

    hasTickerId(): boolean;
    clearTickerId(): void;
    getTickerId(): string | undefined;
    setTickerId(value: string): Sym;

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

    hasCountry(): boolean;
    clearCountry(): void;
    getCountry(): string | undefined;
    setCountry(value: string): Sym;

    hasMinTick(): boolean;
    clearMinTick(): void;
    getMinTick(): number | undefined;
    setMinTick(value: number): Sym;

    hasMinQty(): boolean;
    clearMinQty(): void;
    getMinQty(): number | undefined;
    setMinQty(value: number): Sym;

    hasPriceScale(): boolean;
    clearPriceScale(): void;
    getPriceScale(): number | undefined;
    setPriceScale(value: number): Sym;

    hasPointValue(): boolean;
    clearPointValue(): void;
    getPointValue(): number | undefined;
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
        tickerId?: string,
        prefix?: string,
        currency?: string,
        baseCurrency?: string,
        ticker?: string,
        country?: string,
        minTick?: number,
        minQty?: number,
        priceScale?: number,
        pointValue?: number,
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
