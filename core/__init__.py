from typing import TYPE_CHECKING


if TYPE_CHECKING:
    # from . import utils
    from .qpace_core import *
else:
    from qpace_core import *

# from .script import Script
