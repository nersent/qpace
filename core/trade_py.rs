use crate::trade::{Trade, TradeDirection, TradeEvent};

cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
  use pyo3::types::PyDict;
}}

#[cfg(feature = "bindings_py")]
#[gen_stub_pymethods]
#[pymethods]
impl TradeEvent {
    #[getter(id)]
    #[inline]
    pub fn py_id(&self) -> Option<String> {
        self.id().cloned()
    }

    #[getter(order_bar_index)]
    #[inline]
    pub fn py_order_bar_index(&self) -> usize {
        self.order_bar_index()
    }

    #[getter(fill_bar_index)]
    #[inline]
    pub fn py_fill_bar_index(&self) -> usize {
        self.fill_bar_index()
    }

    #[getter(price)]
    #[inline]
    pub fn py_price(&self) -> f64 {
        self.price()
    }

    #[getter(comment)]
    #[inline]
    pub fn py_comment(&self) -> Option<String> {
        self.comment().cloned()
    }

    #[pyo3(name = "to_dict")]
    #[inline]
    pub fn py_to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("id", self.id().clone())?;
        dict.set_item("order_bar_index", self.order_bar_index())?;
        dict.set_item("fill_bar_index", self.fill_bar_index())?;
        dict.set_item("price", self.price())?;
        dict.set_item("comment", self.comment().clone())?;
        return Ok(dict.to_object(py));
    }
}

#[cfg(feature = "bindings_py")]
#[gen_stub_pymethods]
#[pymethods]
impl Trade {
    #[getter(size)]
    #[inline]
    #[doc = "the direction and the number of contracts traded in the closed trade. If the value is > 0, the market position was long. If the value is < 0, the market position was short."]
    pub fn py_size(&self) -> f64 {
        self.size()
    }

    #[getter(entry)]
    #[inline]
    pub fn py_entry(&self) -> Option<TradeEvent> {
        self.entry().cloned()
    }

    #[getter(exit)]
    #[inline]
    pub fn py_exit(&self) -> Option<TradeEvent> {
        self.exit().cloned()
    }

    #[getter(pnl)]
    #[inline]
    pub fn py_pnl(&self) -> f64 {
        self.pnl()
    }

    #[getter(direction)]
    #[inline]
    pub fn py_direction(&self) -> TradeDirection {
        self.direction()
    }

    #[getter(is_active)]
    #[inline]
    pub fn py_is_active(&self) -> bool {
        self.is_active()
    }

    #[getter(is_closed)]
    #[inline]
    pub fn py_is_closed(&self) -> bool {
        self.is_closed()
    }

    #[pyo3(name = "to_dict")]
    #[inline]
    pub fn py_to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("size", self.size())?;
        dict.set_item("pnl", self.pnl())?;
        dict.set_item(
            "entry",
            self.entry().as_ref().map(|r| r.py_to_dict(py).unwrap()),
        )?;
        dict.set_item(
            "exit",
            self.exit().as_ref().map(|r| r.py_to_dict(py).unwrap()),
        )?;
        return Ok(dict.to_object(py));
    }
}
