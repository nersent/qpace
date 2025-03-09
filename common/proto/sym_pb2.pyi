from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GetSymRequest(_message.Message):
    __slots__ = ("id",)
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class GetSymResponse(_message.Message):
    __slots__ = ("sym",)
    SYM_FIELD_NUMBER: _ClassVar[int]
    sym: Sym
    def __init__(self, sym: _Optional[_Union[Sym, _Mapping]] = ...) -> None: ...

class Sym(_message.Message):
    __slots__ = ("id", "prefix", "currency", "base_currency", "ticker", "ticker_id", "country", "min_tick", "min_qty", "price_scale", "point_value", "icons")
    ID_FIELD_NUMBER: _ClassVar[int]
    PREFIX_FIELD_NUMBER: _ClassVar[int]
    CURRENCY_FIELD_NUMBER: _ClassVar[int]
    BASE_CURRENCY_FIELD_NUMBER: _ClassVar[int]
    TICKER_FIELD_NUMBER: _ClassVar[int]
    TICKER_ID_FIELD_NUMBER: _ClassVar[int]
    COUNTRY_FIELD_NUMBER: _ClassVar[int]
    MIN_TICK_FIELD_NUMBER: _ClassVar[int]
    MIN_QTY_FIELD_NUMBER: _ClassVar[int]
    PRICE_SCALE_FIELD_NUMBER: _ClassVar[int]
    POINT_VALUE_FIELD_NUMBER: _ClassVar[int]
    ICONS_FIELD_NUMBER: _ClassVar[int]
    id: str
    prefix: str
    currency: str
    base_currency: str
    ticker: str
    ticker_id: str
    country: str
    min_tick: float
    min_qty: float
    price_scale: float
    point_value: float
    icons: _containers.RepeatedCompositeFieldContainer[Icon]
    def __init__(self, id: _Optional[str] = ..., prefix: _Optional[str] = ..., currency: _Optional[str] = ..., base_currency: _Optional[str] = ..., ticker: _Optional[str] = ..., ticker_id: _Optional[str] = ..., country: _Optional[str] = ..., min_tick: _Optional[float] = ..., min_qty: _Optional[float] = ..., price_scale: _Optional[float] = ..., point_value: _Optional[float] = ..., icons: _Optional[_Iterable[_Union[Icon, _Mapping]]] = ...) -> None: ...

class Icon(_message.Message):
    __slots__ = ("url", "mime_type")
    URL_FIELD_NUMBER: _ClassVar[int]
    MIME_TYPE_FIELD_NUMBER: _ClassVar[int]
    url: str
    mime_type: str
    def __init__(self, url: _Optional[str] = ..., mime_type: _Optional[str] = ...) -> None: ...
