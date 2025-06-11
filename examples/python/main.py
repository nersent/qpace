import os
from time import perf_counter
import qpace as qp
import qpace_script_e0ffe1 as pine
from tqdm import tqdm
import numpy as np

#  const ohlcvPath = resolve(__dirname, "../btc.csv");

ohlcv_path = os.path.join(os.path.dirname(__file__), "../btc.csv")
ohlcv = qp.Ohlcv.read_csv(ohlcv_path)
ctx = qp.Ctx(ohlcv, qp.Sym.BTC_USD())

# times = []
# for i in tqdm(range(100_000), mininterval=5):
#     start_time = perf_counter()
#     pine.xd.gowno_wdupsku(ctx.copy(), ctx.ohlcv.close)
#     times.append(perf_counter() - start_time)
# print(f"mean: {np.mean(times) * 1000}ms")


script = pine.xd.GownoWdupsku(ctx.copy())
print(script.next(1))
print(script.next(69))
print(script.next(420))
print("xd", script.locals.lastValue)
# print(pine.xd.gowno_wdupsku(ctx.copy(), [1, 69, 420])[0:4])
