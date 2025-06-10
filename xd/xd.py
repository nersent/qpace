

from dataclasses import dataclass
from typing import Optional, Union, Literal, TypedDict, Any, Tuple, List
from qpace import Ctx, Backtest
from qpace_script_e0ffe1 import _lib


def gowno_wdupsku(ctx: Ctx, x: List[float]) -> List[float]:
    return _lib.py_fn_gownoWDupsku_19f4b6(ctx=ctx, _206_x=x)
    
        