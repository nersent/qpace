import os
import sys
import numpy as np
import pandas as pd
import qpace as qp
import qpace_suite as qp_suite
import grpc
from time import perf_counter

if __name__ == "__main__":
    client = qp.Client(
        api_key="sk_73cc632b-b337-42a9-9e2c-5275b76873f3",
        api_base="http://127.0.0.1:3000/v1",
        grpc_api_base="127.0.0.1:3001",
        grpc_credentials=False,
    )
    sym = client.sym.get("BITSTAMP:BTCUSD")
    start_time = perf_counter()
    ohlcv = client.ohlcv.get(sym, qp.Timeframe.Minutes(15), limit=2048)
    print(f"Time taken: {perf_counter() - start_time:.2f} seconds")
    print(ohlcv[0], ohlcv[-1])
    sys.exit()
    ohlcv_path = os.path.join(os.path.dirname(__file__), "../assets/btc_12h.csv")
    ohlcv = qp.Ohlcv.read_csv(ohlcv_path)
    ohlcv.timeframe = qp.Timeframe.Days(1)
    ctx = qp.Ctx(ohlcv, qp.Sym.BTC_USD())

    lorentzian = qp_suite.jdehorty.machine_learning_lorentzian_classification.main(
        ctx.copy()
    )
    lorentzian_pred = np.array(lorentzian["locals"]["prediction"])

    rsi = qp.ta.rsi(ctx.copy(), ctx.ohlcv.close, 14)
    ema = qp.ta.ema(ctx.copy(), ctx.ohlcv.close, 14)

    #####################################
    bt = qp.Backtest(ctx.copy(), initial_capital=1000.0)
    for bar_index in bt:
        enter_long = lorentzian["locals"]["start_long_trade"][bar_index]
        enter_short = lorentzian["locals"]["start_short_trade"][bar_index]
        exit_long = lorentzian["locals"]["end_long_trade"][bar_index]
        exit_short = lorentzian["locals"]["end_short_trade"][bar_index]

        if enter_long:
            bt.signal(qp.Signal.Long())
        if enter_short:
            bt.signal(qp.Signal.Short())
        if exit_long or exit_short:
            bt.signal(qp.Signal.CloseAll())
        if bar_index == 8028:
            bt.signal(qp.Signal.Long())
            print(bt.ctx.bar.close)

    print(bt.to_pine())
    bt.display()
    #####################################

    qp.plot(
        ctx,
        [
            qp.BarPane(
                background_title="Machine Learning: Lorentzian Classification",
                bar_color=pd.Series(
                    np.where(
                        lorentzian_pred > 0,
                        "lime",
                        np.where(lorentzian_pred < 0, "red", "gray"),
                    ),
                    index=ctx.ohlcv.open_time,
                ),
                lines=[qp.Line(pd.Series(ema, ohlcv.open_time), "blue")],
                show_volume=False,
            ),
            qp.Pane(
                background_title="RSI",
                lines=[qp.Line(pd.Series(rsi, ohlcv.open_time), "yellow", 50)],
            ),
        ],
        bt=bt,
    )
