use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use crate::plot::{LineStyle, Position};

#[gen_stub_pyclass]
#[pyclass(name = "LineStyle")]
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct PyLineStyle {
    inner: LineStyle,
}

impl From<LineStyle> for PyLineStyle {
    fn from(value: LineStyle) -> Self {
        Self { inner: value }
    }
}

impl From<PyLineStyle> for LineStyle {
    fn from(value: PyLineStyle) -> Self {
        value.inner
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyLineStyle {
    #[pyo3(name = "__str__")]
    #[inline]
    pub fn py_str(&self) -> String {
        String::from(self.inner)
    }

    #[staticmethod]
    #[pyo3(name = "Solid")]
    #[inline]
    pub fn py_solid() -> Self {
        LineStyle::Solid.into()
    }

    #[staticmethod]
    #[pyo3(name = "Dashed")]
    #[inline]
    pub fn py_dashed() -> Self {
        LineStyle::Dashed.into()
    }

    #[staticmethod]
    #[pyo3(name = "Dotted")]
    #[inline]
    pub fn py_dotted() -> Self {
        LineStyle::Dotted.into()
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "Position")]
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct PyPosition {
    inner: Position,
}

impl From<Position> for PyPosition {
    fn from(value: Position) -> Self {
        Self { inner: value }
    }
}

impl From<PyPosition> for Position {
    fn from(value: PyPosition) -> Self {
        value.inner
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyPosition {
    #[pyo3(name = "__str__")]
    #[inline]
    pub fn py_str(&self) -> String {
        String::from(self.inner)
    }

    #[staticmethod]
    #[pyo3(name = "TopCenter")]
    #[inline]
    pub fn py_top_center() -> Self {
        Position::TopCenter.into()
    }

    #[staticmethod]
    #[pyo3(name = "BottomCenter")]
    #[inline]
    pub fn py_bottom_center() -> Self {
        Position::BottomCenter.into()
    }
}
