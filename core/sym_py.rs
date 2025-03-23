cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
}}
use crate::sym::{Sym, SymIcon};

#[cfg(feature = "bindings_py")]
#[gen_stub_pymethods]
#[pymethods]
impl SymIcon {
    #[new]
    pub fn py_new() -> Self {
        SymIcon::default()
    }

    #[getter(url)]
    #[inline]
    pub fn py_url(&self) -> String {
        self.url().to_string()
    }

    #[setter(url)]
    #[inline]
    pub fn py_set_url(&mut self, url: String) {
        self.set_url(url);
    }

    #[getter(mime_type)]
    #[inline]
    pub fn py_mime_type(&self) -> String {
        self.mime_type().to_string()
    }

    #[setter(mime_type)]
    #[inline]
    pub fn py_set_mime_type(&mut self, mime_type: String) {
        self.set_mime_type(mime_type);
    }
}

#[cfg(feature = "bindings_py")]
#[gen_stub_pymethods]
#[pymethods]
impl Sym {
    #[new]
    pub fn py_new() -> Self {
        Sym::default()
    }

    #[getter(id)]
    #[inline]
    pub fn py_id(&self) -> Option<String> {
        self.id().map(|s| s.to_string())
    }

    #[setter(id)]
    #[inline]
    pub fn py_set_id(&mut self, id: Option<String>) {
        self.set_id(id);
    }

    #[getter(ticker_id)]
    #[inline]
    pub fn py_ticker_id(&self) -> Option<String> {
        self.ticker_id().map(|s| s.to_string())
    }

    #[setter(ticker_id)]
    #[inline]
    pub fn py_set_ticker_id(&mut self, ticker_id: Option<String>) {
        self.set_ticker_id(ticker_id);
    }

    #[getter(min_tick)]
    #[inline]
    #[doc = "
The tick size is the smallest possible price change an instrument can have [1]. In other words, when the price of an instrument fluctuates, it always changes with the size of at least one tick.
Stocks usually have a tick size of one cent (0.01). Most spot forex symbols trade in 0.00001 increments. The E-mini S&P 500 future uses a tick size of 0.25, while the EuroStoxx 50 future works with a value of 0.5.
    
https://www.tradingcode.net/tradingview/instrument-minimum-tick/
      "]
    pub fn py_min_tick(&self) -> f64 {
        self.min_tick()
    }

    #[setter(min_tick)]
    #[inline]
    pub fn py_set_min_tick(&mut self, min_tick: f64) {
        self.set_min_tick(min_tick);
    }

    #[getter(min_qty)]
    #[inline]
    #[doc = "https://www.tradingcode.net/tradingview/equity-percent-default-order/#order-size-formula"]
    pub fn py_min_qty(&self) -> f64 {
        self.min_qty()
    }

    #[setter(min_qty)]
    #[inline]
    pub fn py_set_min_qty(&mut self, min_qty: f64) {
        self.set_min_qty(min_qty);
    }

    #[getter(prefix)]
    #[inline]
    pub fn py_prefix(&self) -> Option<String> {
        self.prefix().map(|s| s.to_string())
    }

    #[setter(prefix)]
    #[inline]
    pub fn py_set_prefix(&mut self, prefix: Option<String>) {
        self.set_prefix(prefix);
    }

    #[getter(currency)]
    #[inline]
    pub fn py_currency(&self) -> Option<String> {
        self.currency().map(|s| s.to_string())
    }

    #[setter(currency)]
    #[inline]
    pub fn py_set_currency(&mut self, currency: Option<String>) {
        self.set_currency(currency);
    }

    #[getter(base_currency)]
    #[inline]
    pub fn py_base_currency(&self) -> Option<String> {
        self.base_currency().map(|s| s.to_string())
    }

    #[setter(base_currency)]
    #[inline]
    pub fn py_set_base_currency(&mut self, base_currency: Option<String>) {
        self.set_base_currency(base_currency);
    }

    #[getter(ticker)]
    #[inline]
    pub fn py_ticker(&self) -> Option<String> {
        self.ticker().map(|s| s.to_string())
    }

    #[setter(ticker)]
    #[inline]
    pub fn py_set_ticker(&mut self, ticker: Option<String>) {
        self.set_ticker(ticker);
    }

    #[getter(country)]
    #[inline]
    pub fn py_country(&self) -> Option<String> {
        self.country().map(|s| s.to_string())
    }

    #[setter(country)]
    #[inline]
    pub fn py_set_country(&mut self, country: Option<String>) {
        self.set_country(country);
    }

    #[getter(kind)]
    #[inline]
    pub fn py_kind(&self) -> Option<String> {
        self.kind().map(|s| s.to_string())
    }

    #[setter(kind)]
    #[inline]
    pub fn py_set_kind(&mut self, kind: Option<String>) {
        self.set_kind(kind);
    }

    #[getter(price_scale)]
    #[inline]
    pub fn py_price_scale(&self) -> f64 {
        self.price_scale()
    }

    #[setter(price_scale)]
    #[inline]
    pub fn py_set_price_scale(&mut self, price_scale: f64) {
        self.set_price_scale(price_scale);
    }

    #[getter(point_value)]
    #[inline]
    pub fn py_point_value(&self) -> f64 {
        self.point_value()
    }

    #[setter(point_value)]
    #[inline]
    pub fn py_set_point_value(&mut self, point_value: f64) {
        self.set_point_value(point_value);
    }

    #[getter(icons)]
    #[inline]
    pub fn py_icons(&self) -> Vec<SymIcon> {
        self.icons().to_vec()
    }

    #[setter(icons)]
    #[inline]
    pub fn py_set_icons(&mut self, icons: Vec<SymIcon>) {
        self.set_icons(icons);
    }

    #[staticmethod]
    #[pyo3(name = "btc_usd")]
    #[inline]
    pub fn py_btc_usd() -> Self {
        Self::btc_usd()
    }

    #[staticmethod]
    #[pyo3(name = "eth_usd")]
    #[inline]
    pub fn py_eth_usd() -> Self {
        Self::eth_usd()
    }

    #[staticmethod]
    #[pyo3(name = "sol_usd")]
    #[inline]
    pub fn py_sol_usd() -> Self {
        Self::sol_usd()
    }
}
