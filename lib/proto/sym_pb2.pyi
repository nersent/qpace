from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Filter(_message.Message):
    __slots__ = ("id", "ticker_id_pat", "timeframe")
    ID_FIELD_NUMBER: _ClassVar[int]
    TICKER_ID_PAT_FIELD_NUMBER: _ClassVar[int]
    TIMEFRAME_FIELD_NUMBER: _ClassVar[int]
    id: str
    ticker_id_pat: str
    timeframe: str
    def __init__(self, id: _Optional[str] = ..., ticker_id_pat: _Optional[str] = ..., timeframe: _Optional[str] = ...) -> None: ...

class Query(_message.Message):
    __slots__ = ("filter", "limit", "offset")
    FILTER_FIELD_NUMBER: _ClassVar[int]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    filter: Filter
    limit: int
    offset: int
    def __init__(self, filter: _Optional[_Union[Filter, _Mapping]] = ..., limit: _Optional[int] = ..., offset: _Optional[int] = ...) -> None: ...

class GetRequest(_message.Message):
    __slots__ = ("query",)
    QUERY_FIELD_NUMBER: _ClassVar[int]
    query: Query
    def __init__(self, query: _Optional[_Union[Query, _Mapping]] = ...) -> None: ...

class GetResponse(_message.Message):
    __slots__ = ("syms",)
    SYMS_FIELD_NUMBER: _ClassVar[int]
    syms: _containers.RepeatedCompositeFieldContainer[Sym]
    def __init__(self, syms: _Optional[_Iterable[_Union[Sym, _Mapping]]] = ...) -> None: ...

class Sym(_message.Message):
    __slots__ = ("id", "ticker_id", "prefix", "currency", "base_currency", "ticker", "country", "min_tick", "min_qty", "price_scale", "point_value", "icons", "kind", "json_metadata")
    ID_FIELD_NUMBER: _ClassVar[int]
    TICKER_ID_FIELD_NUMBER: _ClassVar[int]
    PREFIX_FIELD_NUMBER: _ClassVar[int]
    CURRENCY_FIELD_NUMBER: _ClassVar[int]
    BASE_CURRENCY_FIELD_NUMBER: _ClassVar[int]
    TICKER_FIELD_NUMBER: _ClassVar[int]
    COUNTRY_FIELD_NUMBER: _ClassVar[int]
    MIN_TICK_FIELD_NUMBER: _ClassVar[int]
    MIN_QTY_FIELD_NUMBER: _ClassVar[int]
    PRICE_SCALE_FIELD_NUMBER: _ClassVar[int]
    POINT_VALUE_FIELD_NUMBER: _ClassVar[int]
    ICONS_FIELD_NUMBER: _ClassVar[int]
    KIND_FIELD_NUMBER: _ClassVar[int]
    JSON_METADATA_FIELD_NUMBER: _ClassVar[int]
    id: str
    ticker_id: str
    prefix: str
    currency: str
    base_currency: str
    ticker: str
    country: str
    min_tick: float
    min_qty: float
    price_scale: float
    point_value: float
    icons: _containers.RepeatedCompositeFieldContainer[Icon]
    kind: str
    json_metadata: str
    def __init__(self, id: _Optional[str] = ..., ticker_id: _Optional[str] = ..., prefix: _Optional[str] = ..., currency: _Optional[str] = ..., base_currency: _Optional[str] = ..., ticker: _Optional[str] = ..., country: _Optional[str] = ..., min_tick: _Optional[float] = ..., min_qty: _Optional[float] = ..., price_scale: _Optional[float] = ..., point_value: _Optional[float] = ..., icons: _Optional[_Iterable[_Union[Icon, _Mapping]]] = ..., kind: _Optional[str] = ..., json_metadata: _Optional[str] = ...) -> None: ...

class Icon(_message.Message):
    __slots__ = ("url", "mime_type")
    URL_FIELD_NUMBER: _ClassVar[int]
    MIME_TYPE_FIELD_NUMBER: _ClassVar[int]
    url: str
    mime_type: str
    def __init__(self, url: _Optional[str] = ..., mime_type: _Optional[str] = ...) -> None: ...
