from datetime import datetime, timezone
import sys
import grpc
from typing import Literal, Optional, Union
import qpace_core as qp
import json
import requests
from tqdm import tqdm
from datetime import timezone
import requests
from lib.proto.ohlcv_pb2_grpc import OhlcvApiStub as OhlcvApiClient
import lib.proto.ohlcv_pb2 as ohlcv_api
from google.protobuf import timestamp_pb2

# DEFAULT_REST_ENDPOINT = "http://0.0.0.0:3000/v1"
# DEFAULT_GRPC_ENDPOINT = "0.0.0.0:3001"

DEFAULT_REST_ENDPOINT = "https://api.qpace.com"
DEFAULT_GRPC_ENDPOINT = "grpc.qpace.com"


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


class SymClient:
    def __init__(self, client: "Client"):
        self.client = client

    def get(
        self,
        pat: Optional[str] = None,
        timeframe: Optional[Union[qp.Timeframe, str]] = None,
        id: Optional[str] = None,
        ticker_id: Optional[str] = None,
        **kwargs,
    ) -> qp.Sym:
        syms = self.list(
            pat=pat,
            timeframe=timeframe,
            id=id,
            ticker_id=ticker_id,
            limit=1,
        )
        if len(syms) == 0:
            raise Exception(f"Symbol not found")
        return syms[0]

    def list(
        self,
        pat: Optional[str] = None,
        timeframe: Optional[Union[qp.Timeframe, str]] = None,
        id: Optional[str] = None,
        ticker_id: Optional[str] = None,
        limit: Optional[int] = None,
        offset: Optional[int] = None,
        **kwargs,
    ) -> list[qp.Sym]:
        if timeframe is not None:
            timeframe = str(timeframe)
        if pat is not None:
            id = pat
            ticker_id = pat
        res = self.client.http.get(
            f"{self.client.api_base}/symbols",
            params={
                "id_pat": id,
                "ticker_id_pat": ticker_id,
                "limit": limit,
                "offset": offset,
                "timeframe": timeframe,
            },
        )
        if not res.ok:
            raise Exception(f"Error: {res.status_code} {res.reason} {res.text}")
        res = res.json()
        return [qp.Sym.from_dict(sym) for sym in res["symbols"]]


class OhlcvClient:
    def __init__(self, client: "Client"):
        self.client = client
        self._ohlcv_api_client = OhlcvApiClient(self.client.grpc_channel)

    def get(
        self,
        sym: Union[Optional[str], qp.Sym],
        timeframe: Union[qp.Timeframe, str],
        limit: Optional[int] = None,
        offset: Optional[int] = None,
        order: Optional[Union[Literal["asc"], Literal["desc"]]] = None,
        start_open_time: Optional[datetime] = None,
        end_open_time: Optional[datetime] = None,
        start_close_time: Optional[datetime] = None,
        end_close_time: Optional[datetime] = None,
        pb: bool = False,
        **kwargs,
    ) -> qp.Ohlcv:
        if not isinstance(sym, qp.Sym):
            sym = self.client.sym.get(sym)
        if sym.id is None:
            raise Exception(f"Symbol has no id")
        if not isinstance(timeframe, qp.Timeframe):
            timeframe = qp.Timeframe.from_str(timeframe)
        if offset is None:
            offset = 0
        _pb: Optional[tqdm] = None
        _bars: list[ohlcv_api.OhlcvBar] = []

        # _start_time: Optional[timestamp_pb2.Timestamp] = None
        # _end_time: Optional[timestamp_pb2.Timestamp] = None
        # if start_time is not None:
        #     _start_time = timestamp_pb2.Timestamp()
        #     _start_time.FromDatetime(start_time)
        # if end_time is not None:
        #     _end_time = timestamp_pb2.Timestamp()
        #     _end_time.FromDatetime(end_time)
        _start_open_time: Optional[timestamp_pb2.Timestamp] = None
        _end_open_time: Optional[timestamp_pb2.Timestamp] = None
        _start_close_time: Optional[timestamp_pb2.Timestamp] = None
        _end_close_time: Optional[timestamp_pb2.Timestamp] = None
        if start_open_time is not None:
            _start_open_time = timestamp_pb2.Timestamp()
            _start_open_time.FromDatetime(start_open_time)
        if end_open_time is not None:
            _end_open_time = timestamp_pb2.Timestamp()
            _end_open_time.FromDatetime(end_open_time)
        if start_close_time is not None:
            _start_close_time = timestamp_pb2.Timestamp()
            _start_close_time.FromDatetime(start_close_time)
        if end_close_time is not None:
            _end_close_time = timestamp_pb2.Timestamp()
            _end_close_time.FromDatetime(end_close_time)

        _order = None
        if order == "asc":
            _order = ohlcv_api.Order.ASC
        elif order == "desc":
            _order = ohlcv_api.Order.DESC

        while True:
            req = ohlcv_api.GetRequest(
                sym_id=sym.id,
                timeframe=str(timeframe),
                limit=limit,
                offset=offset,
                order=_order,
                start_open_time=_start_open_time,
                end_open_time=_end_open_time,
                start_close_time=_start_close_time,
                end_close_time=_end_close_time,
            )
            res: ohlcv_api.GetResponse = self._ohlcv_api_client.Get(
                req, metadata=self.client._create_grpc_metadata()
            )
            bars = [proto_to_ohlcv_bar(proto) for proto in res.bars]
            remaining: int = res.remaining
            _bars.extend(bars)
            offset += len(bars)
            if remaining == 0 or limit is not None:
                break
            if pb:
                if _pb is None:
                    _pb = tqdm(
                        total=remaining + len(_bars),
                        desc=f"Loading OHLCV for {sym.id} {str(timeframe)}",
                        mininterval=1.0,
                    )
                _pb.update(len(bars))

        ohlcv = qp.Ohlcv.from_bars(_bars)
        ohlcv.timeframe = timeframe

        return ohlcv


class Client:
    def __init__(
        self,
        api_key: str,
        api_base: Optional[str] = None,
        grpc_api_base: Optional[str] = None,
        grpc_credentials: Optional[grpc.ChannelCredentials] = None,
    ):
        from . import __version__, __core__version__

        self.client_info = {}
        self.client_info["qpaceVersion"] = __version__
        self.client_info["qpaceCoreVersion"] = __core__version__

        self.api_key = api_key
        self.api_base = api_base or DEFAULT_REST_ENDPOINT
        self.grpc_api_base = grpc_api_base or DEFAULT_GRPC_ENDPOINT

        self.http = requests.Session()
        self.http.headers.update({"Content-Type": "application/json"})
        self.http.headers.update({"x-api-key": self.api_key})
        self.http.headers.update({"x-info": json.dumps(self.client_info)})
        self.http.headers.update({"Accept": "application/json"})

        grpc_options = [
            ("grpc.max_receive_message_length", -1),
            ("grpc.max_send_message_length", -1),
        ]
        grpc_secure = grpc_credentials != False
        if grpc_secure and grpc_credentials is None:
            grpc_credentials = grpc.ssl_channel_credentials()
        if grpc_secure:
            self.grpc_channel = grpc.secure_channel(
                self.grpc_api_base,
                grpc_credentials,
                options=grpc_options,
            )
        else:
            self.grpc_channel = grpc.insecure_channel(
                self.grpc_api_base,
                options=grpc_options,
            )
        self._grpc_metadata = None

        self.sym = SymClient(self)
        self.ohlcv = OhlcvClient(self)

    def _create_grpc_metadata(self) -> list[tuple[str, str]]:
        if self._grpc_metadata is None:
            metadata = {}
            metadata["x-api-key"] = self.api_key
            metadata["x-info"] = json.dumps(self.client_info)
            self._grpc_metadata = metadata
        return self._grpc_metadata.items()
