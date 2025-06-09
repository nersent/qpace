from dataclasses import dataclass
from typing import Optional, Union, Literal, TypedDict, Any, Tuple, List
from qpace import Ctx, Backtest
from qpace_script_e0ffe1 import qpace_script_e0ffe1 as __lib__


def __get_qpace_core_version__() -> str:
    """
    Version of qPACE core library used in compilation.
    """
    return __lib__.get_core_version()

__checked_qpc_core__version__ = False

def __check__qpace_core_version__():
    global __checked_qpc_core__version__
    if not __checked_qpc_core__version__:
        __checked_qpc_core__version__ = True
        from qpace import get_core_version
        expected_core = get_core_version()
        actual_core = __get_qpace_core_version__()
        if expected_core != actual_core:
            import warnings 
            warnings.warn(f"Script was compiled with qpace-core {expected_core}, but you have {actual_core} installed. This may cause issues. More: https://qpace.dev/faq#qpace-core-version-warning-from-script\nTry running `pip install qpace --upgrade` to fix this.")

__check__qpace_core_version__()

from . import xd