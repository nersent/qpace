"""
plot_widgets.py – flexible lightweight‑charts wrapper
=====================================================================
Render one **window** containing any number of synchronised panes
(candles, OHLC, indicators).  Pane heights are specified in **CSS‑grid–style
fraction units** (`fr`).  For example `BarPane(fr=2)` together with
`Pane(fr=1)` yields a layout where the price pane occupies twice the vertical
space of the RSI pane (2 : 1 ratio).

New naming convention
---------------------
* **background_title** – faint watermark text in the middle of the pane.
* **title**            – small legend/label pinned to the pane’s *top‑left*.

Usage
-----
```python
qp.plot(
    ctx,
    panes=[
        qp.BarPane(
            background_title="BTC/USDT",          # watermark
            title="Price",                        # legend
            lines=[qp.Line(ema_series, "blue")],
            fr=2,
        ),
        qp.Pane(
            background_title="Relative Strength", # watermark
            title="RSI",                          # legend
            lines=[qp.Line(rsi_series, "yellow", 50)],
            fr=1,
        ),
    ],
)
```

`fr` is stored on every pane; the helper `_normalised_heights` converts those
relative units into floats ∈ (0,1] so they can be passed as `inner_height` to
`lightweight-charts-python`.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from datetime import datetime
from typing import List, Optional, Union, Literal

import pandas as pd
import qpace_core as qp
import numpy as np

# ───────────────────────────── Widgets ──────────────────────────────


@dataclass
class Line:
    """A time‑series plus an optional constant **mean** line."""

    data: pd.Series
    color: Optional[str] = None
    mean: Optional[float] = None


@dataclass
class Pane:
    lines: List[Line] = field(default_factory=list)
    position: Literal["top", "bottom"] = "bottom"  # future left/right support
    fr: float = 1.0  # CSS‑grid fraction unit (>0)
    background_title: Optional[str] = None  # centre watermark
    title: Optional[str] = None  # legend (top‑left)


@dataclass
class BarPane(Pane):
    price_style: Literal["candle", "ohlc"] = "candle"
    bar_color: Optional[pd.Series] = None
    show_volume: bool = True
    labels_above: Optional[pd.Series] = None
    labels_below: Optional[pd.Series] = None
    fr: float = 2.0


# ─────────────────────────── Internals ──────────────────────────────


def _normalised_heights(panes: List[Pane]) -> List[float]:
    total = sum(max(p.fr, 0.01) for p in panes)
    return [p.fr / total for p in panes]


def _series_to_df(s: pd.Series, value_col: str = "value") -> pd.DataFrame:
    if s.empty:
        return pd.DataFrame(columns=["time", value_col])
    idx = s.index
    if isinstance(idx, pd.DatetimeIndex):
        times = idx.tz_convert("UTC") if idx.tz else idx.tz_localize("UTC")
    elif pd.api.types.is_integer_dtype(idx):
        unit = "s" if idx.max() < 1_000_000_000_000 else None
        times = pd.to_datetime(idx, unit=unit, utc=True, errors="coerce")
    else:
        times = pd.to_datetime(idx, utc=True, errors="coerce")
    mask = times.notna()
    if not mask.any():
        return pd.DataFrame(columns=["time", value_col])
    df_out = pd.DataFrame({"time": times[mask], value_col: s.values[mask]})
    df_out.sort_values("time", inplace=True)
    return df_out[["time", value_col]]


def _map_color(v: Union[float, str]) -> str:
    if isinstance(v, str):
        return v
    return __trend_to_color(v)


def __trend_to_color(v: float) -> str:
    """
    Map a trend value v in [-1.0, 1.0] to an rgba colour string.

    v  < 0  => red   (negative trend)
    v  > 0  => green (positive trend)
    v == 0  => grey  (#2a2a2a)

    |v| is used as alpha/opacity.
    """
    v = float(np.clip(np.nan_to_num(v), -1.0, 1.0))
    alpha = abs(v)  # 0 … 1
    if alpha == 0.0:
        return "rgba(42,42,42,1.0)"  # solid grey
    if v > 0:  # green side
        r, g, b = (0, 255, 0)
    else:  # red side
        r, g, b = (255, 0, 0)
    return f"rgba({r},{g},{b},{alpha:.3f})"


def _safe_create_line(parent, line: Line):
    df = _series_to_df(line.data)
    if df.empty:
        return
    opts = {"width": 1, "price_line": False, "price_label": False}
    if line.color:
        opts["color"] = line.color
    parent.create_line(**opts).set(df)
    if line.mean is not None:
        baseline = df.copy()
        baseline["value"] = line.mean
        parent.create_line(
            width=1,
            style="dotted",
            price_line=False,
            price_label=False,
            color=line.color or "#666",
        ).set(baseline)


def _draw_bar(parent, df: pd.DataFrame, pane: BarPane):
    dfi = df.copy()
    if pane.bar_color is not None:
        dfi["color"] = pane.bar_color.reindex(dfi.index).fillna(0).map(_map_color)
        dfi["borderColor"] = dfi["color"]
        dfi["wickColor"] = dfi["color"]
    dfi["time"] = dfi["open_time"]
    if pane.price_style == "candle":
        cols = [
            "time",
            "open",
            "high",
            "low",
            "close",
            "volume" if pane.show_volume else None,
            *(c for c in ("color", "borderColor", "wickColor") if c in dfi.columns),
        ]
        parent.set(dfi[[c for c in cols if c]])
    else:
        parent.create_bar_series(color="#999").set(
            dfi[["time", "open", "high", "low", "close"]]
        )
        if pane.show_volume:
            vol = _series_to_df(dfi.set_index("time")["volume"])
            if not vol.empty:
                parent.create_histogram_series(price_format={"type": "volume"}).set(vol)
    for ln in pane.lines:
        _safe_create_line(parent, ln)
    _add_labels(parent, pane.labels_above, "above")
    _add_labels(parent, pane.labels_below, "below")


# ─────────────────────────── Public API ─────────────────────────────


def plot(
    ctx: qp.Ctx,
    panes: List[Pane],
    bt: Optional[Union[pd.DataFrame, qp.Backtest]] = None,
    start_time: Optional[datetime] = None,
    end_time: Optional[datetime] = None,
    display: bool = True,
    width: int = 800,
    height: int = 600,
    scale_candles_only: bool = False,
):
    import lightweight_charts as lw

    if not panes:
        raise ValueError("'panes' list cannot be empty.")
    heights = _normalised_heights(panes)
    df = ctx.ohlcv.to_pandas().copy()
    df["open_time"] = pd.to_datetime(df["open_time"], unit="s", utc=True)
    df.set_index("open_time", inplace=True, drop=False)
    if start_time is not None:
        df = df[df["open_time"] >= pd.to_datetime(start_time, utc=True)]
    if end_time is not None:
        df = df[df["open_time"] <= pd.to_datetime(end_time, utc=True)]
    if df.empty:
        raise ValueError("No data in selected interval.")
    root = lw.Chart(
        width=width,
        height=height,
        inner_width=1,
        inner_height=heights[0],
        title=panes[0].title,
        scale_candles_only=scale_candles_only,
    )
    _render_pane(root, df, panes[0])
    for pane, h in zip(panes[1:], heights[1:]):
        sub = root.create_subchart(width=1, height=h, position=pane.position, sync=True)
        _render_pane(sub, df, pane)
    if bt is not None:
        _render_trades(root, bt)
    if display:
        root.show(block=True)
    return root


def _render_pane(chart_obj, df: pd.DataFrame, pane: Pane):
    if pane.background_title:
        chart_obj.watermark(
            pane.background_title,
            font_size=48,
            color="rgba(255,255,255,0.16)",
        )
    if pane.title:
        chart_obj.legend(
            visible=True,
            ohlc=False,
            percent=False,
            lines=False,
            text=pane.title,
            color="#cccccc",
            font_size=14,
            font_family="Arial",
            color_based_on_candle=False,
        )
    if isinstance(pane, BarPane):
        _draw_bar(chart_obj, df, pane)
    else:
        for ln in pane.lines:
            _safe_create_line(chart_obj, ln)


# ─────────────────────────── Aux helpers ────────────────────────────


def _add_labels(chart, series: Optional[pd.Series], pos: str):
    if series is None:
        return
    items = [
        {
            "time": t,
            "position": pos,
            "shape": "text",
            "text": str(v),
            "color": "#ffffff",
        }
        for t, v in series.dropna().items()
    ]
    chart.marker_list(items) if items else None


def _render_trades(chart, bt: Union[pd.DataFrame, qp.Backtest]):
    if isinstance(bt, qp.Backtest):
        rows = []
        for tr in bt.trades:
            if tr.entry:
                rows.append(
                    {
                        "open_time": bt.ctx.ohlcv[tr.entry.order_bar_index].open_time,
                        "kind": 1 if tr.direction == 1 else -1,
                    }
                )
            if tr.exit:
                rows.append(
                    {
                        "open_time": bt.ctx.ohlcv[tr.exit.order_bar_index].open_time,
                        "kind": -2 if tr.direction == 1 else 2,
                    }
                )
        bt = pd.DataFrame.from_records(rows)
    if isinstance(bt, pd.DataFrame):
        mp = {
            1: {
                "position": "below",
                "shape": "arrow_up",
                "color": "green",
                "text": "Long",
            },
            -1: {
                "position": "above",
                "shape": "arrow_down",
                "color": "red",
                "text": "Short",
            },
            -2: {
                "position": "above",
                "shape": "circle",
                "color": "blue",
                "text": "Exit L",
            },
            2: {
                "position": "below",
                "shape": "circle",
                "color": "blue",
                "text": "Exit S",
            },
        }
        chart.marker_list(
            [
                {"time": r.open_time, **mp[r.kind]}
                for r in bt.itertuples(index=False)
                if r.kind in mp
            ]
        )
