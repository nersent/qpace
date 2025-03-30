from datetime import timezone
import sys
import grpc
from typing import Optional, Union
import qpace_core as qp
import json
import requests
from tqdm import tqdm
from .internal import (
    DEFAULT_GRPC_ENDPOINT,
    DEFAULT_REST_ENDPOINT,
    proto_to_ohlcv_bar,
)
from proto.ohlcv_pb2_grpc import OhlcvApiStub as OhlcvApiClient
import proto.ohlcv_pb2 as ohlcv_api


class Client:
    def __init__(
        self,
        api_key: str,
        api_base: Optional[str] = None,
        grpc_api_base: Optional[str] = None,
        grpc_credentials: Optional[grpc.ChannelCredentials] = None,
    ):
        from . import get_version

        self.client_info = {}
        self.client_info["qpaceVersion"] = get_version()
        self.client_info["qpaceCoreVersion"] = qp.get_core_version()

        self.api_key = api_key
        self.api_base = api_base or DEFAULT_REST_ENDPOINT
        self.grpc_api_base = grpc_api_base or DEFAULT_GRPC_ENDPOINT

        self.http = requests.Session()
        self.http.headers.update({"Content-Type": "application/json"})
        self.http.headers.update({"x-api-key": self.api_key})
        self.http.headers.update({"x-info": json.dumps(self.client_info)})
        self.http.headers.update({"Accept": "application/json"})

        if grpc_credentials is None:
            grpc_credentials = grpc.ssl_channel_credentials()
        self.grpc_channel = grpc.secure_channel(
            self.grpc_api_base,
            grpc_credentials,
            options=[
                ("grpc.max_receive_message_length", -1),
                ("grpc.max_send_message_length", -1),
            ],
        )
        self._grpc_metadata = None
        self.ohlcv_api_client = OhlcvApiClient(self.grpc_channel)

    def _create_grpc_metadata(self) -> list[tuple[str, str]]:
        if self._grpc_metadata is None:
            metadata = {}
            metadata["x-api-key"] = self.api_key
            metadata["x-info"] = json.dumps(self.client_info)
            self._grpc_metadata = metadata
        return self._grpc_metadata.items()

    def sym(
        self,
        pat: Optional[str] = None,
        timeframe: Optional[Union[qp.Timeframe, str]] = None,
        id: Optional[str] = None,
        ticker_id: Optional[str] = None,
        **kwargs,
    ) -> qp.Sym:
        syms = self.syms(
            pat=pat,
            timeframe=timeframe,
            id=id,
            ticker_id=ticker_id,
            limit=1,
        )
        if len(syms) == 0:
            raise Exception(f"Symbol not found")
        return syms[0]

    def syms(
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
        res = self.http.get(
            f"{self.api_base}/symbols",
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

    def ohlcv(
        self,
        sym: Union[Optional[str], qp.Sym],
        timeframe: Union[qp.Timeframe, str],
        limit: Optional[int] = None,
        offset: Optional[int] = None,
        pb: bool = False,
        **kwargs,
    ) -> qp.Ohlcv:
        if not isinstance(timeframe, qp.Timeframe):
            timeframe = qp.Timeframe.from_str(timeframe)
        if offset is None:
            offset = 0
        _pb: Optional[tqdm] = None
        _bars: list[ohlcv_api.OhlcvBar] = []
        while True:
            res = self._ohlcv(
                sym=sym,
                timeframe=timeframe,
                limit=limit,
                offset=offset,
                **kwargs,
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

    def _ohlcv(
        self,
        sym: Union[Optional[str], qp.Sym],
        timeframe: Union[qp.Timeframe, str],
        limit: Optional[int] = None,
        offset: Optional[int] = None,
        **kwargs,
    ) -> ohlcv_api.GetResponse:
        if not isinstance(sym, qp.Sym):
            sym = self.sym(sym)
        if sym.id is None:
            raise Exception(f"Symbol has no id")
        req = ohlcv_api.GetRequest(
            sym_id=sym.id,
            timeframe=str(timeframe),
            limit=limit,
            offset=offset,
        )
        res: ohlcv_api.GetResponse = self.ohlcv_api_client.Get(
            req, metadata=self._create_grpc_metadata()
        )
        return res

    def ctx(
        self,
        sym: Union[Optional[str], qp.Sym],
        timeframe: Union[qp.Timeframe, str],
        **kwargs,
    ) -> qp.Ctx:
        if not isinstance(sym, qp.Sym):
            sym = self.sym(sym, **kwargs)
        ohlcv = self.ohlcv(
            sym=sym,
            timeframe=timeframe,
            **kwargs,
        )
        return qp.Ctx(ohlcv=ohlcv, sym=sym)
