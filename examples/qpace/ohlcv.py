import os
import pandas as pd
import qpace as qp

df_path = os.path.join("btc.csv")

# Load directly from path
ohlcv_loader: qp.OhlcvLoader = qp.OhlcvLoader.read_path(df_path)

# Or convert from Pandas DataFrame
df: pd.DataFrame = pd.read_csv(df_path)
df["time"] = pd.to_datetime(df["time"], utc=True)
ohlcv_loader: qp.OhlcvLoader = qp.OhlcvLoader.from_pandas(df)

# Access all bars
close_list: list[float] = ohlcv_loader.close

# Access individual bar
bar_index = 1
close: float = ohlcv_loader.close[bar_index]
bar: qp.OhlcvBar = ohlcv_loader.bar[bar_index]
close = bar.close
