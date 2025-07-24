use crate::sym::{Sym, SymKind};
use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "SymKind")]
#[derive(Debug, Clone)]
pub struct WasmSymKind {
    inner: SymKind,
}

impl From<SymKind> for WasmSymKind {
    #[inline]
    fn from(inner: SymKind) -> Self {
        Self { inner }
    }
}

impl Into<SymKind> for WasmSymKind {
    #[inline]
    fn into(self) -> SymKind {
        self.inner
    }
}

impl Default for WasmSymKind {
    #[inline]
    fn default() -> Self {
        SymKind::default().into()
    }
}

#[wasm_bindgen(js_class = SymKind)]
impl WasmSymKind {
    #[wasm_bindgen(js_name = toString)]
    #[inline]
    pub fn wasm_to_string(&self) -> String {
        (&self.inner).into()
    }

    #[wasm_bindgen(js_name = fromString)]
    #[inline]
    pub fn wasm_from_string(kind: String) -> Self {
        SymKind::from(kind).into()
    }

    #[wasm_bindgen(js_name = Stock)]
    #[inline]
    pub fn wasm_stock() -> Self {
        SymKind::Stock.into()
    }

    #[wasm_bindgen(js_name = Future)]
    #[inline]
    pub fn wasm_future() -> Self {
        SymKind::Future.into()
    }

    #[wasm_bindgen(js_name = Option)]
    #[inline]
    pub fn wasm_option() -> Self {
        SymKind::Option.into()
    }

    #[wasm_bindgen(js_name = Crypto)]
    #[inline]
    pub fn wasm_crypto() -> Self {
        SymKind::Crypto.into()
    }

    #[wasm_bindgen(js_name = Forex)]
    #[inline]
    pub fn wasm_forex() -> Self {
        SymKind::Forex.into()
    }

    #[wasm_bindgen(js_name = Unknown)]
    #[inline]
    pub fn wasm_unknown() -> Self {
        SymKind::Unknown.into()
    }

    #[wasm_bindgen(js_name = Other)]
    #[inline]
    pub fn wasm_other(kind: String) -> Self {
        SymKind::Other(kind).into()
    }

    #[wasm_bindgen(js_name = eq)]
    #[inline]
    pub fn wasm_eq(&self, other: WasmSymKind) -> bool {
        self.inner == other.inner
    }
}

#[wasm_bindgen(js_name = "Sym")]
#[derive(Debug, Clone)]
pub struct WasmSym {
    inner: Sym,
}

impl Default for WasmSym {
    #[inline]
    fn default() -> Self {
        Sym::default().into()
    }
}

impl From<Sym> for WasmSym {
    #[inline]
    fn from(inner: Sym) -> Self {
        Self { inner }
    }
}

impl Into<Sym> for WasmSym {
    #[inline]
    fn into(self) -> Sym {
        self.inner
    }
}

impl Into<JsValue> for &WasmSym {
    #[inline]
    fn into(self) -> JsValue {
        let obj = Object::new();
        fn set_opt_string(obj: &Object, key: &str, val: Option<&str>) {
            let v = match val {
                Some(s) => JsValue::from_str(s),
                None => JsValue::NULL,
            };
            let _ = Reflect::set(obj, &key.into(), &v);
        }
        set_opt_string(&obj, "id", self.inner.id());
        set_opt_string(&obj, "ticker_id", self.inner.ticker_id());
        set_opt_string(&obj, "prefix", self.inner.prefix());
        set_opt_string(&obj, "currency", self.inner.currency());
        set_opt_string(&obj, "base_currency", self.inner.base_currency());
        set_opt_string(&obj, "ticker", self.inner.ticker());
        set_opt_string(&obj, "country", self.inner.country());
        let kind: String = self.inner.kind().into();
        set_opt_string(&obj, "kind", Some(&kind));
        set_opt_string(&obj, "metadata", self.inner.metadata());
        let _ = Reflect::set(
            &obj,
            &"price_scale".into(),
            &JsValue::from_f64(self.inner.price_scale()),
        );
        let _ = Reflect::set(
            &obj,
            &"point_value".into(),
            &JsValue::from_f64(self.inner.point_value()),
        );
        let _ = Reflect::set(
            &obj,
            &"min_tick".into(),
            &JsValue::from_f64(self.inner.min_tick()),
        );
        let _ = Reflect::set(
            &obj,
            &"min_qty".into(),
            &JsValue::from_f64(self.inner.min_qty()),
        );
        obj.into()
    }
}

impl Into<WasmSym> for JsValue {
    #[inline]
    fn into(self) -> WasmSym {
        let obj = self.unchecked_into::<Object>();
        let id = Reflect::get(&obj, &"id".into())
            .unwrap()
            .as_string()
            .map(|s| s.to_string());
        let ticker_id = Reflect::get(&obj, &"ticker_id".into())
            .unwrap()
            .as_string()
            .map(|s| s.to_string());
        let prefix = Reflect::get(&obj, &"prefix".into())
            .unwrap()
            .as_string()
            .map(|s| s.to_string());
        let currency = Reflect::get(&obj, &"currency".into())
            .unwrap()
            .as_string()
            .map(|s| s.to_string());
        let base_currency = Reflect::get(&obj, &"base_currency".into())
            .unwrap()
            .as_string()
            .map(|s| s.to_string());
        let ticker = Reflect::get(&obj, &"ticker".into())
            .unwrap()
            .as_string()
            .map(|s| s.to_string());
        let country = Reflect::get(&obj, &"country".into())
            .unwrap()
            .as_string()
            .map(|s| s.to_string());
        let kind = Reflect::get(&obj, &"kind".into())
            .unwrap()
            .as_string()
            .map(|s| s.to_string());
        let kind = kind.map(|x| SymKind::from(x)).unwrap_or(SymKind::Unknown);
        let metadata = Reflect::get(&obj, &"metadata".into())
            .unwrap()
            .as_string()
            .map(|s| s.to_string());
        let price_scale = Reflect::get(&obj, &"price_scale".into())
            .unwrap()
            .as_f64()
            .unwrap_or(f64::NAN);
        let point_value = Reflect::get(&obj, &"point_value".into())
            .unwrap()
            .as_f64()
            .unwrap_or(f64::NAN);
        let min_tick = Reflect::get(&obj, &"min_tick".into())
            .unwrap()
            .as_f64()
            .unwrap_or(f64::NAN);
        let min_qty = Reflect::get(&obj, &"min_qty".into())
            .unwrap()
            .as_f64()
            .unwrap_or(f64::NAN);
        let mut sym = Sym::default();
        sym.set_id(id);
        sym.set_ticker_id(ticker_id);
        sym.set_prefix(prefix);
        sym.set_currency(currency);
        sym.set_base_currency(base_currency);
        sym.set_ticker(ticker);
        sym.set_country(country);
        sym.set_metadata(metadata);
        sym.set_price_scale(price_scale);
        sym.set_point_value(point_value);
        sym.set_min_tick(min_tick);
        sym.set_min_qty(min_qty);
        sym.set_kind(kind);
        return sym.into();
    }
}

#[wasm_bindgen(js_class = Sym)]
impl WasmSym {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[wasm_bindgen(getter = id)]
    #[inline]
    pub fn wasm_id(&self) -> Option<String> {
        self.inner.id().map(|id| id.to_string())
    }

    #[wasm_bindgen(setter = id)]
    #[inline]
    pub fn wasm_set_id(&mut self, id: Option<String>) {
        self.inner.set_id(id);
    }

    #[wasm_bindgen(getter = tickerId)]
    #[inline]
    pub fn wasm_ticker_id(&self) -> Option<String> {
        self.inner.ticker_id().map(|id| id.to_string())
    }

    #[wasm_bindgen(setter = tickerId)]
    #[inline]
    pub fn wasm_set_ticker_id(&mut self, id: Option<String>) {
        self.inner.set_ticker_id(id);
    }

    #[wasm_bindgen(getter = kind)]
    #[inline]
    pub fn wasm_kind(&self) -> WasmSymKind {
        self.inner.kind().clone().into()
    }

    #[wasm_bindgen(setter = kind)]
    #[inline]
    pub fn wasm_set_kind(&mut self, kind: WasmSymKind) {
        self.inner.set_kind(kind.into());
    }

    #[wasm_bindgen(getter = minTick)]
    #[inline]
    pub fn wasm_min_tick(&self) -> f64 {
        self.inner.min_tick()
    }

    #[wasm_bindgen(setter = minTick)]
    #[inline]
    pub fn wasm_set_min_tick(&mut self, min_tick: f64) {
        self.inner.set_min_tick(min_tick);
    }

    #[wasm_bindgen(getter = minQty)]
    #[inline]
    pub fn wasm_min_qty(&self) -> f64 {
        self.inner.min_qty()
    }

    #[wasm_bindgen(setter = minQty)]
    #[inline]
    pub fn wasm_set_min_qty(&mut self, min_qty: f64) {
        self.inner.set_min_qty(min_qty);
    }

    #[wasm_bindgen(getter = prefix)]
    #[inline]
    pub fn wasm_prefix(&self) -> Option<String> {
        self.inner.prefix().map(|id| id.to_string())
    }

    #[wasm_bindgen(setter = prefix)]
    #[inline]
    pub fn wasm_set_prefix(&mut self, prefix: Option<String>) {
        self.inner.set_prefix(prefix);
    }

    #[wasm_bindgen(getter = currency)]
    #[inline]
    pub fn wasm_currency(&self) -> Option<String> {
        self.inner.currency().map(|id| id.to_string())
    }

    #[wasm_bindgen(setter = currency)]
    #[inline]
    pub fn wasm_set_currency(&mut self, currency: Option<String>) {
        self.inner.set_currency(currency);
    }

    #[wasm_bindgen(getter = baseCurrency)]
    #[inline]
    pub fn wasm_base_currency(&self) -> Option<String> {
        self.inner.base_currency().map(|id| id.to_string())
    }

    #[wasm_bindgen(setter = baseCurrency)]
    #[inline]
    pub fn wasm_set_base_currency(&mut self, base_currency: Option<String>) {
        self.inner.set_base_currency(base_currency);
    }

    #[wasm_bindgen(getter = ticker)]
    #[inline]
    pub fn wasm_ticker(&self) -> Option<String> {
        self.inner.ticker().map(|id| id.to_string())
    }

    #[wasm_bindgen(setter = ticker)]
    #[inline]
    pub fn wasm_set_ticker(&mut self, ticker: Option<String>) {
        self.inner.set_ticker(ticker);
    }

    #[wasm_bindgen(getter = country)]
    #[inline]
    pub fn wasm_country(&self) -> Option<String> {
        self.inner.country().map(|id| id.to_string())
    }

    #[wasm_bindgen(setter = country)]
    #[inline]
    pub fn wasm_set_country(&mut self, country: Option<String>) {
        self.inner.set_country(country);
    }

    #[wasm_bindgen(getter = priceScale)]
    #[inline]
    pub fn wasm_price_scale(&self) -> f64 {
        self.inner.price_scale()
    }

    #[wasm_bindgen(setter = priceScale)]
    #[inline]
    pub fn wasm_set_price_scale(&mut self, price_scale: f64) {
        self.inner.set_price_scale(price_scale);
    }

    #[wasm_bindgen(getter = pointValue)]
    #[inline]
    pub fn wasm_point_value(&self) -> f64 {
        self.inner.point_value()
    }

    #[wasm_bindgen(setter = pointValue)]
    #[inline]
    pub fn wasm_set_point_value(&mut self, point_value: f64) {
        self.inner.set_point_value(point_value);
    }

    #[wasm_bindgen(getter = metadata)]
    #[inline]
    pub fn wasm_metadata(&self) -> Option<String> {
        self.inner.metadata().map(|id| id.to_string())
    }

    #[wasm_bindgen(setter = metadata)]
    #[inline]
    pub fn wasm_set_metadata(&mut self, metadata: Option<String>) {
        self.inner.set_metadata(metadata);
    }

    #[wasm_bindgen(getter = qtyScale)]
    #[inline]
    pub fn wasm_qty_scale(&self) -> f64 {
        self.inner.qty_scale()
    }

    #[wasm_bindgen(js_name = BTC_USD)]
    #[inline]
    pub fn wasm_btc_usd() -> Self {
        Sym::btc_usd().into()
    }

    #[wasm_bindgen(js_name = ETH_USD)]
    #[inline]
    pub fn wasm_eth_usd() -> Self {
        Sym::eth_usd().into()
    }

    #[wasm_bindgen(js_name = SOL_USD)]
    #[inline]
    pub fn wasm_sol_usd() -> Self {
        Sym::sol_usd().into()
    }

    #[wasm_bindgen(js_name = DOGE_USD)]
    #[inline]
    pub fn wasm_doge_usd() -> Self {
        Sym::doge_usd().into()
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn js_to_json(&self) -> JsValue {
        self.into()
    }

    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn js_from_json(json: JsValue) -> WasmSym {
        json.into()
    }

    #[wasm_bindgen(js_name = "eq")]
    pub fn js_eq(&self, other: WasmSym) -> bool {
        self.inner == other.inner
    }
}
