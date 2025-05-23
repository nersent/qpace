import pandas as pd
import qpace as qp
from datetime import timezone, datetime

client = qp.Client(api_key="ENTER_YOUR_API_KEY_HERE")

# Bar
if True:
    bar = qp.OhlcvBar(
        open_time=datetime(2025, 1, 1, tzinfo=timezone.utc),
        close_time=datetime(2025, 1, 2, tzinfo=timezone.utc),
        open=100.0,
        close=110.0,
        high=115.0,
        low=95.0,
        volume=1000.0,
    )
    print(bar.open_time)
    print(bar.close)
    print(str(bar))
    print(bar.to_dict())

# Zip bars from lists
if True:
    bars = qp.zip_ohlcv_bars(
        open_time=[datetime(2025, 1, 1, tzinfo=timezone.utc)],
        close_time=[datetime(2025, 1, 2, tzinfo=timezone.utc)],
        open=[100.0],
        close=[110.0],
        high=[115.0],
        low=[95.0],
        volume=[1000.0],
    )

# Loading OHLCV dataframe from path
if True:
    ohlcv = qp.Ohlcv.read_csv("ohlcv.csv")
    ohlcv = qp.Ohlcv.read_parquet("ohlcv.parquet")
    ohlcv.timeframe = qp.Timeframe.Days(1)

# Saving OHLCV dataframe to path
if True:
    ohlcv.write_csv("ohlcv.csv")
    ohlcv.write_parquet("ohlcv.parquet")

# OHLCV dataframe from pandas
if True:
    df = pd.DataFrame(
        {
            "open_time": [datetime(2025, 1, 1, tzinfo=timezone.utc)],
            "close_time": [datetime(2025, 1, 2, tzinfo=timezone.utc)],
            "open": [100.0],
            "close": [110.0],
            "high": [115.0],
            "low": [95.0],
            "volume": [1000.0],
        }
    )

    # You may want to do the following, if you have timestamps in seconds or milliseconds. Datetime timezone info needs to be UTC.
    # df["open_time"] = pd.to_datetime(df["open_time"], utc=True, unit="s")
    # df["close_time"] = pd.to_datetime(df["open_time"], utc=True, unit="s")

    ohlcv = qp.Ohlcv.from_pandas(df)

# OHLCV dataframe from bars
if True:
    bars = [
        qp.OhlcvBar(
            open_time=datetime(2025, 1, 1, tzinfo=timezone.utc),
            close_time=datetime(2025, 1, 2, tzinfo=timezone.utc),
            open=100.0,
            close=110.0,
            high=115.0,
            low=95.0,
            volume=1000.0,
        )
    ]
    ohlcv = qp.Ohlcv.from_bars(bars)

# Fetching OHLCV dataframe
if True:
    ohlcv = client.ohlcv("BITSTAMP_BTCUSD", "1D")
    ohlcv = client.ohlcv("BITSTAMP_BTCUSD", timeframe=qp.Timeframe.Days(1))
    ohlcv = client.ohlcv(
        "BITSTAMP_BTCUSD",
        timeframe=qp.Timeframe.Days(1),
        offset=50,  # starting from bar index 50
        limit=100,  # maximum 100 bars, so last bar index will be 149
        pb=True,  # displays download progress bar
    )
if True:
    sym = client.sym("BITSTAMP_BTCUSD")
    ohlcv = client.ohlcv(sym, timeframe=qp.Timeframe.Days(1))

# Empty OHLCV dataframe
if True:
    ohlcv = qp.Ohlcv()

# Updating OHLCV dataframe
if True:
    ohlcv = qp.Ohlcv()
    ohlcv.add(qp.OhlcvBar())
    ohlcv.add_list([qp.OhlcvBar(), qp.OhlcvBar()])
