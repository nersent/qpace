from datetime import datetime, timezone
import os
import lib as qp
import pandas as pd

WORKSPACE_PATH = os.getenv("BAZED_WORKSPACE_ROOT", os.getcwd())


if __name__ == "__main__":
    ohlcv_path = os.path.join(WORKSPACE_PATH, "playground/btc.csv")
    ohlcv_df: pd.DataFrame = pd.read_csv(ohlcv_path)
    ohlcv_df["open_time"] = pd.to_datetime(
        ohlcv_df["__open_time__"], utc=True, unit="s"
    )
    ohlcv_df["close_time"] = pd.to_datetime(
        ohlcv_df["__close_time__"], utc=True, unit="s"
    )
    ohlcv_df["open"] = ohlcv_df["__open__"]
    ohlcv_df["high"] = ohlcv_df["__high__"]
    ohlcv_df["low"] = ohlcv_df["__low__"]
    ohlcv_df["close"] = ohlcv_df["__close__"]
    ohlcv_df["volume"] = ohlcv_df["__volume__"]
    ohlcv_df = ohlcv_df[
        ["open_time", "close_time", "open", "high", "low", "close", "volume"]
    ]
    ohlcv = qp.Ohlcv.from_pandas(ohlcv_df)

    ctx = qp.Ctx.from_ohlcv(ohlcv, qp.SymInfo.btc_usd())
    bt = qp.Backtest(ctx, qp.BacktestConfig())
    for bar_index in bt:
        if bar_index == 10:
            print(f"[{bar_index}] short")
            bt.signal(qp.Signal.short())
        elif bar_index == 20:
            print(f"[{bar_index}] close all")
            bt.signal(qp.Signal.close_all())
    print(bt.summary())
    print(bt.to_pine())
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
