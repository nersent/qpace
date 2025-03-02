from datetime import datetime, timezone
import os
import sys

from matplotlib import pyplot as plt
import lib as qp
import pandas as pd

WORKSPACE_PATH = os.getenv("BAZED_WORKSPACE_ROOT", os.getcwd())


def load_ohlcv_df(path: str) -> pd.DataFrame:
    df: pd.DataFrame = pd.read_csv(path)
    df["open_time"] = pd.to_datetime(df["__open_time__"], utc=True, unit="s")
    df["close_time"] = pd.to_datetime(df["__close_time__"], utc=True, unit="s")
    df["open"] = df["__open__"]
    df["high"] = df["__high__"]
    df["low"] = df["__low__"]
    df["close"] = df["__close__"]
    df["volume"] = df["__volume__"]
    df = df[["open_time", "close_time", "open", "high", "low", "close", "volume"]]
    return df


if __name__ == "__main__":
    ohlcv_path = os.path.join(WORKSPACE_PATH, "playground/btc.csv")
    ohlcv_df = load_ohlcv_df(ohlcv_path)
    ohlcv = qp.Ohlcv.from_pandas(ohlcv_df)
    ctx = qp.Ctx.from_ohlcv(ohlcv, qp.SymInfo.btc_usd())

    # x = qp.ta.aroon(ctx, 14)
    # aroon_up, aroon_down = zip(*x)
    # plt.plot(ohlcv.open_time[0:90], aroon_up[0:90], label="aroon_up")
    # plt.plot(ohlcv.open_time[0:90], aroon_down[0:90], label="aroon_down")
    # plt.show()

    vi = qp.ta.vortex_indicator(ctx.fork(), 14)
    bt = qp.Backtest(ctx.fork(), qp.BacktestConfig())

    for bar_index in bt:
        if bt.ctx.bar.open_time.year < 2023:
            continue
        vi_plus, vi_minus = vi[bar_index]
        if vi_plus > vi_minus:
            bt.signal(qp.Signal.long())
        elif vi_plus < vi_minus:
            bt.signal(qp.Signal.short())
        else:
            bt.signal(qp.Signal.close_all())
    bt.print()
    # print(bt.summary())
    # print(bt.to_pine())

    # bt = qp.Backtest(ctx, qp.BacktestConfig())
    # for bar_index in bt:
    #     if bar_index == 10:
    #         print(f"[{bar_index}] short")
    #         bt.signal(qp.Signal.short())
    #     elif bar_index == 50:
    #         print(f"[{bar_index}] close all")
    #         bt.signal(qp.Signal.close_all())
    # print(bt.summary())
    # print(bt.to_pine())
    # ctx.next()
    # bt.on_bar_open()
    # print(bt.equity)
    # print(f"{ohlcv.bars[0]}")
    # bars = [
    #     qp.OhlcvBar(
    #         datetime.now(timezone.utc),
    #         datetime.now(timezone.utc),
    #         1.0,
    #         2.0,
    #         3.0,
    #         4.0,
    #         5000.0,
    #     ),
    #     qp.OhlcvBar(
    #         datetime.now(timezone.utc),
    #         datetime.now(timezone.utc),
    #         1.0,
    #         2.0,
    #         3.0,
    #         4.0,
    #         69420.0,
    #     ),
    # ]
    # ohlcv = qp.ArcOhlcv(bars)
    # ctx = qp.Ctx.from_ohlcv(ohlcv)
    # print(f"{ohlcv}")
    # ctx.ohlcv.add(qp.OhlcvBar())
    # print([f"{b}" for b in ohlcv.bars])
    # x = qp.OhlcvLoader()
    # print(qp.get_version())
