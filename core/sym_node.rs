use crate::sym::{Sym, SymKind};
use napi::bindgen_prelude::*;
use napi::{Error, Result, Status};
use napi_derive::napi;

#[napi]
#[derive(Debug, Clone)]
pub struct NodeSymKind {
    inner: SymKind,
}

impl From<SymKind> for NodeSymKind {
    fn from(inner: SymKind) -> Self {
        NodeSymKind { inner }
    }
}

impl Into<SymKind> for NodeSymKind {
    fn into(self) -> SymKind {
        self.inner
    }
}

#[napi]
impl NodeSymKind {
    #[napi(js_name = toString)]
    pub fn node_to_string(&self) -> String {
        (&self.inner).into()
    }

    #[napi(js_name = fromString)]
    pub fn node_from_string(text: String) -> Self {
        SymKind::from(text).into()
    }

    #[napi(js_name = Stock)]
    pub fn node_stock() -> Self {
        SymKind::Stock.into()
    }

    #[napi(js_name = Future)]
    pub fn node_future() -> Self {
        SymKind::Future.into()
    }

    #[napi(js_name = Option)]
    pub fn node_option() -> Self {
        SymKind::Option.into()
    }

    #[napi(js_name = Forex)]
    pub fn node_forex() -> Self {
        SymKind::Forex.into()
    }

    #[napi(js_name = Crypto)]
    pub fn node_crypto() -> Self {
        SymKind::Crypto.into()
    }

    #[napi(js_name = Unknown)]
    pub fn node_unknown() -> Self {
        SymKind::Unknown.into()
    }

    #[napi(js_name = Other)]
    pub fn node_other(other: String) -> Self {
        SymKind::Other(other).into()
    }

    #[napi(js_name = eq)]
    pub fn node_eq(&self, other: &NodeSymKind) -> bool {
        self.inner == other.inner
    }
}

#[napi]
#[derive(Debug, Clone)]
pub struct NodeSym {
    inner: Sym,
}

impl Default for NodeSym {
    fn default() -> Self {
        Sym::default().into()
    }
}

impl From<Sym> for NodeSym {
    fn from(inner: Sym) -> Self {
        NodeSym { inner }
    }
}

impl Into<Sym> for NodeSym {
    fn into(self) -> Sym {
        self.inner
    }
}

#[napi]
impl NodeSym {
    #[napi(constructor)]
    pub fn new() -> Self {
        Sym::default().into()
    }

    #[napi(js_name = toString)]
    pub fn node_to_string(&self) -> String {
        (&self.inner).into()
    }

    #[napi(getter = id)]
    pub fn node_id(&self) -> Option<String> {
        self.inner.id().map(|s| s.to_string())
    }

    #[napi(setter = id)]
    pub fn node_set_id(&mut self, id: Option<String>) {
        self.inner.set_id(id);
    }

    #[napi(getter = tickerId)]
    pub fn node_ticker_id(&self) -> Option<String> {
        self.inner.ticker_id().map(|s| s.to_string())
    }

    #[napi(setter = tickerId)]
    pub fn node_set_ticker_id(&mut self, ticker_id: Option<String>) {
        self.inner.set_ticker_id(ticker_id);
    }

    #[napi(getter = kind)]
    pub fn node_kind(&self) -> NodeSymKind {
        self.inner.kind().clone().into()
    }

    #[napi(setter = kind)]
    pub fn node_set_kind(&mut self, kind: &NodeSymKind) {
        self.inner.set_kind(kind.clone().into());
    }

    #[napi(getter = minTick)]
    pub fn node_min_tick(&self) -> f64 {
        self.inner.min_tick()
    }

    #[napi(setter = minTick)]
    pub fn node_set_min_tick(&mut self, v: f64) {
        self.inner.set_min_tick(v);
    }

    #[napi(getter = minQty)]
    pub fn node_min_qty(&self) -> f64 {
        self.inner.min_qty()
    }

    #[napi(setter = minQty)]
    pub fn node_set_min_qty(&mut self, v: f64) {
        self.inner.set_min_qty(v);
    }

    #[napi(getter = prefix)]
    pub fn node_prefix(&self) -> Option<String> {
        self.inner.prefix().map(|s| s.to_string())
    }

    #[napi(setter = prefix)]
    pub fn node_set_prefix(&mut self, v: Option<String>) {
        self.inner.set_prefix(v);
    }

    #[napi(getter = currency)]
    pub fn node_currency(&self) -> Option<String> {
        self.inner.currency().map(|s| s.to_string())
    }

    #[napi(setter   = currency)]
    pub fn node_set_currency(&mut self, v: Option<String>) {
        self.inner.set_currency(v);
    }

    #[napi(getter = baseCurrency)]
    pub fn node_base_currency(&self) -> Option<String> {
        self.inner.base_currency().map(|s| s.to_string())
    }
    #[napi(setter = baseCurrency)]
    pub fn node_set_base_currency(&mut self, v: Option<String>) {
        self.inner.set_base_currency(v);
    }

    #[napi(getter = ticker)]
    pub fn node_ticker(&self) -> Option<String> {
        self.inner.ticker().map(|s| s.to_string())
    }

    #[napi(setter = ticker)]
    pub fn node_set_ticker(&mut self, v: Option<String>) {
        self.inner.set_ticker(v);
    }

    #[napi(getter = country)]
    pub fn node_country(&self) -> Option<String> {
        self.inner.country().map(|s| s.to_string())
    }

    #[napi(setter = country)]
    pub fn node_set_country(&mut self, v: Option<String>) {
        self.inner.set_country(v);
    }

    #[napi(getter = priceScale)]
    pub fn node_price_scale(&self) -> f64 {
        self.inner.price_scale()
    }

    #[napi(setter = priceScale)]
    pub fn node_set_price_scale(&mut self, v: f64) {
        self.inner.set_price_scale(v);
    }

    #[napi(getter = pointValue)]
    pub fn node_point_value(&self) -> f64 {
        self.inner.point_value()
    }

    #[napi(setter = pointValue)]
    pub fn node_set_point_value(&mut self, v: f64) {
        self.inner.set_point_value(v);
    }

    #[napi(getter = metadata)]
    pub fn node_metadata(&self) -> Option<String> {
        self.inner.metadata().map(|s| s.to_string())
    }

    #[napi(setter = metadata)]
    pub fn node_set_metadata(&mut self, v: Option<String>) {
        self.inner.set_metadata(v);
    }

    #[napi(js_name = BTC_USD)]
    pub fn node_btc_usd() -> Self {
        Sym::btc_usd().into()
    }

    #[napi(js_name = ETH_USD)]
    pub fn node_eth_usd() -> Self {
        Sym::eth_usd().into()
    }

    #[napi(js_name = SOL_USD)]
    pub fn node_sol_usd() -> Self {
        Sym::sol_usd().into()
    }

    #[napi(js_name = DOGE_USD)]
    pub fn node_doge_usd() -> Self {
        Sym::doge_usd().into()
    }

    #[napi(js_name = eq)]
    pub fn node_eq(&self, other: &NodeSym) -> bool {
        self.inner == other.inner
    }

    #[napi(js_name = toJSON)]
    pub fn node_to_json(&self, env: Env) -> Result<Object> {
        let mut obj = Object::new(&env)?;

        macro_rules! set_opt_str {
            ($getter:expr, $js_key:literal) => {
                if let Some(v) = $getter {
                    obj.set($js_key, v)?;
                } else {
                    obj.set($js_key, Null)?;
                }
            };
        }

        set_opt_str!(self.node_id(), "id");
        set_opt_str!(self.node_ticker_id(), "ticker_id");
        set_opt_str!(self.node_prefix(), "prefix");
        set_opt_str!(self.node_currency(), "currency");
        set_opt_str!(self.node_base_currency(), "base_currency");
        set_opt_str!(self.node_ticker(), "ticker");
        set_opt_str!(self.node_country(), "country");
        set_opt_str!(self.node_metadata(), "metadata");

        obj.set("min_tick", self.node_min_tick())?;
        obj.set("min_qty", self.node_min_qty())?;
        obj.set("price_scale", self.node_price_scale())?;
        obj.set("point_value", self.node_point_value())?;

        let kind_str: String = self.inner.kind().into();
        obj.set("kind", kind_str)?;

        Ok(obj)
    }

    #[napi(js_name = fromJSON)]
    pub fn node_from_json(json: Object) -> Result<NodeSym> {
        fn opt_str(json: &Object, key: &str) -> Result<Option<String>> {
            json.get(key)
        }

        let min_tick: Option<f64> = json.get("min_tick")?;
        let min_qty: Option<f64> = json.get("min_qty")?;
        let price_scale: Option<f64> = json.get("price_scale")?;
        let point_value: Option<f64> = json.get("point_value")?;

        let kind_str: Option<String> = json.get("kind")?;
        let kind = kind_str.map(SymKind::from).unwrap_or(SymKind::Unknown);

        let mut sym = Sym::default();
        sym.set_id(opt_str(&json, "id")?);
        sym.set_ticker_id(opt_str(&json, "ticker_id")?);
        sym.set_prefix(opt_str(&json, "prefix")?);
        sym.set_currency(opt_str(&json, "currency")?);
        sym.set_base_currency(opt_str(&json, "base_currency")?);
        sym.set_ticker(opt_str(&json, "ticker")?);
        sym.set_country(opt_str(&json, "country")?);
        sym.set_metadata(opt_str(&json, "metadata")?);

        sym.set_min_tick(min_tick.unwrap_or(f64::NAN));
        sym.set_min_qty(min_qty.unwrap_or(f64::NAN));
        sym.set_price_scale(price_scale.unwrap_or(f64::NAN));
        sym.set_point_value(point_value.unwrap_or(f64::NAN));
        sym.set_kind(kind);

        Ok(sym.into())
    }
}
