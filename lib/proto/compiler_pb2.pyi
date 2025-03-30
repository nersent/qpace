from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class VersionRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class VersionResponse(_message.Message):
    __slots__ = ("version", "build_time")
    VERSION_FIELD_NUMBER: _ClassVar[int]
    BUILD_TIME_FIELD_NUMBER: _ClassVar[int]
    version: str
    build_time: _timestamp_pb2.Timestamp
    def __init__(self, version: _Optional[str] = ..., build_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class File(_message.Message):
    __slots__ = ("path", "data")
    PATH_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    path: str
    data: bytes
    def __init__(self, path: _Optional[str] = ..., data: _Optional[bytes] = ...) -> None: ...

class BuildRequest(_message.Message):
    __slots__ = ("qpc_config", "target", "check_only", "files")
    QPC_CONFIG_FIELD_NUMBER: _ClassVar[int]
    TARGET_FIELD_NUMBER: _ClassVar[int]
    CHECK_ONLY_FIELD_NUMBER: _ClassVar[int]
    FILES_FIELD_NUMBER: _ClassVar[int]
    qpc_config: str
    target: str
    check_only: bool
    files: _containers.RepeatedCompositeFieldContainer[File]
    def __init__(self, qpc_config: _Optional[str] = ..., target: _Optional[str] = ..., check_only: bool = ..., files: _Optional[_Iterable[_Union[File, _Mapping]]] = ...) -> None: ...

class StageEvent(_message.Message):
    __slots__ = ("check_start", "check_end", "emit_start", "emit_end", "build_start", "build_end", "message")
    CHECK_START_FIELD_NUMBER: _ClassVar[int]
    CHECK_END_FIELD_NUMBER: _ClassVar[int]
    EMIT_START_FIELD_NUMBER: _ClassVar[int]
    EMIT_END_FIELD_NUMBER: _ClassVar[int]
    BUILD_START_FIELD_NUMBER: _ClassVar[int]
    BUILD_END_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    check_start: CheckStart
    check_end: CheckEnd
    emit_start: EmitStart
    emit_end: EmitEnd
    build_start: BuildStart
    build_end: BuildEnd
    message: str
    def __init__(self, check_start: _Optional[_Union[CheckStart, _Mapping]] = ..., check_end: _Optional[_Union[CheckEnd, _Mapping]] = ..., emit_start: _Optional[_Union[EmitStart, _Mapping]] = ..., emit_end: _Optional[_Union[EmitEnd, _Mapping]] = ..., build_start: _Optional[_Union[BuildStart, _Mapping]] = ..., build_end: _Optional[_Union[BuildEnd, _Mapping]] = ..., message: _Optional[str] = ...) -> None: ...

class CheckStart(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class CheckEnd(_message.Message):
    __slots__ = ("ok", "message")
    OK_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    ok: bool
    message: str
    def __init__(self, ok: bool = ..., message: _Optional[str] = ...) -> None: ...

class EmitStart(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class EmitEnd(_message.Message):
    __slots__ = ("ok", "message", "files")
    OK_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    FILES_FIELD_NUMBER: _ClassVar[int]
    ok: bool
    message: str
    files: _containers.RepeatedCompositeFieldContainer[File]
    def __init__(self, ok: bool = ..., message: _Optional[str] = ..., files: _Optional[_Iterable[_Union[File, _Mapping]]] = ...) -> None: ...

class BuildStart(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class BuildEnd(_message.Message):
    __slots__ = ("ok", "message", "wheel")
    OK_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    WHEEL_FIELD_NUMBER: _ClassVar[int]
    ok: bool
    message: str
    wheel: File
    def __init__(self, ok: bool = ..., message: _Optional[str] = ..., wheel: _Optional[_Union[File, _Mapping]] = ...) -> None: ...
