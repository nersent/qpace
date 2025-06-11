

from dataclasses import dataclass
from typing import Optional, Union, Literal, TypedDict, Any, Tuple, List
from qpace import Ctx, Backtest
from qpace_script_e0ffe1 import _lib


def gowno_wdupsku(ctx: Ctx, x: List[float]) -> List[float]:
    return _lib.Incr_fn_gownoWDupsku_aae924(ctx).collect(_53_x=x)

class GownoWdupskuLocals:
    def __init__(self, inner):
        self.__inner = inner

    

    @property
    def lastValue(self) -> float:
        return self.__inner._54_lastValue()
  
      

class GownoWdupsku:
  def __init__(self, ctx: Ctx):
    self.ctx = ctx
    self.inner = _lib.Incr_fn_gownoWDupsku_aae924(ctx)
    self.locals = GownoWdupskuLocals(self.inner)

  def next(self, x: float) -> Optional[float]:
    return self.inner.next(_53_x=x)
    
        