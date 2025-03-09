cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
}}
use crate::timeframe::Timeframe;

#[cfg(feature = "bindings_py")]
#[gen_stub_pymethods]
#[pymethods]
impl Timeframe {
    #[pyo3(name = "__format__", signature = (format_spec=None))]
    #[inline]
    pub fn py_format(&self, format_spec: Option<String>) -> String {
        format!("{:?}", self)
    }

    #[staticmethod]
    #[pyo3(name = "from_str")]
    #[inline]
    pub fn py_from_str(timeframe: String) -> Self {
        Timeframe::from(timeframe)
    }
}
