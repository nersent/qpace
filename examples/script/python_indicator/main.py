import os
from matplotlib import pyplot as plt
import qpace as qp
from main_qp import my_indicator

ohlcv_loader = qp.OhlcvLoader.read_path(os.path.join("../", "btc.csv"))

values = my_indicator(ohlcv_loader.close, 60)
plt.plot(values)
plt.show()
