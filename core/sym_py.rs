use crate::sym::{Sym, SymKind};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[gen_stub_pyclass]
#[pyclass(name = "SymKind")]
#[derive(Debug, Clone, PartialEq)]
pub struct PySymKind {
    inner: SymKind,
}

impl Into<SymKind> for PySymKind {
    #[inline]
    fn into(self) -> SymKind {
        self.inner
    }
}

impl From<SymKind> for PySymKind {
    #[inline]
    fn from(inner: SymKind) -> Self {
        PySymKind { inner }
    }
}

impl Default for PySymKind {
    #[inline]
    fn default() -> Self {
        SymKind::default().into()
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PySymKind {
    #[pyo3(name = "__str__")]
    #[inline]
    pub fn py_str(&self) -> String {
        (&self.inner).into()
    }

    #[pyo3(name = "__repr__")]
    #[inline]
    pub fn py_repr(&self) -> String {
        (&self.inner).into()
    }

    #[staticmethod]
    #[pyo3(name = "from_str")]
    #[inline]
    pub fn py_from_str(kind: String) -> Self {
        SymKind::from(kind).into()
    }

    #[staticmethod]
    #[pyo3(name = "Stock")]
    #[inline]
    pub fn py_stock() -> Self {
        SymKind::Stock.into()
    }

    #[staticmethod]
    #[pyo3(name = "Future")]
    #[inline]
    pub fn py_future() -> Self {
        SymKind::Future.into()
    }

    #[staticmethod]
    #[pyo3(name = "Option")]
    #[inline]
    pub fn py_option() -> Self {
        SymKind::Option.into()
    }

    #[staticmethod]
    #[pyo3(name = "Forex")]
    #[inline]
    pub fn py_forex() -> Self {
        SymKind::Forex.into()
    }

    #[staticmethod]
    #[pyo3(name = "Crypto")]
    #[inline]
    pub fn py_crypto() -> Self {
        SymKind::Crypto.into()
    }

    #[staticmethod]
    #[pyo3(name = "Unknown")]
    #[inline]
    pub fn py_unknown() -> Self {
        SymKind::Unknown.into()
    }

    #[staticmethod]
    #[pyo3(name = "Other")]
    #[inline]
    pub fn py_other(kind: String) -> Self {
        SymKind::Other(kind).into()
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "Sym")]
#[derive(Debug, Clone)]
pub struct PySym {
    inner: Sym,
}

impl Default for PySym {
    #[inline]
    fn default() -> Self {
        Sym::default().into()
    }
}

impl Into<Sym> for PySym {
    #[inline]
    fn into(self) -> Sym {
        self.inner
    }
}

impl From<Sym> for PySym {
    #[inline]
    fn from(inner: Sym) -> Self {
        PySym { inner }
    }
}

impl IntoPy<PyResult<PyObject>> for PySym {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        dict.set_item("id", self.inner.id())?;
        dict.set_item("ticker_id", self.inner.ticker_id())?;
        let kind: String = self.inner.kind().into();
        dict.set_item("kind", kind)?;
        dict.set_item("min_tick", self.inner.min_tick())?;
        dict.set_item("min_qty", self.inner.min_qty())?;
        dict.set_item("prefix", self.inner.prefix())?;
        dict.set_item("currency", self.inner.currency())?;
        dict.set_item("base_currency", self.inner.base_currency())?;
        dict.set_item("ticker", self.inner.ticker())?;
        dict.set_item("country", self.inner.country())?;
        dict.set_item("price_scale", self.inner.price_scale())?;
        dict.set_item("point_value", self.inner.point_value())?;
        dict.set_item("metadata", self.inner.metadata())?;
        Ok(dict.into())
    }
}

impl PySym {
    #[inline]
    pub fn from_py(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        let id = obj.get_item("id")?.extract::<Option<String>>()?;
        let ticker_id = obj.get_item("ticker_id")?.extract::<Option<String>>()?;
        let kind = obj.get_item("kind")?.extract::<String>()?;
        let kind: SymKind = kind.into();
        let min_tick = obj.get_item("min_tick")?.extract::<f64>()?;
        let min_qty = obj.get_item("min_qty")?.extract::<f64>()?;
        let prefix = obj.get_item("prefix")?.extract::<Option<String>>()?;
        let currency = obj.get_item("currency")?.extract::<Option<String>>()?;
        let base_currency = obj.get_item("base_currency")?.extract::<Option<String>>()?;
        let ticker = obj.get_item("ticker")?.extract::<Option<String>>()?;
        let country = obj.get_item("country")?.extract::<Option<String>>()?;
        let price_scale = obj.get_item("price_scale")?.extract::<f64>()?;
        let point_value = obj.get_item("point_value")?.extract::<f64>()?;
        let metadata = obj.get_item("metadata")?.extract::<Option<String>>()?;
        let mut sym = Sym::default();
        sym.set_id(id);
        sym.set_ticker_id(ticker_id);
        sym.set_kind(kind);
        sym.set_min_tick(min_tick);
        sym.set_min_qty(min_qty);
        sym.set_prefix(prefix);
        sym.set_currency(currency);
        sym.set_base_currency(base_currency);
        sym.set_ticker(ticker);
        sym.set_country(country);
        sym.set_price_scale(price_scale);
        sym.set_point_value(point_value);
        sym.set_metadata(metadata);
        Ok(sym.into())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PySym {
    #[new]
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[pyo3(name = "__str__")]
    #[inline]
    pub fn py_str(&self) -> String {
        (&self.inner).into()
    }

    #[pyo3(name = "__repr__")]
    #[inline]
    pub fn py_repr(&self) -> String {
        (&self.inner).into()
    }

    #[getter(id)]
    #[inline]
    pub fn py_id(&self) -> Option<String> {
        self.inner.id().map(|s| s.to_string())
    }

    #[setter(id)]
    #[inline]
    pub fn py_set_id(&mut self, id: Option<String>) {
        self.inner.set_id(id);
    }

    #[getter(ticker_id)]
    #[inline]
    pub fn py_ticker_id(&self) -> Option<String> {
        self.inner.ticker_id().map(|s| s.to_string())
    }

    #[setter(ticker_id)]
    #[inline]
    pub fn py_set_ticker_id(&mut self, ticker_id: Option<String>) {
        self.inner.set_ticker_id(ticker_id);
    }

    #[getter(kind)]
    #[inline]
    pub fn py_kind(&self) -> PySymKind {
        self.inner.kind().clone().into()
    }

    #[setter(kind)]
    #[inline]
    pub fn py_set_kind(&mut self, kind: PySymKind) {
        self.inner.set_kind(kind.inner);
    }

    #[getter(min_tick)]
    #[inline]
    pub fn py_min_tick(&self) -> f64 {
        self.inner.min_tick()
    }

    #[setter(min_tick)]
    #[inline]
    pub fn py_set_min_tick(&mut self, min_tick: f64) {
        self.inner.set_min_tick(min_tick);
    }

    #[getter(min_qty)]
    #[inline]
    pub fn py_min_qty(&self) -> f64 {
        self.inner.min_qty()
    }

    #[setter(min_qty)]
    #[inline]
    pub fn py_set_min_qty(&mut self, min_qty: f64) {
        self.inner.set_min_qty(min_qty);
    }

    #[getter(prefix)]
    #[inline]
    pub fn py_prefix(&self) -> Option<String> {
        self.inner.prefix().map(|s| s.to_string())
    }

    #[setter(prefix)]
    #[inline]
    pub fn py_set_prefix(&mut self, prefix: Option<String>) {
        self.inner.set_prefix(prefix);
    }

    #[getter(currency)]
    #[inline]
    pub fn py_currency(&self) -> Option<String> {
        self.inner.currency().map(|s| s.to_string())
    }

    #[setter(currency)]
    #[inline]
    pub fn py_set_currency(&mut self, currency: Option<String>) {
        self.inner.set_currency(currency);
    }

    #[getter(base_currency)]
    #[inline]
    pub fn py_base_currency(&self) -> Option<String> {
        self.inner.base_currency().map(|s| s.to_string())
    }

    #[setter(base_currency)]
    #[inline]
    pub fn py_set_base_currency(&mut self, base_currency: Option<String>) {
        self.inner.set_base_currency(base_currency);
    }

    #[getter(ticker)]
    #[inline]
    pub fn py_ticker(&self) -> Option<String> {
        self.inner.ticker().map(|s| s.to_string())
    }

    #[setter(ticker)]
    #[inline]
    pub fn py_set_ticker(&mut self, ticker: Option<String>) {
        self.inner.set_ticker(ticker);
    }

    #[getter(country)]
    #[inline]
    pub fn py_country(&self) -> Option<String> {
        self.inner.country().map(|s| s.to_string())
    }

    #[setter(country)]
    #[inline]
    pub fn py_set_country(&mut self, country: Option<String>) {
        self.inner.set_country(country);
    }

    #[getter(price_scale)]
    #[inline]
    pub fn py_price_scale(&self) -> f64 {
        self.inner.price_scale()
    }

    #[setter(price_scale)]
    #[inline]
    pub fn py_set_price_scale(&mut self, price_scale: f64) {
        self.inner.set_price_scale(price_scale);
    }

    #[getter(point_value)]
    #[inline]
    pub fn py_point_value(&self) -> f64 {
        self.inner.point_value()
    }

    #[setter(point_value)]
    #[inline]
    pub fn py_set_point_value(&mut self, point_value: f64) {
        self.inner.set_point_value(point_value);
    }

    #[getter(metadata)]
    #[inline]
    pub fn py_metadata(&self) -> Option<String> {
        self.inner.metadata().map(|s| s.to_string())
    }

    #[setter(metadata)]
    #[inline]
    pub fn py_set_metadata(&mut self, metadata: Option<String>) {
        self.inner.set_metadata(metadata);
    }

    #[getter(qty_scale)]
    #[inline]
    pub fn py_qty_scale(&self) -> f64 {
        self.inner.qty_scale()
    }

    #[staticmethod]
    #[pyo3(name = "from_dict")]
    pub fn py_from_dict(_py: Python<'_>, dict: &Bound<'_, PyAny>) -> PyResult<PySym> {
        PySym::from_py(dict)
    }

    #[pyo3(name = "to_dict")]
    pub fn py_to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.clone().into_py(py)
    }

    #[staticmethod]
    #[pyo3(name = "BTC_USD")]
    #[inline]
    pub fn py_btc_usd() -> Self {
        Sym::btc_usd().into()
    }

    #[staticmethod]
    #[pyo3(name = "ETH_USD")]
    #[inline]
    pub fn py_eth_usd() -> Self {
        Sym::eth_usd().into()
    }

    #[staticmethod]
    #[pyo3(name = "SOL_USD")]
    #[inline]
    pub fn py_sol_usd() -> Self {
        Sym::sol_usd().into()
    }

    #[staticmethod]
    #[pyo3(name = "DOGE_USD")]
    #[inline]
    pub fn py_doge_usd() -> Self {
        Sym::doge_usd().into()
    }
}
