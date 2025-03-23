import qpace as qp

client = qp.Client(
    api_key="ENTER_YOUR_API_KEY_HERE",
)

# Fetching symbol
if True:
    sym = client.sym("BITSTAMP:BTCUSD")
    sym = client.sym(ticker_id="BITSTAMP:BTCUSD")
    sym = client.sym("d791fa0d-19d3-4bd0-8ace-fd0e2f2db442")
    sym = client.sym(id="d791fa0d-19d3-4bd0-8ace-fd0e2f2db442")
    print(sym.id)

# Fetching multiple symbols
if True:
    syms = client.syms("BITSTAMP:*USD")
    syms = client.syms(ticker_id="BITSTAMP:*USD")
    print(syms[0].id, syms[0].ticker_id)

# Built-in static symbols
if True:
    sym = qp.Sym.btc_usd()
    sym = qp.Sym.eth_usd()
    sym = qp.Sym.sol_usd()
    print(sym.min_qty, sym.min_tick)

# Constructor
if True:
    sym = qp.Sym()
    sym.id = ""
    sym.ticker_id = ""
    sym.prefix = ""
    sym.currency = ""
    sym.base_currency = ""
    sym.ticker = ""
    sym.country = ""
    sym.min_tick = 1.0
    sym.min_qty = 0.1
    sym.price_scale = 10
    sym.point_value = 1.0
    sym.kind = "crypto"
    icon = qp.SymIcon()
    icon.mime_type = "image/png"
    icon.url = "https://example.com/icon.png"
    sym.icons = [icon]
