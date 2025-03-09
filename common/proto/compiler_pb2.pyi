from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class BuildStatus(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    OK: _ClassVar[BuildStatus]
    ERROR: _ClassVar[BuildStatus]
OK: BuildStatus
ERROR: BuildStatus

class File(_message.Message):
    __slots__ = ("path", "checksum", "tags", "data")
    PATH_FIELD_NUMBER: _ClassVar[int]
    CHECKSUM_FIELD_NUMBER: _ClassVar[int]
    TAGS_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    path: str
    checksum: str
    tags: _containers.RepeatedScalarFieldContainer[str]
    data: bytes
    def __init__(self, path: _Optional[str] = ..., checksum: _Optional[str] = ..., tags: _Optional[_Iterable[str]] = ..., data: _Optional[bytes] = ...) -> None: ...

class BuildRequest(_message.Message):
    __slots__ = ("qpc_config", "target", "files", "check_only")
    QPC_CONFIG_FIELD_NUMBER: _ClassVar[int]
    TARGET_FIELD_NUMBER: _ClassVar[int]
    FILES_FIELD_NUMBER: _ClassVar[int]
    CHECK_ONLY_FIELD_NUMBER: _ClassVar[int]
    qpc_config: str
    target: str
    files: _containers.RepeatedCompositeFieldContainer[File]
    check_only: bool
    def __init__(self, qpc_config: _Optional[str] = ..., target: _Optional[str] = ..., files: _Optional[_Iterable[_Union[File, _Mapping]]] = ..., check_only: bool = ...) -> None: ...

class BuildResponse(_message.Message):
    __slots__ = ("files", "status", "message")
    FILES_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    files: _containers.RepeatedCompositeFieldContainer[File]
    status: BuildStatus
    message: str
    def __init__(self, files: _Optional[_Iterable[_Union[File, _Mapping]]] = ..., status: _Optional[_Union[BuildStatus, str]] = ..., message: _Optional[str] = ...) -> None: ...

class BuildStartEvent(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class BuildEndEvent(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class BuildResponseEvent(_message.Message):
    __slots__ = ("message", "response", "start", "end")
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    RESPONSE_FIELD_NUMBER: _ClassVar[int]
    START_FIELD_NUMBER: _ClassVar[int]
    END_FIELD_NUMBER: _ClassVar[int]
    message: str
    response: BuildResponse
    start: BuildStartEvent
    end: BuildEndEvent
    def __init__(self, message: _Optional[str] = ..., response: _Optional[_Union[BuildResponse, _Mapping]] = ..., start: _Optional[_Union[BuildStartEvent, _Mapping]] = ..., end: _Optional[_Union[BuildEndEvent, _Mapping]] = ...) -> None: ...
