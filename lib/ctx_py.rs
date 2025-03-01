cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
  use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    types::{PySequence, PySlice, PySliceIndices},
  };
  use pyo3::exceptions::PyStopIteration;
  use crate::rs_utils::{pyslice_to_range};
}}
use std::{cell::RefCell, rc::Rc};

use crate::{
    ctx::Ctx,
    ohlcv::{ArcOhlcv, Ohlcv, OhlcvBar, OhlcvLoader, OhlcvReader, OhlcvWriter},
    rs_utils::get_oldest_possible_datetime,
    sym::SymInfo,
};
use chrono::{DateTime, Utc};

#[cfg(feature = "bindings_py")]
#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "Ctx", unsendable))]
#[derive(Clone)]
pub struct PyCtx {
    ctx: Rc<RefCell<Ctx>>,
}

#[cfg(feature = "bindings_py")]
impl PyCtx {
    #[inline]
    pub fn new(ohlcv: Box<dyn OhlcvReader>, sym_info: SymInfo) -> Self {
        Self {
            ctx: Rc::new(RefCell::new(Ctx::new(ohlcv, sym_info))),
        }
    }

    #[inline]
    pub fn fork(&self) -> Self {
        Self {
            ctx: Rc::new(RefCell::new(self.ctx.borrow().fork())),
        }
    }

    #[inline]
    pub fn downcast_py(_: Python<'_>, dp: &Bound<'_, PyAny>) -> PyCtx {
        // @TODO
        unsafe {
            let x = dp.downcast_unchecked::<PyCtx>();
            let x = x.borrow();
            return x.clone();
        }
    }
}

#[cfg(feature = "bindings_py")]
impl Into<Rc<RefCell<Ctx>>> for PyCtx {
    fn into(self) -> Rc<RefCell<Ctx>> {
        self.ctx
    }
}

#[cfg(feature = "bindings_py")]
#[gen_stub_pymethods]
#[pymethods]
impl PyCtx {
    #[staticmethod]
    #[pyo3(
        name = "from_arc_ohlcv",
        signature = (ohlcv, sym_info=None)
    )]
    pub fn py_from_arc_ohlcv(ohlcv: ArcOhlcv, sym_info: Option<SymInfo>) -> Self {
        let sym_info = sym_info.unwrap_or_else(|| SymInfo::default());
        Self::new(ohlcv.clone_box(), sym_info)
    }

    #[staticmethod]
    #[pyo3(
        name = "from_ohlcv",
        signature = (ohlcv, sym_info=None)
    )]
    pub fn py_from_ohlcv(ohlcv: OhlcvLoader, sym_info: Option<SymInfo>) -> Self {
        let sym_info = sym_info.unwrap_or_else(|| SymInfo::default());
        Self::new(ohlcv.clone_box(), sym_info)
    }

    #[getter(bar_index)]
    #[inline]
    pub fn py_bar_index(&self) -> usize {
        self.ctx.borrow().bar_index()
    }

    #[getter(bar)]
    #[inline]
    pub fn py_bar(&self) -> OhlcvBar {
        *self.ctx.borrow().bar()
    }

    #[getter(is_initialized)]
    #[inline]
    pub fn py_is_initialized(&self) -> bool {
        self.ctx.borrow().is_initialized()
    }

    #[getter(sym_info)]
    #[inline]
    pub fn py_sym_info(&self) -> SymInfo {
        *self.ctx.borrow().sym_info()
    }

    #[getter(ohlcv)]
    #[inline]
    pub fn py_ohlcv(&self) -> Option<OhlcvLoader> {
        self.ctx.borrow().ohlcv().into()
    }

    #[getter(arc_ohlcv)]
    #[inline]
    pub fn py_arc_ohlcv(&self) -> Option<ArcOhlcv> {
        self.ctx.borrow().ohlcv().into()
    }

    #[pyo3(name = "fork")]
    #[inline]
    pub fn py_fork(&self) -> Self {
        self.fork()
    }

    #[pyo3(name = "next")]
    #[inline]
    #[doc = "Creates a fresh instance that can be run again. Reuses same OHLCV and symbol."]
    pub fn py_next_(&self) -> Option<usize> {
        self.ctx.borrow_mut().next()
    }

    #[pyo3(name = "__len__")]
    #[inline]
    pub fn py_len(&self) -> usize {
        self.ctx.borrow().len()
    }

    #[pyo3(name = "__next__")]
    #[inline]
    pub fn py_next(&mut self) -> PyResult<usize> {
        let next = self.ctx.borrow_mut().next();
        if next.is_none() {
            return Err(PyStopIteration::new_err("No more items"));
        }
        return Ok(next.unwrap());
    }

    #[pyo3(name = "__iter__")]
    #[inline]
    pub fn py_itr(s: PyRefMut<Self>) -> PyRefMut<Self> {
        s
    }
}
