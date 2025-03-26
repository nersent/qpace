import * as qp from "qpace";

/*
 * Use `qpc ohlcv <id pattern or ticker id pattern --timeframe <timeframe>` command to view OHLCV data for a symbol.
 *
 * Use `qpc sym --timeframe <timeframe>` command to list all available symbols that have OHLCV at given timeframe.
 */

const client = new qp.Client({
  apiKey: "ENTER_YOUR_API_KEY_HERE",
});

// Bar
{
  const bar = new qp.OhlcvBar(
    /* open time */
    new Date("2025-01-01"),
    /* close time */
    new Date("2025-01-02"),
    /* open price */
    100.0,
    /* close price */
    110.0,
    /* high price */
    115.0,
    /* low price */
    95.0,
    /* volume */
    1000.0,
  );
  console.log(bar.openTime);
  console.log(bar.close);
  console.log(bar.toString());
  console.log(bar.toJSON());
}

// Zip bars from arrays
{
  const bars = qp.zipOhlcvBars(
    /* open time */
    [new Date("2025-01-01")],
    /* close time */
    [new Date("2025-01-02")],
    /* open price */
    Float64Array.from([100.0]),
    /* close price */
    Float64Array.from([110.0]),
    /* high price */
    Float64Array.from([115.0]),
    /* low price */
    Float64Array.from([95.0]),
    /* volume */
    Float64Array.from([1000.0]),
  );
  console.log(bars[0].close);
}

// Loading OHLCV dataframe from path
{
  const ohlcv = await qp.Ohlcv.readCSV("ohlcv.csv");
  const ohlcv = await qp.Ohlcv.readParquet("ohlcv.parquet");
}

// Saving OHLCV dataframe to path
{
  await ohlcv.writeCSV("ohlcv.csv");
  await ohlcv.writeParquet("ohlcv.parquet");
}

// OHLCV dataframe from bars
{
  const bars = [
    new qp.OhlcvBar(
      /* open time */
      new Date("2025-01-01"),
      /* close time */
      new Date("2025-01-02"),
      /* open price */
      100.0,
      /* close price */
      110.0,
      /* high price */
      115.0,
      /* low price */
      95.0,
      /* volume */
      1000.0,
    ),
  ];
  const ohlcv = new qp.Ohlcv.fromBars(bars);
  ohlcv.timeframe = qp.Timeframe.days(1);
}

// Fetching OHLCV dataframe
{
  const ohlcv = await client.ohlcv("BITSTAMP:BTCUSD", "1D");
  const ohlcv = await client.ohlcv("BITSTAMP:BTCUSD", qp.Timeframe.days(1));
  const ohlcv = await client.ohlcv("BITSTAMP:BTCUSD", qp.Timeframe.days(1), {
    offset: 50, // starting from bar index 50
    limit: 100, // maximum 100 bars, so last bar index will be 149
  });
}
{
  const sym = await client.sym("BITSTAMP:BTCUSD");
  const ohlcv = await client.ohlcv(sym, qp.Timeframe.days(1));
}

// Empty OHLCV dataframe
{
  const ohlcv = new qp.Ohlcv();
}

// Updating OHLCV dataframe
{
  const ohlcv = new qp.Ohlcv();
  ohlcv.add(new qp.OhlcvBar());
  ohlcv.addMany([new qp.OhlcvBar(), new qp.OhlcvBar()]);
}
