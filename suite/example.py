import os
from time import perf_counter
from typing import Optional
import qpace as qp
import pandas as pd
import qpace_suite as qp_suite
import lightweight_charts
import numpy as np


def __trend_to_color(v: float) -> str:
    """
    Map a trend value v in [-1.0, 1.0] to an rgba colour string.

    v  < 0  => red   (negative trend)
    v  > 0  => green (positive trend)
    v == 0  => grey  (#2a2a2a)

    |v| is used as alpha/opacity.
    """
    v = max(-1.0, min(1.0, float(v)))  # clamp to [-1,1]
    alpha = abs(v)  # 0 … 1
    if alpha == 0.0:
        return "rgba(42,42,42,1.0)"  # solid grey
    if v > 0:  # green side
        r, g, b = (0, 255, 0)
    else:  # red side
        r, g, b = (255, 0, 0)
    return f"rgba({r},{g},{b},{alpha:.3f})"


if __name__ == "__main__":
    ohlcv_path = os.path.join(os.path.dirname(__file__), "../assets/btc.csv")
    ohlcv_df = pd.read_csv(
        ohlcv_path,
        parse_dates=["open_time", "close_time"],
        date_parser=lambda x: pd.to_datetime(x, utc=True, unit="s"),
    )

    ohlcv = qp.Ohlcv.from_pandas(ohlcv_df, qp.Timeframe.Days(1))
    ctx = qp.Ctx(ohlcv, qp.Sym.BTC_USD())

    # # rsi = qp.ta.rsi(ctx.copy(), ctx.ohlcv.close, 14)
    # # print(rsi[0:30])
    #
    lorentzian = qp_suite.jdehorty.machine_learning_lorentzian_classification.main(
        ctx.copy()
    )
    lorentzian_pred = np.array(lorentzian["locals"]["prediction"])

    chart = lightweight_charts.Chart()
    chart_df = ohlcv.to_pandas()
    chart_df["time"] = chart_df["open_time"]
    chart_df.drop(
        columns=[
            "open_time",
            "close_time",
            "volume",
        ],
        inplace=True,
        errors="ignore",
    )

    bar_color: Optional[pd.Series] = None
    bar_color = pd.Series(
        np.where(lorentzian_pred > 0, 1.0, np.where(lorentzian_pred < 0, -1.0, 0.0)),
        index=chart_df.index,
    )
    if bar_color is not None:
        chart_df["color"] = bar_color.map(__trend_to_color)
        chart_df["color"] = chart_df["color"].fillna(__trend_to_color(0))
        chart_df["borderColor"] = chart_df["color"]
        chart_df["wickColor"] = chart_df["color"]

    chart.set(chart_df)
    chart.show(block=True)
