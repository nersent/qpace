import os
from time import perf_counter
import qpace as qp
import qpace_example_lib as pine
from tqdm import tqdm
import numpy as np
import pandas as pd


#  const ohlcvPath = resolve(__dirname, "../btc.csv");

ohlcv_path = os.path.join(os.path.dirname(__file__), "../btc.csv")
ohlcv = qp.Ohlcv.read_csv(ohlcv_path)
ctx = qp.Ctx(ohlcv, qp.Sym.BTC_USD())

# df = pd.DataFrame(pine.xd.gowno(ctx.copy()), columns=["src", "ma", "dev", "cci"])
# print df with 5 digit precision
# print(df[0:10].to_string(float_format="%.5f"))

print(pine.xd.gowno(ctx.copy(), [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 69.0, 420.0])[0:15])
# print(
#     pd.DataFrame(
#         pine.xd.xd_rma(ctx.copy()), columns=["u", "d", "rs_numerator", "rs_dominator"]
#     )
# )

# print(pd.DataFrame(pine.xd.xd_rma2(ctx.copy()), columns=["u", "rs_numerator"]))

# print(pd.DataFrame(pine.xd.gowno(ctx.copy()), columns=["u", "ta.rma(u, 2)"]))
# values = pine.xd.xd(ctx.copy())
# print(values[0:15])
# times = []
# for i in tqdm(range(100_000), mininterval=5):
#     start_time = perf_counter()
#     pine.xd.gowno_wdupsku(ctx.copy(), ctx.ohlcv.close)
#     times.append(perf_counter() - start_time)
# print(f"mean: {np.mean(times) * 1000}ms")


# script = pine.xd.GownoWdupsku(ctx.copy())
# print(script.next(1))
# print(script.next(69))
# print(script.next(420))
# print("xd", script.locals.lastValue)
# print(pine.xd.gowno_wdupsku(ctx.copy(), [1, 69, 420])[0:4])
