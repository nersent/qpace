from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GetOhlcvRequest(_message.Message):
    __slots__ = ("sym_id", "timeframe")
    SYM_ID_FIELD_NUMBER: _ClassVar[int]
    TIMEFRAME_FIELD_NUMBER: _ClassVar[int]
    sym_id: str
    timeframe: Timeframe
    def __init__(self, sym_id: _Optional[str] = ..., timeframe: _Optional[_Union[Timeframe, _Mapping]] = ...) -> None: ...

class GetOhlcvResponse(_message.Message):
    __slots__ = ("bars",)
    BARS_FIELD_NUMBER: _ClassVar[int]
    bars: _containers.RepeatedCompositeFieldContainer[OhlcvBar]
    def __init__(self, bars: _Optional[_Iterable[_Union[OhlcvBar, _Mapping]]] = ...) -> None: ...

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

class Timeframe(_message.Message):
    __slots__ = ("years", "months", "weeks", "days", "hours", "minutes", "seconds", "ticks", "ranges", "unknown")
    YEARS_FIELD_NUMBER: _ClassVar[int]
    MONTHS_FIELD_NUMBER: _ClassVar[int]
    WEEKS_FIELD_NUMBER: _ClassVar[int]
    DAYS_FIELD_NUMBER: _ClassVar[int]
    HOURS_FIELD_NUMBER: _ClassVar[int]
    MINUTES_FIELD_NUMBER: _ClassVar[int]
    SECONDS_FIELD_NUMBER: _ClassVar[int]
    TICKS_FIELD_NUMBER: _ClassVar[int]
    RANGES_FIELD_NUMBER: _ClassVar[int]
    UNKNOWN_FIELD_NUMBER: _ClassVar[int]
    years: int
    months: int
    weeks: int
    days: int
    hours: int
    minutes: int
    seconds: int
    ticks: int
    ranges: int
    unknown: _empty_pb2.Empty
    def __init__(self, years: _Optional[int] = ..., months: _Optional[int] = ..., weeks: _Optional[int] = ..., days: _Optional[int] = ..., hours: _Optional[int] = ..., minutes: _Optional[int] = ..., seconds: _Optional[int] = ..., ticks: _Optional[int] = ..., ranges: _Optional[int] = ..., unknown: _Optional[_Union[_empty_pb2.Empty, _Mapping]] = ...) -> None: ...
