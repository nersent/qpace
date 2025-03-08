

from dataclasses import dataclass
from typing import Optional, Union, Literal, TypedDict, Any, Tuple, List
from qpace import Ctx, Backtest
from qpace_artifact import qpace_artifact as __lib__

        


class Script:
    """
    
    """
    
    @dataclass(frozen=True)
    class Config:
    
        pass
    
    
    class RunResult(TypedDict):
    
        pass
        

    Kind: Optional[Literal["indicator", "strategy"]] = None 

    Local = str

    def __init__(self, ctx: Ctx, config: Config = Config()):
        self.ctx = ctx
        self.config = config
        self.__inner__ = __lib__.ScriptContext__8_qpc_main_64_3401a6(ctx, )
        self.locals = []
        self.bt: Backtest = None

    def collect(self, locals: Optional[list[Local]] = None) -> RunResult:
        if locals is not None:
            for name in locals:
                assert name in self.locals, f"Unknown local \"{name}\""
        locals = locals or self.locals
        res = self.__inner__.collect(set(locals))
        if Script.Kind == "strategy":
            self.bt = self.__inner__.get_bt()
        return res
    
    