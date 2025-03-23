import * as qp from "qpace";

/*
 * Use `qpc sym` command to list all available symbols.
 */

const client = new qp.Client({
  apiKey: "ENTER_YOUR_API_KEY_HERE",
});

// Fetching symbol
{
  let sym = await client.sym("BITSTAMP:BTCUSD");
  sym = await client.sym({ tickerId: "BITSTAMP:BTCUSD" });
  sym = await client.sym("d791fa0d-19d3-4bd0-8ace-fd0e2f2db442");
  sym = await client.sym({ id: "d791fa0d-19d3-4bd0-8ace-fd0e2f2db442" });
  console.log(sym.id, sym.tickerId);
}

// Fetching multiple symbols
{
  let syms = await client.syms("BITSTAMP:*USD");
  syms = await client.syms({ tickerId: "BITSTAMP:BTC*" });
  console.log(syms[0].id, syms[0].tickerId);
}

// Built-in static symbols
{
  let sym = qp.Sym.btc_usd();
  sym = qp.Sym.eth_usd();
  sym = qp.Sym.sol_usd();
  console.log(sym.minQty, sym.minTick);
}

/* Constructor */
{
  const sym = new qp.Sym();
  sym.id = "";
  sym.tickerId = "";
  sym.prefix = "";
  sym.currency = "";
  sym.baseCurrency = "";
  sym.ticker = "";
  sym.country = "";
  sym.minTick = 1.0;
  sym.minQty = 0.1;
  sym.priceScale = 10;
  sym.pointValue = 1.0;
  sym.kind = "crypto";
  const icon = new qp.SymIcon();
  icon.mimeType = "image/png";
  icon.url = "https://example.com/icon.png";
  sym.icons = [icon];
}
