from typing import TYPE_CHECKING


if TYPE_CHECKING:
    from . import utils
    from .qpace_rs import *
else:
    from qpace_rs import *

# from .script import Script
