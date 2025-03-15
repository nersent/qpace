use crate::utils::{round_contracts, round_to_min_tick, validate_contracts};

cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
}}
cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "SymIcon"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "SymIcon"))]
#[derive(Debug, Clone)]
pub struct SymIcon {
    url: String,
    mime_type: String,
}

impl Default for SymIcon {
    #[inline]
    fn default() -> Self {
        Self {
            url: String::new(),
            mime_type: String::new(),
        }
    }
}

impl SymIcon {
    #[inline]
    pub fn url(&self) -> &str {
        &self.url
    }

    #[inline]
    pub fn set_url(&mut self, url: String) -> &mut Self {
        self.url = url;
        self
    }

    #[inline]
    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }

    #[inline]
    pub fn set_mime_type(&mut self, mime_type: String) -> &mut Self {
        self.mime_type = mime_type;
        self
    }
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "Sym"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "Sym"))]
#[derive(Debug, Clone)]
pub struct Sym {
    min_tick: f64,
    min_qty: f64,
    id: Option<String>,
    ticker_id: Option<String>,
    prefix: Option<String>,
    currency: Option<String>,
    base_currency: Option<String>,
    ticker: Option<String>,
    country: Option<String>,
    icons: Vec<SymIcon>,
}

impl Default for Sym {
    #[inline]
    fn default() -> Self {
        Self {
            min_tick: f64::NAN,
            min_qty: f64::NAN,
            id: None,
            ticker_id: None,
            prefix: None,
            currency: None,
            base_currency: None,
            ticker: None,
            country: None,
            icons: vec![],
        }
    }
}

impl Sym {
    #[inline]
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    #[inline]
    pub fn set_id(&mut self, id: Option<String>) -> &mut Self {
        self.id = id;
        self
    }

    #[inline]
    pub fn ticker_id(&self) -> Option<&str> {
        self.ticker_id.as_deref()
    }

    #[inline]
    pub fn set_ticker_id(&mut self, ticker_id: Option<String>) -> &mut Self {
        self.ticker_id = ticker_id;
        self
    }

    #[inline]
    pub fn min_tick(&self) -> f64 {
        self.min_tick
    }

    #[inline]
    pub fn set_min_tick(&mut self, min_tick: f64) -> &mut Self {
        self.min_tick = min_tick;
        self
    }

    #[inline]
    pub fn min_qty(&self) -> f64 {
        self.min_qty
    }

    #[inline]
    pub fn set_min_qty(&mut self, min_qty: f64) -> &mut Self {
        self.min_qty = min_qty;
        self
    }

    #[inline]
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    #[inline]
    pub fn set_prefix(&mut self, prefix: Option<String>) -> &mut Self {
        self.prefix = prefix;
        self
    }

    #[inline]
    pub fn currency(&self) -> Option<&str> {
        self.currency.as_deref()
    }

    #[inline]
    pub fn set_currency(&mut self, currency: Option<String>) -> &mut Self {
        self.currency = currency;
        self
    }

    #[inline]
    pub fn base_currency(&self) -> Option<&str> {
        self.base_currency.as_deref()
    }

    #[inline]
    pub fn set_base_currency(&mut self, base_currency: Option<String>) -> &mut Self {
        self.base_currency = base_currency;
        self
    }

    #[inline]
    pub fn ticker(&self) -> Option<&str> {
        self.ticker.as_deref()
    }

    #[inline]
    pub fn set_ticker(&mut self, ticker: Option<String>) -> &mut Self {
        self.ticker = ticker;
        self
    }

    #[inline]
    pub fn country(&self) -> Option<&str> {
        self.country.as_deref()
    }

    #[inline]
    pub fn set_country(&mut self, country: Option<String>) -> &mut Self {
        self.country = country;
        self
    }

    #[inline]
    pub fn icons(&self) -> &Vec<SymIcon> {
        &self.icons
    }

    #[inline]
    pub fn set_icons(&mut self, icons: Vec<SymIcon>) -> &mut Self {
        self.icons = icons;
        self
    }

    #[inline]
    pub fn round_to_min_tick(&self, value: f64) -> f64 {
        round_to_min_tick(value, self.min_tick)
    }

    #[inline]
    pub fn validate_contracts(&self, size: f64) -> bool {
        return validate_contracts(size, self.min_qty);
    }

    #[inline]
    pub fn round_contracts(&self, size: f64) -> f64 {
        return round_contracts(size, self.min_qty);
    }

    #[inline]
    pub fn btc_usd() -> Self {
        Self {
            min_tick: 1.0,
            min_qty: 0.000001,
            ..Default::default()
        }
    }

    #[inline]
    pub fn eth_usd() -> Self {
        Self {
            min_tick: 0.1,
            min_qty: 0.0001,
            ..Default::default()
        }
    }

    #[inline]
    pub fn sol_usd() -> Self {
        Self {
            min_tick: 0.01,
            min_qty: 0.0001,
            ..Default::default()
        }
    }
}
