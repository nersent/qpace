use crate::ctx::CtxSkip;
use crate::ohlcv::ArcOhlcv;
use crate::ohlcv_py::PyOhlcv;
use crate::sym_py::PySym;
use pyo3::exceptions::PyStopIteration;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use std::{cell::RefCell, rc::Rc};

use crate::{
    ctx::Ctx,
    ohlcv::{OhlcvBar, OhlcvReader},
};
use chrono::{DateTime, Utc};

#[gen_stub_pyclass]
#[pyclass(name = "Ctx", unsendable)]
#[derive(Clone)]
pub struct PyCtx {
    inner: Rc<RefCell<Ctx>>,
    ohlcv: PyOhlcv,
}

impl Into<Rc<RefCell<Ctx>>> for PyCtx {
    fn into(self) -> Rc<RefCell<Ctx>> {
        self.inner
    }
}

impl PyCtx {
    #[inline]
    pub fn copy(&self) -> Self {
        Self {
            inner: Rc::new(RefCell::new(self.inner.borrow().copy())),
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

    #[inline]
    pub fn inner(&self) -> &Rc<RefCell<Ctx>> {
        &self.inner
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCtx {
    #[new]
    #[pyo3(signature = (ohlcv=None, sym=None))]
    pub fn py_new(ohlcv: Option<PyOhlcv>, sym: Option<PySym>) -> Self {
        let ohlcv = ohlcv.unwrap_or_else(|| PyOhlcv::default());
        let sym = sym.unwrap_or_else(|| PySym::default());
        let mut ctx = Ctx::new();
        let _ohlcv: ArcOhlcv = ohlcv.clone().into();
        ctx.set_ohlcv(_ohlcv.clone_box());
        ctx.set_sym(sym.into());
        Self {
            inner: Rc::new(RefCell::new(ctx)),
            ohlcv,
        }
    }

    #[getter(bar_index)]
    #[inline]
    pub fn py_bar_index(&self) -> usize {
        self.inner.borrow().bar_index()
    }

    #[getter(bar)]
    #[inline]
    pub fn py_bar(&self) -> OhlcvBar {
        self.inner.borrow().bar()
    }

    #[getter(is_initialized)]
    #[inline]
    pub fn py_is_initialized(&self) -> bool {
        self.inner.borrow().is_initialized()
    }

    #[getter(sym)]
    #[inline]
    pub fn py_sym(&self) -> PySym {
        self.inner.borrow().sym().clone().into()
    }

    #[getter(ohlcv)]
    #[inline]
    pub fn py_ohlcv(&self) -> PyOhlcv {
        self.ohlcv.clone()
    }

    #[pyo3(name = "copy")]
    #[inline]
    pub fn py_copy(&self) -> Self {
        self.copy()
    }

    #[pyo3(name = "reset")]
    #[inline]
    pub fn py_reset(&self) {
        self.inner.borrow_mut().reset();
    }

    #[pyo3(name = "next")]
    #[inline]
    pub fn py_next_(&self) -> Option<usize> {
        self.inner.borrow_mut().next()
    }

    #[pyo3(name = "__len__")]
    #[inline]
    pub fn py_len(&self) -> usize {
        self.inner.borrow().len()
    }

    #[pyo3(name = "__next__")]
    #[inline]
    pub fn py_next(&mut self) -> PyResult<usize> {
        let next = self.inner.borrow_mut().next();
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

    #[pyo3(name = "skip")]
    #[inline]
    pub fn py_skip(&mut self, skip: PyCtxSkip) {
        self.inner.borrow_mut().skip(skip.into());
    }

    #[pyo3(name = "ref")]
    #[inline]
    pub fn py_ref(&self) -> Self {
        self.clone()
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "CtxSkip")]
#[derive(Clone, Copy)]
pub struct PyCtxSkip {
    inner: CtxSkip,
}

impl Into<CtxSkip> for PyCtxSkip {
    fn into(self) -> CtxSkip {
        self.inner
    }
}

impl From<CtxSkip> for PyCtxSkip {
    fn from(inner: CtxSkip) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCtxSkip {
    #[pyo3(name = "end")]
    #[staticmethod]
    pub fn py_end() -> Self {
        CtxSkip::End.into()
    }

    #[pyo3(name = "bars")]
    #[staticmethod]
    pub fn py_bars(bars: usize) -> Self {
        CtxSkip::Bars(bars).into()
    }

    #[pyo3(name = "bar_index")]
    #[staticmethod]
    pub fn py_bar_index(bar_index: usize) -> Self {
        CtxSkip::BarIndex(bar_index).into()
    }

    #[pyo3(name = "open_time_eq")]
    #[staticmethod]
    pub fn py_open_time_eq(open_time: DateTime<Utc>) -> Self {
        CtxSkip::OpenTimeEq(open_time).into()
    }

    #[pyo3(name = "open_time_geq")]
    #[staticmethod]
    pub fn py_open_time_geq(open_time: DateTime<Utc>) -> Self {
        CtxSkip::OpenTimeGeq(open_time).into()
    }
}
