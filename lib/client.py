from qpace_core import Sym, Ohlcv, Ctx
from typing import Optional


class Client:
    def __init__(self, api_key: Optional[str] = None):
        self.api_key = api_key

    def sym(self, id: str) -> Optional[Sym]:
        return None

    def ohlcv(self, sym: Sym) -> Optional[Ohlcv]:
        pass

    def ctx(self, ticker: str) -> Ctx:
        pass
