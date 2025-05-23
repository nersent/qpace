import * as qp from "qpace";

const client = new qp.Client({
  apiKey: "ENTER_YOUR_API_KEY_HERE",
});

// Constructor
{
  const sym = qp.Sym.btc_usd();
  const ohlcv = new qp.Ohlcv();
  const ctx = new qp.Ctx(sym, ohlcv);
  const bt = new qp.Backtest(
    ctx,
    /* initial capital */ 1000.0,
    /* process orders on close */ false,
  );
}

// Fetching
{
  const ctx = await client.ctx("BITSTAMP:BTCUSD", "1D");
  const bt = new qp.Backtest(ctx.fork());
  console.log(
    bt.ctx.timeframe.toString(),
    bt.ctx.sym.tickerId,
    bt.ctx.ohlcv.length,
  );
}

// Iterating over bars
{
  while (bt.next() != null) {
    console.log(bt.ctx.barIndex, bt.ctx.bar.close);
  }
}

// Iterating over bars with explicit lifecycle
{
  while (bt.ctx.next() != null) {
    bt.onBarOpen();
    // if `bt.processOrdersOnClose` is false, orders are processed here
    {
      // your signals here
    }
    bt.onBarClose();
    // if `bt.processOrdersOnClose` is true, orders are processed here
  }
}

// Iterating over bars with signals
{
  while (bt.next() != null) {
    if (bt.ctx.barIndex == 1) {
      bt.signal(qp.Signal.short());
    } else if (bt.ctx.barIndex == 20) {
      bt.signal(qp.Signal.closeAll());
    } else if (bt.ctx.barIndex == 50) {
      bt.signal(qp.Signal.long());
    } else if (bt.ctx.barIndex == 52) {
      bt.signal(qp.Signal.closeAll());
    }
  }
  bt.print();
}

// Array Vectorized backtest
{
  const signals = [
    qp.Signal.hold(),
    null, // same as `qp.Signal.hold()`
    qp.Signal.short(),
    qp.Signal.closeAll(),
    qp.Signal.long(),
    qp.Signal.long(), // nothing happens
    qp.Signal.short(), // flips to short
  ];
  if (signals.length !== bt.ctx.length) {
    throw new Error("Signals length must be equal to ctx length");
  }
  bt.signalBatch(signals);
  bt.print();
}

// Map Vectorized backtest
{
  const signals = new Map({
    1: qp.Signal.short(),
    2: null,
    20: qp.Signal.closeAll(),
    50: qp.Signal.long(),
    52: qp.Signal.closeAll(),
  });
  bt.signalBatchMap(signals);
  bt.print();
}

// Exporting backtest to Pine
{
  console.log(bt.toPine()); // copy-pastable to other platforms
}

// Skipping bars
{
  bt.skip(); // skipps all remaining bars
  bt.skip(5); // skips 5 next bars
  bt.skipTo(20); // skips to bar index 20
}

// Backtest metrics
{
  console.log(
    bt.equity,
    bt.netEquity,
    bt.openProfit,
    bt.positionSize,
  ); /* and more */
}

// Equity curve
{
  const currentEquity = bt.equity;
  const allEquityList = bt.equityList;
}
