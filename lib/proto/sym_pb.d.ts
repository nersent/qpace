// package: sym
// file: sym.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";

export class Filter extends jspb.Message { 

    hasId(): boolean;
    clearId(): void;
    getId(): string | undefined;
    setId(value: string): Filter;

    hasTickerIdPat(): boolean;
    clearTickerIdPat(): void;
    getTickerIdPat(): string | undefined;
    setTickerIdPat(value: string): Filter;

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
        id?: string,
        tickerIdPat?: string,
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
    clearSymsList(): void;
    getSymsList(): Array<Sym>;
    setSymsList(value: Array<Sym>): GetResponse;
    addSyms(value?: Sym, index?: number): Sym;

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

    hasKind(): boolean;
    clearKind(): void;
    getKind(): string | undefined;
    setKind(value: string): Sym;

    hasJsonMetadata(): boolean;
    clearJsonMetadata(): void;
    getJsonMetadata(): string | undefined;
    setJsonMetadata(value: string): Sym;

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
        kind?: string,
        jsonMetadata?: string,
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
