cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
  use js_sys::{Object, Reflect};
}}
use crate::{
    ohlcv::{OhlcvReader, OhlcvWriter},
    rs_utils::get_oldest_possible_datetime,
    sym::{Sym, SymIcon},
};

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(js_class=SymIcon)]
impl SymIcon {
    #[wasm_bindgen(constructor)]
    pub fn js_new() -> SymIcon {
        SymIcon::default()
    }

    #[wasm_bindgen(getter = url)]
    #[inline]
    pub fn js_url(&self) -> String {
        self.url().to_string()
    }

    #[wasm_bindgen(setter = url)]
    #[inline]
    pub fn js_set_url(&mut self, url: String) {
        self.set_url(url);
    }

    #[wasm_bindgen(getter = mimeType)]
    #[inline]
    pub fn js_mime_type(&self) -> String {
        self.mime_type().to_string()
    }

    #[wasm_bindgen(setter = mimeType)]
    #[inline]
    pub fn js_set_mime_type(&mut self, mime_type: String) {
        self.set_mime_type(mime_type);
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn js_to_json(&self) -> JsValue {
        let obj = Object::new();
        let _ = Reflect::set(&obj, &"url".into(), &JsValue::from_str(&self.js_url()));
        let _ = Reflect::set(
            &obj,
            &"mimeType".into(),
            &JsValue::from_str(&self.js_mime_type()),
        );
        obj.into()
    }

    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn js_from_json(json: JsValue) -> SymIcon {
        let obj = json.unchecked_into::<Object>();
        let url = Reflect::get(&obj, &"url".into())
            .unwrap()
            .as_string()
            .unwrap_or_default();
        let mime_type = Reflect::get(&obj, &"mimeType".into())
            .unwrap()
            .as_string()
            .unwrap_or_default();
        let mut icon = SymIcon::default();
        icon.set_url(url);
        icon.set_mime_type(mime_type);
        icon
    }
}

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(js_class=Sym)]
impl Sym {
    #[wasm_bindgen(constructor)]
    pub fn js_new() -> Sym {
        Sym::default()
    }

    #[wasm_bindgen(getter = id)]
    #[inline]
    #[doc = "Unique identifier of the symbol fetched via qPACE API"]
    pub fn js_id(&self) -> Option<String> {
        self.id().map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = id)]
    #[inline]
    pub fn js_set_id(&mut self, id: Option<String>) {
        self.set_id(id);
    }

    #[wasm_bindgen(getter = tickerId)]
    #[inline]
    #[doc = "example: `NASDAQ:AAPL`"]
    pub fn js_ticker_id(&self) -> Option<String> {
        self.ticker_id().map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = tickerId)]
    #[inline]
    pub fn js_set_ticker_id(&mut self, id: Option<String>) {
        self.set_ticker_id(id);
    }

    #[wasm_bindgen(getter = minTick)]
    #[inline]
    #[doc = "
The tick size is the smallest possible price change an instrument can have [1]. In other words, when the price of an instrument fluctuates, it always changes with the size of at least one tick.
Stocks usually have a tick size of one cent (0.01). Most spot forex symbols trade in 0.00001 increments. The E-mini S&P 500 future uses a tick size of 0.25, while the EuroStoxx 50 future works with a value of 0.5.
    
https://www.tradingcode.net/tradingview/instrument-minimum-tick/
      "]
    pub fn js_min_tick(&self) -> f64 {
        self.min_tick()
    }

    #[wasm_bindgen(setter = minTick)]
    #[inline]
    pub fn js_set_min_tick(&mut self, min_tick: f64) {
        self.set_min_tick(min_tick);
    }

    #[wasm_bindgen(getter = minQty)]
    #[inline]
    #[doc = "https://www.tradingcode.net/tradingview/equity-percent-default-order/#order-size-formula"]
    pub fn js_min_qty(&self) -> f64 {
        self.min_qty()
    }

    #[wasm_bindgen(setter = minQty)]
    #[inline]
    pub fn js_set_min_qty(&mut self, min_qty: f64) {
        self.set_min_qty(min_qty);
    }

    #[wasm_bindgen(getter = prefix)]
    #[inline]
    #[doc = "example: `CME_EOD:TICKER -> CME_EOD`"]
    pub fn js_prefix(&self) -> Option<String> {
        self.prefix().map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = prefix)]
    #[inline]
    pub fn js_set_prefix(&mut self, prefix: Option<String>) {
        self.set_prefix(prefix);
    }

    #[wasm_bindgen(getter = currency)]
    #[inline]
    #[doc = "example: `NASDAQ:AAPL -> USD`, `EURJPY -> JPY`"]
    pub fn js_currency(&self) -> Option<String> {
        self.currency().map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = currency)]
    #[inline]
    pub fn js_set_currency(&mut self, currency: Option<String>) {
        self.set_currency(currency);
    }

    #[wasm_bindgen(getter = baseCurrency)]
    #[inline]
    #[doc = "example: `EURJPY -> EUR`, `BTCUSDT -> BTC`, `CME:6C1! -> CAD`, `NASDAQ:AAPL -> \"\"`"]
    pub fn js_base_currency(&self) -> Option<String> {
        self.base_currency().map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = baseCurrency)]
    #[inline]
    pub fn js_set_base_currency(&mut self, base_currency: Option<String>) {
        self.set_base_currency(base_currency);
    }

    #[wasm_bindgen(getter = ticker)]
    #[inline]
    #[doc = "symbol name without exchange prefix, \"MSFT\""]
    pub fn js_ticker(&self) -> Option<String> {
        self.ticker().map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = ticker)]
    #[inline]
    pub fn js_set_ticker(&mut self, ticker: Option<String>) {
        self.set_ticker(ticker);
    }

    #[wasm_bindgen(getter = country)]
    #[inline]
    pub fn js_country(&self) -> Option<String> {
        self.country().map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = country)]
    #[inline]
    pub fn js_set_country(&mut self, country: Option<String>) {
        self.set_country(country);
    }

    #[wasm_bindgen(getter = kind)]
    #[inline]
    pub fn js_kind(&self) -> Option<String> {
        self.kind().map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = kind)]
    #[inline]
    pub fn js_set_kind(&mut self, kind: Option<String>) {
        self.set_kind(kind);
    }

    #[wasm_bindgen(getter = priceScale)]
    #[inline]
    pub fn js_price_scale(&self) -> f64 {
        self.price_scale()
    }

    #[wasm_bindgen(setter = priceScale)]
    #[inline]
    pub fn js_set_price_scale(&mut self, price_scale: f64) {
        self.set_price_scale(price_scale);
    }

    #[wasm_bindgen(getter = pointValue)]
    #[inline]
    pub fn js_point_value(&self) -> f64 {
        self.point_value()
    }

    #[wasm_bindgen(setter = pointValue)]
    #[inline]
    pub fn js_set_point_value(&mut self, point_value: f64) {
        self.set_point_value(point_value);
    }

    #[wasm_bindgen(getter = icons)]
    #[inline]
    pub fn js_icons(&self) -> Vec<SymIcon> {
        self.icons().to_vec()
    }

    #[wasm_bindgen(setter = icons)]
    #[inline]
    pub fn js_set_icons(&mut self, icons: Vec<SymIcon>) {
        self.set_icons(icons);
    }

    #[wasm_bindgen(js_name = "btc_usd")]
    #[inline]
    pub fn js_btc_usd() -> Self {
        Self::btc_usd()
    }

    #[wasm_bindgen(js_name = "eth_usd")]
    #[inline]
    pub fn js_eth_usd() -> Self {
        Self::eth_usd()
    }

    #[wasm_bindgen(js_name = "sol_usd")]
    #[inline]
    pub fn js_sol_usd() -> Self {
        Self::sol_usd()
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn js_to_json(&self) -> JsValue {
        let obj = Object::new();

        fn set_opt_string(obj: &Object, key: &str, val: &Option<String>) {
            let v = match val {
                Some(s) => JsValue::from_str(s),
                None => JsValue::NULL,
            };
            let _ = Reflect::set(obj, &key.into(), &v);
        }

        set_opt_string(&obj, "id", &self.js_id());
        set_opt_string(&obj, "tickerId", &self.js_ticker_id());

        // Use references for numeric values as well
        let _ = Reflect::set(
            &obj,
            &"minTick".into(),
            &JsValue::from_f64(self.js_min_tick()),
        );
        let _ = Reflect::set(
            &obj,
            &"minQty".into(),
            &JsValue::from_f64(self.js_min_qty()),
        );

        set_opt_string(&obj, "prefix", &self.js_prefix());
        set_opt_string(&obj, "currency", &self.js_currency());
        set_opt_string(&obj, "baseCurrency", &self.js_base_currency());
        set_opt_string(&obj, "ticker", &self.js_ticker());
        set_opt_string(&obj, "country", &self.js_country());
        set_opt_string(&obj, "kind", &self.js_kind());

        let _ = Reflect::set(
            &obj,
            &"priceScale".into(),
            &JsValue::from_f64(self.js_price_scale()),
        );
        let _ = Reflect::set(
            &obj,
            &"pointValue".into(),
            &JsValue::from_f64(self.js_point_value()),
        );

        let icons_js = js_sys::Array::new();
        for icon in self.js_icons() {
            icons_js.push(&icon.js_to_json());
        }
        let _ = Reflect::set(&obj, &"icons".into(), &icons_js);

        obj.into()
    }

    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn js_from_json(json: JsValue) -> Sym {
        let obj = json.unchecked_into::<Object>();
        let id = Reflect::get(&obj, &"id".into())
            .unwrap()
            .as_string()
            .map(|s| s.to_string());
        let ticker_id = Reflect::get(&obj, &"tickerId".into())
            .unwrap()
            .as_string()
            .map(|s| s.to_string());
        let min_tick = Reflect::get(&obj, &"minTick".into())
            .unwrap()
            .as_f64()
            .unwrap_or_default();
        let min_qty = Reflect::get(&obj, &"minQty".into())
            .unwrap()
            .as_f64()
            .unwrap_or_default();
        let prefix = Reflect::get(&obj, &"prefix".into())
            .unwrap()
            .as_string()
            .map(|s| s.to_string());
        let currency = Reflect::get(&obj, &"currency".into())
            .unwrap()
            .as_string()
            .map(|s| s.to_string());
        let base_currency = Reflect::get(&obj, &"baseCurrency".into())
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
        let price_scale = Reflect::get(&obj, &"priceScale".into())
            .unwrap()
            .as_f64()
            .unwrap_or_default();
        let point_value = Reflect::get(&obj, &"pointValue".into())
            .unwrap()
            .as_f64()
            .unwrap_or_default();
        let icons_js_value = Reflect::get(&obj, &"icons".into()).unwrap_or_else(|_| JsValue::NULL);
        let icons_js_array = icons_js_value
            .dyn_into::<js_sys::Array>()
            .unwrap_or_else(|_| js_sys::Array::new());
        let mut icons: Vec<SymIcon> = vec![];
        for icon_js in icons_js_array.iter() {
            let icon = SymIcon::js_from_json(icon_js.clone());
            icons.push(icon);
        }
        let mut sym = Sym::default();
        sym.set_id(id);
        sym.set_ticker_id(ticker_id);
        sym.set_min_tick(min_tick);
        sym.set_min_qty(min_qty);
        sym.set_prefix(prefix);
        sym.set_currency(currency);
        sym.set_base_currency(base_currency);
        sym.set_ticker(ticker);
        sym.set_country(country);
        sym.set_kind(kind);
        sym.set_price_scale(price_scale);
        sym.set_point_value(point_value);
        sym.set_icons(icons);
        sym
    }
}
