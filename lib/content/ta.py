

from dataclasses import dataclass
from typing import Optional, Union, Literal, TypedDict, Any, Tuple, List
from qpace_core import Ctx, Backtest
from qpace_content import qpace_content as __lib__

        


class Script:
    """
    ta
    """
    
    @dataclass(frozen=True)
    class Config:
    
        pass
    
    
    class RunResult(TypedDict):
    
        pass
        

    Kind: Optional[Literal["indicator", "strategy"]] = "indicator" 

    Local = str

    def __init__(self, ctx: Ctx, config: Config = Config()):
        self.ctx = ctx
        self.config = config
        self.__inner__ = __lib__.ScriptContext__8_qpc_main_1832_6beed8(ctx, )
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
    
    
def accdist(ctx: Ctx) -> List[float]:
    return __lib__.py__10_accdist_11c3e8(ctx, )
def cum(ctx: Ctx,src: List[float]) -> List[float]:
    return __lib__.py__11_cum_cda70d(ctx, src)
def change(ctx: Ctx,src: List[float]) -> List[float]:
    return __lib__.py__13_change_86999c(ctx, src)
def barssince(ctx: Ctx,condition: List[Optional[bool]]) -> List[Optional[int]]:
    return __lib__.py__15_barssince_4843f8(ctx, condition)
def roc(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[float]:
    return __lib__.py__17_roc_cada68(ctx, src, length)
def crossover(ctx: Ctx,source1: List[float],source2: List[float]) -> List[Optional[bool]]:
    return __lib__.py__20_crossover_765544(ctx, source1, source2)
def crossunder(ctx: Ctx,source1: List[float],source2: List[float]) -> List[Optional[bool]]:
    return __lib__.py__23_crossunder_11b1fc(ctx, source1, source2)
def cross(ctx: Ctx,source1: List[float],source2: List[float]) -> List[Optional[bool]]:
    return __lib__.py__26_cross_3544aa(ctx, source1, source2)
def highestbars(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[Optional[int]]:
    return __lib__.py__29_highestbars_dda950(ctx, src, length)
def lowestbars(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[Optional[int]]:
    return __lib__.py__32_lowestbars_ac0c6c(ctx, src, length)
def highest(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[float]:
    return __lib__.py__35_highest_7004c3(ctx, src, length)
def lowest(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[float]:
    return __lib__.py__38_lowest_948224(ctx, src, length)
def swma(ctx: Ctx,src: List[float]) -> List[float]:
    return __lib__.py__41_swma_9bc39d(ctx, src)
def sma(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[float]:
    return __lib__.py__43_sma_539698(ctx, src, length)
def ema(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[float]:
    return __lib__.py__46_ema_cda730(ctx, src, length)
def rma(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[float]:
    return __lib__.py__49_rma_29445f(ctx, src, length)
def wma(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[float]:
    return __lib__.py__52_wma_f65ce1(ctx, src, length)
def lwma(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[float]:
    return __lib__.py__55_lwma_861fb5(ctx, src, length)
def hma(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[float]:
    return __lib__.py__58_hma_893fb4(ctx, src, length)
def vwma(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[float]:
    return __lib__.py__61_vwma_083794(ctx, src, length)
def dev(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[float]:
    return __lib__.py__64_dev_3840e2(ctx, src, length)
def tr(ctx: Ctx,handle_na: Optional[bool] = True) -> List[float]:
    return __lib__.py__67_tr_6fecfd(ctx, handle_na)
def atr(ctx: Ctx,length: Optional[int] = 14) -> List[float]:
    return __lib__.py__69_atr_2c80de(ctx, length)
def rsi(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[float]:
    return __lib__.py__71_rsi_9b9849(ctx, src, length)
def cci(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[float]:
    return __lib__.py__74_cci_2fd7e2(ctx, src, length)
def aroon(ctx: Ctx,length: Optional[int] = 14) -> List[Tuple[float, float]]:
    return __lib__.py__77_aroon_dc49ff(ctx, length)
def supertrend(ctx: Ctx,src: List[float],factor: float,atr_period: int) -> List[Tuple[float, Optional[int]]]:
    return __lib__.py__79_supertrend_d7065b(ctx, src, factor, atr_period)
def awesome_oscillator(ctx: Ctx,src: List[float],slow_length: Optional[int] = 5,fast_length: Optional[int] = 34) -> List[float]:
    return __lib__.py__83_awesome_oscillator_c4b143(ctx, src, slow_length, fast_length)
def balance_of_power(ctx: Ctx) -> List[float]:
    return __lib__.py__89_balance_of_power_c36d50(ctx, )
def bollinger_bands_pct_b(ctx: Ctx,src: List[float],length: Optional[int] = 20,mult: float = None) -> List[float]:
    return __lib__.py__90_bollinger_bands_pct_b_cf0090(ctx, src, length, mult)
def bollinger_bands_width(ctx: Ctx,src: List[float],length: Optional[int] = 20,mult: float = None) -> List[float]:
    return __lib__.py__99_bollinger_bands_width_935a52(ctx, src, length, mult)
def bollinger_bands(ctx: Ctx,src: List[float],length: Optional[int] = 20,mult: float = None) -> List[Tuple[float, float]]:
    return __lib__.py__108_bollinger_bands_e0db10(ctx, src, length, mult)
def chaikin_money_flow(ctx: Ctx,length: Optional[int] = 20) -> List[float]:
    return __lib__.py__116_chaikin_money_flow_93bd54(ctx, length)
def chande_kroll_stop(ctx: Ctx,atr_length: Optional[int] = 10,atr_coeff: float = None,stop_length: Optional[int] = 9) -> List[Tuple[float, float]]:
    return __lib__.py__121_chande_kroll_stop_b9c264(ctx, atr_length, atr_coeff, stop_length)
def choppiness_index(ctx: Ctx,length: Optional[int] = 14) -> List[float]:
    return __lib__.py__132_choppiness_index_b9a78e(ctx, length)
def coppock_curve(ctx: Ctx,src: List[float],wma_length: Optional[int] = 10,long_roc_length: Optional[int] = 14,short_roc_length: Optional[int] = 11) -> List[float]:
    return __lib__.py__134_coppock_curve_e71b71(ctx, src, wma_length, long_roc_length, short_roc_length)
def donchian_channel(ctx: Ctx,src: List[float],length: Optional[int] = 20) -> List[Tuple[float, float, float]]:
    return __lib__.py__139_donchian_channel_a4ae4b(ctx, src, length)
def macd(ctx: Ctx,src: List[float],short_length: Optional[int] = 12,long_length: Optional[int] = 26) -> List[float]:
    return __lib__.py__145_macd_464af1(ctx, src, short_length, long_length)
def price_oscillator(ctx: Ctx,src: List[float],short_length: Optional[int] = 12,long_length: Optional[int] = 26) -> List[float]:
    return __lib__.py__150_price_oscillator_19a52e(ctx, src, short_length, long_length)
def relative_vigor_index(ctx: Ctx,length: Optional[int] = 14) -> List[float]:
    return __lib__.py__157_relative_vigor_index_9b77cb(ctx, length)
def relative_volatility_index(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[float]:
    return __lib__.py__159_relative_volatility_index_6e8c65(ctx, src, length)
def ultimate_oscillator(ctx: Ctx,fast_length: Optional[int] = 7,medium_length: Optional[int] = 14,slow_length: Optional[int] = 28) -> List[float]:
    return __lib__.py__170_ultimate_oscillator_7d89ec(ctx, fast_length, medium_length, slow_length)
def volume_oscillator(ctx: Ctx,short_length: Optional[int] = 5,long_length: Optional[int] = 10) -> List[float]:
    return __lib__.py__182_volume_oscillator_e75f52(ctx, short_length, long_length)
def vortex_indicator(ctx: Ctx,length: Optional[int] = 14) -> List[Tuple[float, float]]:
    return __lib__.py__188_vortex_indicator_0df67c(ctx, length)
def williams_pct_r(ctx: Ctx,src: List[float],length: Optional[int] = 14) -> List[float]:
    return __lib__.py__195_williams_pct_r_fb8b2b(ctx, src, length)
__all__ = ['Script','accdist','cum','change','barssince','roc','crossover','crossunder','cross','highestbars','lowestbars','highest','lowest','swma','sma','ema','rma','wma','lwma','hma','vwma','dev','tr','atr','rsi','cci','aroon','supertrend','awesome_oscillator','balance_of_power','bollinger_bands_pct_b','bollinger_bands_width','bollinger_bands','chaikin_money_flow','chande_kroll_stop','choppiness_index','coppock_curve','donchian_channel','macd','price_oscillator','relative_vigor_index','relative_volatility_index','ultimate_oscillator','volume_oscillator','vortex_indicator','williams_pct_r']
