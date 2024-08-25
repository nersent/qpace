from datetime import datetime
import sys
import pandas as pd
import qpace as qp
import numpy as np

df: pd.DataFrame = pd.read_csv("btc.csv")
sym: qp.Symbol = qp.Symbol.btc_usd()
dp: qp.DataProvider = qp.DataProvider.from_pandas(df, sym)

TRAIN_START_TIME_MS = datetime(2017, 1, 1)
TRAIN_END_TIME = datetime(2022, 12, 31)
train_dp = dp.clone(
    start_time=TRAIN_START_TIME_MS,
    end_time=TRAIN_END_TIME,
)
val_dp = dp.clone(
    start_time=TRAIN_END_TIME,
)

optimizer = qp.optimization.GeneticOptimizer(
    params={
        "aroon_weight": qp.optimization.Param.number().set_range(0.0, 1.0, 0.1),
        "aroon_length": qp.optimization.Param.number().set_range(2, 30, 4),
        #
        "rsi_weight": qp.optimization.Param.number().set_range(0.0, 1.0, 0.1),
        "rsi_length": qp.optimization.Param.number().set_range(2, 30, 4),
        "rsi_overbought": qp.optimization.Param.number().set_range(70, 100, 5),
        "rsi_oversold": qp.optimization.Param.number().set_range(0, 30, 5),
        #
        "bop_weight": qp.optimization.Param.number().set_range(0.0, 1.0, 0.1),
        "bop_overbought": qp.optimization.Param.number().set_range(0.5, 1.0, 0.1),
        "bop_oversold": qp.optimization.Param.number().set_range(-1.0, -0.5, 0.1),
    },
    options={
        "generations": 1024,
    },
)


def backtest(dp: qp.DataProvider, params) -> qp.Backtest:
    aroon_up, aroon_down = qp.ta.aroon(dp, length=int(params["aroon_length"]))
    rsi = qp.ta.relative_strength_index(dp, length=int(params["rsi_length"]))
    bop = qp.ta.balance_of_power(dp)

    aroon_up = np.array(aroon_up)
    aroon_down = np.array(aroon_down)
    rsi = np.array(rsi)
    bop = np.array(bop)

    aroon_overbought = np.where(aroon_up > aroon_down, 1, 0)
    aroon_oversold = np.where(aroon_up < aroon_down, -1, 0)
    rsi_overbought = np.where(rsi > params["rsi_overbought"], 1, 0)
    rsi_oversold = np.where(rsi < params["rsi_oversold"], -1, 0)
    bop_overbought = np.where(bop > params["bop_overbought"], 1, 0)
    bop_oversold = np.where(bop < params["bop_oversold"], -1, 0)

    avg = np.mean(
        [
            aroon_overbought * params["aroon_weight"],
            aroon_oversold * params["aroon_weight"],
            rsi_overbought * params["rsi_weight"],
            rsi_oversold * params["rsi_weight"],
            bop_overbought * params["bop_weight"],
            bop_oversold * params["bop_weight"],
        ],
        axis=0,
    )

    bt = qp.Backtest(dp, initial_capital=1000.0)
    bt.signal_batch(
        long=(avg > 0).astype(bool).tolist(),
        short=(avg < 0).astype(bool).tolist(),
    )

    return bt


def loss_fn(params, *kwargs) -> float:
    bt = backtest(train_dp, params)
    return np.nan_to_num(bt.expectancy, nan=0)


res = optimizer.fit(loss_fn, verbose=True)
best_params = res.best.params
print(best_params)

train_bt = backtest(train_dp, best_params)
train_bt.print()

val_bt = backtest(val_dp, best_params)
val_bt.print()

res.plot()
