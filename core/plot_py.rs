use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use crate::plot::{Label, LineStyle, Position};

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

#[gen_stub_pyclass]
#[pyclass(name = "Label")]
#[derive(Debug, Clone)]
pub struct PyLabel {
    inner: Label,
}

impl From<Label> for PyLabel {
    fn from(value: Label) -> Self {
        Self { inner: value }
    }
}

impl From<PyLabel> for Label {
    fn from(value: PyLabel) -> Self {
        value.inner
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyLabel {
    #[new]
    #[pyo3(signature = (
        bar_index=None,
        text=None,
        color=None,
        position=None
    ))]
    pub fn py_new(
        bar_index: Option<usize>,
        text: Option<String>,
        color: Option<String>,
        position: Option<PyPosition>,
    ) -> Self {
        Label {
            bar_index: bar_index.unwrap_or(0),
            text: text.unwrap_or_default(),
            color,
            position: position.map_or(Position::TopCenter, |p| p.into()),
        }
        .into()
    }

    #[getter(bar_index)]
    #[inline]
    pub fn py_bar_index(&self) -> usize {
        self.inner.bar_index
    }

    #[setter(bar_index)]
    #[inline]
    pub fn py_set_bar_index(&mut self, value: usize) {
        self.inner.bar_index = value;
    }

    #[getter(text)]
    #[inline]
    pub fn py_text(&self) -> String {
        self.inner.text.clone()
    }

    #[setter(text)]
    #[inline]
    pub fn py_set_text(&mut self, value: String) {
        self.inner.text = value;
    }

    #[getter(color)]
    #[inline]
    pub fn py_color(&self) -> Option<String> {
        self.inner.color.clone()
    }

    #[setter(color)]
    #[inline]
    pub fn py_set_color(&mut self, value: Option<String>) {
        self.inner.color = value;
    }

    #[getter(position)]
    #[inline]
    pub fn py_position(&self) -> PyPosition {
        self.inner.position.into()
    }

    #[setter(position)]
    #[inline]
    pub fn py_set_position(&mut self, value: PyPosition) {
        self.inner.position = value.into();
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "Box")]
#[derive(Debug, Clone)]
pub struct PyBox {
    inner: crate::plot::Box,
}

impl From<crate::plot::Box> for PyBox {
    fn from(value: crate::plot::Box) -> Self {
        Self { inner: value }
    }
}

impl From<PyBox> for crate::plot::Box {
    fn from(value: PyBox) -> Self {
        value.inner
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyBox {
    #[new]
    #[pyo3(signature = (
        start_bar_index=None,
        start_value=None,
        end_bar_index=None,
        end_value=None,
        fill_color=None,
        line_color=None,
        line_style=None,
        line_width=None
    ))]
    pub fn py_new(
        start_bar_index: Option<usize>,
        start_value: Option<f64>,
        end_bar_index: Option<usize>,
        end_value: Option<f64>,
        fill_color: Option<String>,
        line_color: Option<String>,
        line_style: Option<PyLineStyle>,
        line_width: Option<usize>,
    ) -> Self {
        crate::plot::Box {
            start_bar_index: start_bar_index.unwrap_or(0),
            start_value: start_value.unwrap_or(0.0),
            end_bar_index: end_bar_index.unwrap_or(0),
            end_value: end_value.unwrap_or(0.0),
            fill_color,
            line_color,
            line_style: line_style.map_or(LineStyle::Solid, |ls| ls.into()),
            line_width: line_width.unwrap_or(1),
        }
        .into()
    }

    #[getter(start_bar_index)]
    #[inline]
    pub fn py_start_bar_index(&self) -> usize {
        self.inner.start_bar_index
    }

    #[setter(start_bar_index)]
    #[inline]
    pub fn py_set_start_bar_index(&mut self, value: usize) {
        self.inner.start_bar_index = value;
    }

    #[getter(start_value)]
    #[inline]
    pub fn py_start_value(&self) -> f64 {
        self.inner.start_value
    }

    #[setter(start_value)]
    #[inline]
    pub fn py_set_start_value(&mut self, value: f64) {
        self.inner.start_value = value;
    }

    #[getter(end_bar_index)]
    #[inline]
    pub fn py_end_bar_index(&self) -> usize {
        self.inner.end_bar_index
    }

    #[setter(end_bar_index)]
    #[inline]
    pub fn py_set_end_bar_index(&mut self, value: usize) {
        self.inner.end_bar_index = value;
    }

    #[getter(end_value)]
    #[inline]
    pub fn py_end_value(&self) -> f64 {
        self.inner.end_value
    }

    #[setter(end_value)]
    #[inline]
    pub fn py_set_end_value(&mut self, value: f64) {
        self.inner.end_value = value;
    }

    #[getter(fill_color)]
    #[inline]
    pub fn py_fill_color(&self) -> Option<String> {
        self.inner.fill_color.clone()
    }

    #[setter(fill_color)]
    #[inline]
    pub fn py_set_fill_color(&mut self, value: Option<String>) {
        self.inner.fill_color = value;
    }

    #[getter(line_color)]
    #[inline]
    pub fn py_line_color(&self) -> Option<String> {
        self.inner.line_color.clone()
    }

    #[setter(line_color)]
    #[inline]
    pub fn py_set_line_color(&mut self, value: Option<String>) {
        self.inner.line_color = value;
    }

    #[getter(line_style)]
    #[inline]
    pub fn py_line_style(&self) -> PyLineStyle {
        self.inner.line_style.into()
    }

    #[setter(line_style)]
    #[inline]
    pub fn py_set_line_style(&mut self, value: PyLineStyle) {
        self.inner.line_style = value.into();
    }

    #[getter(line_width)]
    #[inline]
    pub fn py_line_width(&self) -> usize {
        self.inner.line_width
    }

    #[setter(line_width)]
    #[inline]
    pub fn py_set_line_width(&mut self, value: usize) {
        self.inner.line_width = value;
    }
}
