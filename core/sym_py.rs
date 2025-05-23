cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
  use pyo3::types::PyDict;
  use pyo3::types::PyList;
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

    #[pyo3(name = "to_dict")]
    pub fn py_to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        dict.set_item("url", self.py_url())?;
        dict.set_item("mime_type", self.py_mime_type())?;
        Ok(dict.into())
    }

    #[staticmethod]
    #[pyo3(name = "from_dict")]
    pub fn py_from_dict(_py: Python<'_>, dict: &Bound<'_, PyAny>) -> PyResult<SymIcon> {
        let url = dict.get_item("url")?.extract::<String>()?;
        let mime_type = dict.get_item("mime_type")?.extract::<String>()?;
        let mut icon = SymIcon::default();
        icon.set_url(url);
        icon.set_mime_type(mime_type);
        Ok(icon)
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
    #[doc = "Unique identifier of the symbol fetched via qPACE API"]
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
    #[doc = "example: `NASDAQ:AAPL`"]
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
    #[doc = "example: `CME_EOD:TICKER -> CME_EOD`"]
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
    #[doc = "example: `NASDAQ:AAPL -> USD`, `EURJPY -> JPY`"]
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
    #[doc = "example: `EURJPY -> EUR`, `BTCUSDT -> BTC`, `CME:6C1! -> CAD`, `NASDAQ:AAPL -> \"\"`"]
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
    #[doc = "symbol name without exchange prefix, \"MSFT\""]
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

    #[pyo3(name = "to_dict")]
    pub fn py_to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        dict.set_item("id", self.py_id())?;
        dict.set_item("ticker_id", self.py_ticker_id())?;
        dict.set_item("min_tick", self.py_min_tick())?;
        dict.set_item("min_qty", self.py_min_qty())?;
        dict.set_item("prefix", self.py_prefix())?;
        dict.set_item("currency", self.py_currency())?;
        dict.set_item("base_currency", self.py_base_currency())?;
        dict.set_item("ticker", self.py_ticker())?;
        dict.set_item("country", self.py_country())?;
        dict.set_item("kind", self.py_kind())?;
        dict.set_item("price_scale", self.py_price_scale())?;
        dict.set_item("point_value", self.py_point_value())?;
        dict.set_item("icons", self.py_icons())?;
        Ok(dict.into())
    }

    #[staticmethod]
    #[pyo3(name = "from_dict")]
    pub fn py_from_dict(_py: Python<'_>, dict: &Bound<'_, PyAny>) -> PyResult<Sym> {
        let id = dict.get_item("id")?.extract::<Option<String>>()?;
        let ticker_id = dict.get_item("ticker_id")?.extract::<Option<String>>()?;
        let min_tick = dict.get_item("min_tick")?.extract::<f64>()?;
        let min_qty = dict.get_item("min_qty")?.extract::<f64>()?;
        let prefix = dict.get_item("prefix")?.extract::<Option<String>>()?;
        let currency = dict.get_item("currency")?.extract::<Option<String>>()?;
        let base_currency = dict
            .get_item("base_currency")?
            .extract::<Option<String>>()?;
        let ticker = dict.get_item("ticker")?.extract::<Option<String>>()?;
        let country = dict.get_item("country")?.extract::<Option<String>>()?;
        let kind = dict.get_item("kind")?.extract::<Option<String>>()?;
        let price_scale = dict.get_item("price_scale")?.extract::<f64>()?;
        let point_value = dict.get_item("point_value")?.extract::<f64>()?;
        let icons = dict.get_item("icons")?.extract::<Vec<SymIcon>>()?;
        let icons = icons.into_iter().map(|icon| icon.clone()).collect();
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
        Ok(sym)
    }
}
