from typing import Optional
from qpace import Context
from qpace_example_python_indicator import qpace_example_python_indicator as __lib__
def my_indicator(x: list[float], length: int, ctx: Optional[Context]= None) -> list[float]:
    return __lib__.my_indicator(x,length,ctx)