cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
}}
use crate::signal::{Signal, SignalKind};

#[cfg(feature = "bindings_py")]
#[gen_stub_pymethods]
#[pymethods]
impl Signal {
    #[getter(kind)]
    #[inline]
    pub fn py_kind(&self) -> SignalKind {
        *self.kind()
    }

    #[getter(id)]
    #[inline]
    pub fn py_id(&self) -> Option<String> {
        self.id().clone()
    }

    #[setter(id)]
    #[inline]
    pub fn py_set_id(&mut self, id: Option<String>) {
        self.set_id(id);
    }

    #[staticmethod]
    #[pyo3(name = "hold")]
    #[inline]
    pub fn py_hold() -> Self {
        Self::hold()
    }

    #[staticmethod]
    #[pyo3(name = "size")]
    #[inline]
    pub fn py_size(size: f64) -> Self {
        Self::size(size)
    }

    #[staticmethod]
    #[pyo3(name = "equity_pct")]
    #[inline]
    pub fn py_equity_pct(equity_pct: f64) -> Self {
        Self::equity_pct(equity_pct)
    }

    #[staticmethod]
    #[pyo3(name = "close_all")]
    #[inline]
    pub fn py_close_all() -> Self {
        Self::close_all()
    }

    #[staticmethod]
    #[pyo3(name = "long")]
    #[inline]
    pub fn py_long() -> Self {
        Self::long()
    }

    #[staticmethod]
    #[pyo3(name = "short")]
    #[inline]
    pub fn py_short() -> Self {
        Self::short()
    }
}
