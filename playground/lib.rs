use pyo3::prelude::*;
use pyo3::{
    pyfunction, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction, wrap_pymodule, Bound, PyResult,
};

#[pymodule(name = "qpace_playground_lib")]
pub fn py_mod(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
