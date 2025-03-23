import * as qp from "qpace";

/*
 * `qp.Ctx` is used for sharing the same context between for example calculating indicators, backtesting and more.
 */

const client = new qp.Client({
  apiKey: "ENTER_YOUR_API_KEY_HERE",
});

// Constructor
{
  const sym = qp.Sym.btc_usd();
  const ohlcv = new qp.Ohlcv();
  const ctx = new qp.Ctx(ohlcv, sym);
}

// Fetching
{
  const ctx = await client.ctx("BITSTAMP:BTCUSD", {
    timeframe: qp.Timeframe.days(1),
  });
}

// `ctx.fork()` - creates a new context, seperate from the original one, but with the same symbol and ohlcv and bar position.
{
  const newCtx = ctx.fork();
  console.log(newCtx.barIndex === ctx.barIndex); // true
}

// Resetting current bar position
{
  ctx.reset();
  console.log(ctx.isInitialized); // false
  console.log(ctx.barIndex); // 0
}

// Moving to next bar
{
  const barIndex = ctx.next();
  console.log(barIndex === ctx.barIndex); // true
}

// Iterating over bars
{
  while (ctx.next() != null) {
    console.log(ctx.barIndex, ctx.bar.close);
  }
}
