from qpace_core import Sym, Ohlcv, Ctx, Timeframe
from typing import Optional


class Client:
    def __init__(self, api_key: Optional[str] = None):
        self.api_key = api_key

    def ping(self):
        pass

    def sym(self, id: str) -> Sym:
        return None

    def ohlcv(self, sym: Sym) -> Ohlcv:
        pass

    def ctx(
        self,
        sym_id: Optional[str] = None,
        ticker: Optional[str] = None,
        timeframe: Optional[Timeframe] = None,
    ) -> Ctx:
        pass
