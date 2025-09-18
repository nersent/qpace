use crate::signal::Signal;
use pyo3::prelude::*;
use pyo3::types::PyDict;
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
    #[pyo3(name = "EquityPct")]
    #[inline]
    pub fn py_equity_pct(equity_pct: f64) -> Self {
        Signal::equity_pct(equity_pct).into()
    }

    #[staticmethod]
    #[pyo3(name = "CloseAll")]
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

    #[getter(equity_pct)]
    #[inline]
    pub fn py_get_equity_pct(&self) -> Option<f64> {
        self.inner.get_equity_pct()
    }

    #[getter(size)]
    #[inline]
    pub fn py_get_size(&self) -> Option<f64> {
        self.inner.get_size()
    }

    #[getter(is_hold)]
    #[inline]
    pub fn py_is_hold(&self) -> bool {
        self.inner.is_hold()
    }

    #[getter(is_close_all)]
    #[inline]
    pub fn py_is_close_all(&self) -> bool {
        self.inner.is_close_all()
    }

    #[pyo3(name = "__str__")]
    #[inline]
    pub fn py_str(&self) -> String {
        format!("{:?}", self.inner)
    }

    #[pyo3(name = "__repr__")]
    #[inline]
    pub fn py_repr(&self) -> String {
        format!("{:?}", self.inner)
    }

    #[cfg(feature = "json")]
    #[pyo3(name = "to_dict")]
    pub fn py_to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let any = pythonize::pythonize(py, &self.inner)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(any.into_py(py))
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    #[pyo3(name = "from_dict")]
    pub fn py_from_dict(_py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<PySignal> {
        let sig: Signal = pythonize::depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(sig.into())
    }
}
