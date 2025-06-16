use crate::signal::Signal;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[gen_stub_pyclass]
#[pyclass(name = "Signal")]
#[derive(Debug, Clone, PartialEq)]
pub struct PySignal {
    inner: Signal,
}

impl Into<Signal> for PySignal {
    #[inline]
    fn into(self) -> Signal {
        self.inner
    }
}

impl From<Signal> for PySignal {
    #[inline]
    fn from(inner: Signal) -> Self {
        PySignal { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PySignal {
    #[staticmethod]
    #[pyo3(name = "Hold")]
    #[inline]
    pub fn py_hold() -> Self {
        Signal::hold().into()
    }

    #[staticmethod]
    #[pyo3(name = "Size")]
    #[inline]
    pub fn py_size(size: f64) -> Self {
        Signal::size(size).into()
    }

    #[staticmethod]
    #[pyo3(name = "Equity_pct")]
    #[inline]
    pub fn py_equity_pct(equity_pct: f64) -> Self {
        Signal::equity_pct(equity_pct).into()
    }

    #[staticmethod]
    #[pyo3(name = "Close_all")]
    #[inline]
    pub fn py_close_all() -> Self {
        Signal::close_all().into()
    }

    #[staticmethod]
    #[pyo3(name = "Long")]
    #[inline]
    pub fn py_long() -> Self {
        Signal::long().into()
    }

    #[staticmethod]
    #[pyo3(name = "Short")]
    #[inline]
    pub fn py_short() -> Self {
        Signal::short().into()
    }

    #[getter(id)]
    #[inline]
    pub fn py_id(&self) -> Option<String> {
        self.inner.id().cloned()
    }

    #[setter(id)]
    #[inline]
    pub fn py_set_id(&mut self, id: Option<String>) {
        self.inner.set_id(id);
    }

    #[getter(comment)]
    #[inline]
    pub fn py_comment(&self) -> Option<String> {
        self.inner.comment().cloned()
    }

    #[setter(comment)]
    #[inline]
    pub fn py_set_comment(&mut self, comment: Option<String>) {
        self.inner.set_comment(comment);
    }
}
