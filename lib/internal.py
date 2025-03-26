from typing import Optional, TypedDict, Union
from google.protobuf.empty_pb2 import Empty
from datetime import timezone
import grpc
import qpace_core as qp
import proto.ohlcv_pb2 as ohlcv_api

ENV_REST_ENDPOINT = "QPACE_API_BASE"
ENV_GRPC_ENDPOINT = "QPACE_GRPC_API_BASE"
ENV_API_KEY = "QPACE_API_KEY"
ENV_TELEMETRY = "QPACE_TELEMETRY"

DEFAULT_REST_ENDPOINT = "https://api.qpace.dev/v1"
DEFAULT_GRPC_ENDPOINT = "https://api.qpace.dev/grpc"


def proto_to_ohlcv_bar(proto: ohlcv_api.OhlcvBar) -> qp.OhlcvBar:
    open_time = proto.open_time.ToDatetime(tzinfo=timezone.utc)
    close_time = proto.close_time.ToDatetime(tzinfo=timezone.utc)
    return qp.OhlcvBar(
        open_time=open_time,
        close_time=close_time,
        open=proto.open,
        high=proto.high,
        low=proto.low,
        close=proto.close,
        volume=proto.volume,
    )
