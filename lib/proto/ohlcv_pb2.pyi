from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Order(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    ASC: _ClassVar[Order]
    DESC: _ClassVar[Order]
ASC: Order
DESC: Order

class GetRequest(_message.Message):
    __slots__ = ("sym_id", "timeframe", "limit", "offset", "order", "start_open_time", "end_open_time", "start_close_time", "end_close_time")
    SYM_ID_FIELD_NUMBER: _ClassVar[int]
    TIMEFRAME_FIELD_NUMBER: _ClassVar[int]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    ORDER_FIELD_NUMBER: _ClassVar[int]
    START_OPEN_TIME_FIELD_NUMBER: _ClassVar[int]
    END_OPEN_TIME_FIELD_NUMBER: _ClassVar[int]
    START_CLOSE_TIME_FIELD_NUMBER: _ClassVar[int]
    END_CLOSE_TIME_FIELD_NUMBER: _ClassVar[int]
    sym_id: str
    timeframe: str
    limit: int
    offset: int
    order: Order
    start_open_time: _timestamp_pb2.Timestamp
    end_open_time: _timestamp_pb2.Timestamp
    start_close_time: _timestamp_pb2.Timestamp
    end_close_time: _timestamp_pb2.Timestamp
    def __init__(self, sym_id: _Optional[str] = ..., timeframe: _Optional[str] = ..., limit: _Optional[int] = ..., offset: _Optional[int] = ..., order: _Optional[_Union[Order, str]] = ..., start_open_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., end_open_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., start_close_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., end_close_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class GetResponse(_message.Message):
    __slots__ = ("bars", "total", "remaining")
    BARS_FIELD_NUMBER: _ClassVar[int]
    TOTAL_FIELD_NUMBER: _ClassVar[int]
    REMAINING_FIELD_NUMBER: _ClassVar[int]
    bars: _containers.RepeatedCompositeFieldContainer[OhlcvBar]
    total: int
    remaining: int
    def __init__(self, bars: _Optional[_Iterable[_Union[OhlcvBar, _Mapping]]] = ..., total: _Optional[int] = ..., remaining: _Optional[int] = ...) -> None: ...

class OhlcvBar(_message.Message):
    __slots__ = ("open_time", "close_time", "open", "high", "low", "close", "volume")
    OPEN_TIME_FIELD_NUMBER: _ClassVar[int]
    CLOSE_TIME_FIELD_NUMBER: _ClassVar[int]
    OPEN_FIELD_NUMBER: _ClassVar[int]
    HIGH_FIELD_NUMBER: _ClassVar[int]
    LOW_FIELD_NUMBER: _ClassVar[int]
    CLOSE_FIELD_NUMBER: _ClassVar[int]
    VOLUME_FIELD_NUMBER: _ClassVar[int]
    open_time: _timestamp_pb2.Timestamp
    close_time: _timestamp_pb2.Timestamp
    open: float
    high: float
    low: float
    close: float
    volume: float
    def __init__(self, open_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., close_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., open: _Optional[float] = ..., high: _Optional[float] = ..., low: _Optional[float] = ..., close: _Optional[float] = ..., volume: _Optional[float] = ...) -> None: ...
