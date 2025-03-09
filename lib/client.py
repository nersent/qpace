from datetime import timezone
import sys
import grpc
from google.protobuf.empty_pb2 import Empty
from typing import Optional, Union
from common.proto.sym_pb2_grpc import SymApiStub as SymApiClient
import common.proto.sym_pb2 as sym_api
from common.proto.ohlcv_pb2_grpc import OhlcvApiStub as OhlcvApiClient
import common.proto.ohlcv_pb2 as ohlcv_api
import pytz
import core as qp

# import qpace_core as qp

DEFAULT_REST_ENDPOINT = "http://localhost:3000/v1"
DEFAULT_GRPC_ENDPOINT = "localhost:3001"


def _map_to_qp_sym(sym: sym_api.Sym) -> qp.Sym:
    return qp.Sym(min_tick=sym.min_tick, min_qty=sym.min_qty)


def _map_to_qp_ohlcv_bar(bar: ohlcv_api.OhlcvBar) -> qp.OhlcvBar:
    open_time = bar.open_time.ToDatetime(tzinfo=timezone.utc)
    close_time = bar.close_time.ToDatetime(tzinfo=timezone.utc)
    return qp.OhlcvBar(
        open_time=open_time,
        close_time=close_time,
        open=bar.open,
        high=bar.high,
        low=bar.low,
        close=bar.close,
        volume=bar.volume,
    )


def _map_to_qp_ohlcv_bars(bars: list[ohlcv_api.OhlcvBar]) -> list[qp.OhlcvBar]:
    return [_map_to_qp_ohlcv_bar(bar) for bar in bars]


AnyTimeframe = Optional[Union[str, qp.Timeframe]]


def _map_to_qp_timeframe(timeframe: AnyTimeframe) -> qp.Timeframe:
    if timeframe is None:
        return qp.Timeframe.Unknown()
    if isinstance(timeframe, str):
        return qp.Timeframe.from_str(timeframe)
    if isinstance(timeframe, qp.Timeframe):
        return timeframe
    if isinstance(timeframe, ohlcv_api.Timeframe):
        return _map_from_qp_timeframe(timeframe)
    raise ValueError(f"Cannot map to qp.Timeframe: {timeframe}")


def _map_from_qp_timeframe(timeframe: qp.Timeframe) -> ohlcv_api.Timeframe:
    if timeframe.unknown:
        return ohlcv_api.Timeframe(unknown=Empty())
    if timeframe.years is not None:
        return ohlcv_api.Timeframe(years=timeframe.years)
    if timeframe.months is not None:
        return ohlcv_api.Timeframe(months=timeframe.months)
    if timeframe.weeks is not None:
        return ohlcv_api.Timeframe(weeks=timeframe.weeks)
    if timeframe.days is not None:
        return ohlcv_api.Timeframe(days=timeframe.days)
    if timeframe.hours is not None:
        return ohlcv_api.Timeframe(hours=timeframe.hours)
    if timeframe.minutes is not None:
        return ohlcv_api.Timeframe(minutes=timeframe.minutes)
    if timeframe.seconds is not None:
        return ohlcv_api.Timeframe(seconds=timeframe.seconds)
    if timeframe.ranges is not None:
        return ohlcv_api.Timeframe(ranges=timeframe.ranges)
    if timeframe.ticks is not None:
        return ohlcv_api.Timeframe(ticks=timeframe.ticks)
    raise ValueError(f"Cannot map from qp.Timeframe: {timeframe}")


class Client:
    def __init__(self, api_key: Optional[str] = None):
        self.api_key = api_key
        self.grpc_channel = grpc.insecure_channel(DEFAULT_GRPC_ENDPOINT)
        self.sym_api_client = SymApiClient(self.grpc_channel)
        self.ohlcv_api_client = OhlcvApiClient(self.grpc_channel)

    def ping(self):
        pass

    def sym(self, id: str) -> qp.Sym:
        req = sym_api.GetSymRequest(id=id)
        res: sym_api.GetSymResponse = self.sym_api_client.Get(req)
        return _map_to_qp_sym(res.sym)

    def bars(self, sym_id: str, timeframe: AnyTimeframe = None) -> list[qp.OhlcvBar]:
        timeframe: qp.Timeframe = _map_to_qp_timeframe(timeframe)
        req_timeframe: ohlcv_api.Timeframe = _map_from_qp_timeframe(timeframe)
        req = ohlcv_api.GetOhlcvRequest(sym_id=sym_id, timeframe=req_timeframe)
        res: ohlcv_api.GetOhlcvResponse = self.ohlcv_api_client.Get(req)
        bars = _map_to_qp_ohlcv_bars(res.bars)
        return bars

    def ohlcv(self, sym_id: str, timeframe: AnyTimeframe = None) -> qp.Ohlcv:
        bars = self.bars(sym_id, timeframe)
        return qp.Ohlcv.from_bars(bars)

    def ctx(
        self,
        sym_id: Optional[str] = None,
        timeframe: AnyTimeframe = None,
    ) -> qp.Ctx:
        timeframe: qp.Timeframe = _map_to_qp_timeframe(timeframe)
        ohlcv = self.ohlcv(sym_id, timeframe)
        sym = self.sym(sym_id)
        return qp.Ctx(ohlcv, sym, timeframe)
