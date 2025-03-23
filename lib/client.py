from datetime import timezone
import sys
import grpc
from typing import Optional, Union
import qpace_core as qp
import json
from .internal import (
    DEFAULT_GRPC_ENDPOINT,
    DEFAULT_REST_ENDPOINT,
    OhlcvFilter,
    SymFilter,
    SymQuery,
    ohlcv_filter_to_proto,
    proto_to_ohlcv_bar,
    proto_to_sym,
    sym_filter_to_proto,
    sym_query_to_proto,
)
from proto.ohlcv_pb2_grpc import OhlcvApiStub as OhlcvApiClient
from proto.sym_pb2_grpc import SymApiStub as SymApiClient
import proto.ohlcv_pb2 as ohlcv_api
import proto.sym_pb2 as sym_api


class Client:
    def __init__(
        self,
        api_key: str,
        api_base: Optional[str] = None,
        grpc_api_base: Optional[str] = None,
    ):
        self.api_key = api_key
        self.api_base = api_base or DEFAULT_REST_ENDPOINT
        self.grpc_api_base = grpc_api_base or DEFAULT_GRPC_ENDPOINT
        self.grpc_channel = grpc.insecure_channel(self.grpc_api_base)

        self.sym_api_client = SymApiClient(self.grpc_channel)
        self.ohlcv_api_client = OhlcvApiClient(self.grpc_channel)

        self._grpc_metadata = None

        self.telemetry = {}
        self.telemetry["qpaceCoreVersion"] = qp.get_version()
        self.telemetry["qpaceVersion"] = qp.get_version()

    #   private createGrpcMetadata(): grpc.Metadata {
    #     if (this._grpcMetadata == null) {
    #       const metadata = new grpc.Metadata();
    #       metadata.set("x-api-key", `${this.config.apiKey}`);
    #       if (this.telemetry != null) {
    #         metadata.set("x-qpace-telemetry", JSON.stringify(this.telemetry));
    #       }
    #       this._grpcMetadata = metadata;
    #     }
    #     return this._grpcMetadata.clone();
    #   }
    def _create_grpc_metadata(self) -> list[tuple[str, str]]:
        if self._grpc_metadata is None:
            metadata = {}
            metadata["x-api-key"] = self.api_key
            if self.telemetry:
                metadata["x-qpace-telemetry"] = json.dumps(self.telemetry)
            self._grpc_metadata = metadata
        return self._grpc_metadata.items()

    def sym(
        self,
        pat: Union[Optional[str], qp.Sym] = None,
        id: Optional[str] = None,
        ticker_id: Optional[str] = None,
        timeframe: Optional[qp.Timeframe] = None,
    ) -> qp.Sym:
        if isinstance(pat, qp.Sym):
            return pat
        syms = self.syms(
            id=id or pat,
            ticker_id=ticker_id or pat,
            timeframe=timeframe,
            limit=1,
        )
        if len(syms) == 0:
            raise Exception(f"No matching symbol found")
        return syms[0]

    def syms(
        self,
        id: Optional[str] = None,
        ticker_id: Optional[str] = None,
        timeframe: Optional[qp.Timeframe] = None,
        limit: Optional[int] = None,
        offset: Optional[int] = None,
    ) -> list[qp.Sym]:
        req = sym_api.GetRequest(
            query=sym_query_to_proto(
                SymQuery(
                    id=id,
                    ticker_id=ticker_id,
                    timeframe=timeframe,
                    limit=limit,
                    offset=offset,
                )
            ),
        )
        res: sym_api.GetResponse = self.sym_api_client.Get(
            req, metadata=self._create_grpc_metadata()
        )
        return [proto_to_sym(proto) for proto in res.syms]

    def bars(
        self,
        pat: Union[Optional[str], qp.Sym] = None,
        limit: Optional[int] = None,
        offset: Optional[int] = None,
        **kwargs,
    ) -> list[qp.OhlcvBar]:
        sym = self.sym(pat, **kwargs)
        if sym.id is None:
            raise Exception(f"Symbol has no id")
        query = ohlcv_api.Query(
            filter=ohlcv_filter_to_proto(
                OhlcvFilter(
                    sym=sym.id,
                    **kwargs,
                )
            ),
            limit=limit,
            offset=offset,
        )
        req = ohlcv_api.GetRequest(query=query)
        res: ohlcv_api.GetResponse = self.ohlcv_api_client.Get(
            req, metadata=self._create_grpc_metadata()
        )
        return [proto_to_ohlcv_bar(proto) for proto in res.bars]

    def ohlcv(
        self,
        pat: Union[Optional[str], qp.Sym, qp.Ohlcv] = None,
        limit: Optional[int] = None,
        offset: Optional[int] = None,
        **kwargs,
    ) -> qp.Ohlcv:
        if isinstance(pat, qp.Ohlcv):
            return pat
        bars = self.bars(pat, limit=limit, offset=offset, **kwargs)
        return qp.Ohlcv.from_bars(bars)

    def ctx(
        self,
        pat: Union[Optional[str], qp.Sym] = None,
        **kwargs,
    ) -> qp.Ctx:
        sym = self.sym(pat, **kwargs)
        ohlcv = self.ohlcv(pat, **kwargs)
        return qp.Ctx(sym=sym, ohlcv=ohlcv)
