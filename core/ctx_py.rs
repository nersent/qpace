cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
  use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    types::{PySequence, PySlice, PySliceIndices},
  };
  use pyo3::exceptions::PyStopIteration;
  use crate::rs_utils::{pyslice_to_range};
  use crate::ohlcv_py::PyOhlcv;
  use crate::timeframe_py::PyTimeframe;
}}
use std::{cell::RefCell, rc::Rc};

use crate::{
    ctx::Ctx,
    ohlcv::{Ohlcv, OhlcvBar, OhlcvReader, OhlcvWriter},
    rs_utils::get_oldest_possible_datetime,
    sym::Sym,
    timeframe::Timeframe,
};
use chrono::{DateTime, Utc};

#[cfg(feature = "bindings_py")]
#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "Ctx", unsendable))]
#[derive(Clone)]
pub struct PyCtx {
    ctx: Rc<RefCell<Ctx>>,
    ohlcv: PyOhlcv,
}

#[cfg(feature = "bindings_py")]
impl PyCtx {
    #[inline]
    pub fn fork(&self) -> Self {
        Self {
            ctx: Rc::new(RefCell::new(self.ctx.borrow().fork())),
            ohlcv: self.ohlcv.clone(),
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
    #[new]
    #[pyo3(signature = (ohlcv, sym=None))]
    pub fn py_new(ohlcv: PyOhlcv, sym: Option<Sym>) -> Self {
        let sym = sym.unwrap_or_else(|| Sym::default());
        let timeframe = ohlcv.py_timeframe();
        let mut ctx = Ctx::new();
        ctx.set_ohlcv(ohlcv.clone_box());
        ctx.set_sym(sym);
        ctx.set_timeframe(timeframe.into());
        Self {
            ctx: Rc::new(RefCell::new(ctx)),
            ohlcv,
        }
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

    #[getter(sym)]
    #[inline]
    pub fn py_sym(&self) -> Sym {
        self.ctx.borrow().sym().clone()
    }

    #[getter(timeframe)]
    #[inline]
    pub fn py_timeframe(&self) -> PyTimeframe {
        (*self.ctx.borrow().timeframe()).into()
    }

    #[getter(ohlcv)]
    #[inline]
    pub fn py_ohlcv(&self) -> PyOhlcv {
        self.ohlcv.clone()
    }

    #[pyo3(name = "fork")]
    #[inline]
    #[doc = "Creates a new instance starting from first bar. Reuses same OHLCV and symbol."]
    pub fn py_fork(&self) -> Self {
        self.fork()
    }

    #[pyo3(name = "reset")]
    #[inline]
    #[doc = "Resets the context to the first bar and marks it as uninitialized."]
    pub fn py_reset(&self) {
        self.ctx.borrow_mut().reset();
    }

    #[pyo3(name = "next")]
    #[inline]
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
    pub fn py_iter(s: PyRefMut<Self>) -> PyRefMut<Self> {
        s
    }
}
