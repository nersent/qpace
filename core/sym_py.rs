cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
}}
use crate::sym::Sym;

#[cfg(feature = "bindings_py")]
#[gen_stub_pymethods]
#[pymethods]
impl Sym {
    #[pyo3(signature = (min_qty=None, min_tick=None))]
    #[new]
    pub fn py_new(min_qty: Option<f64>, min_tick: Option<f64>) -> Self {
        let min_qty = min_qty.unwrap_or(f64::NAN);
        let min_tick = min_tick.unwrap_or(f64::NAN);
        let mut sym = Sym::default();
        sym.set_min_qty(min_qty);
        sym.set_min_tick(min_tick);
        return sym;
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

    #[getter(min_qty)]
    #[inline]
    #[doc = "https://www.tradingcode.net/tradingview/equity-percent-default-order/#order-size-formula"]
    pub fn py_min_qty(&self) -> f64 {
        self.min_qty()
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
