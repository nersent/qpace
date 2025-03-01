// use std::cell::{Ref, RefCell, RefMut};
// use std::rc::Rc;

// use pyo3::exceptions::{PyTypeError, PyValueError};
// use pyo3::prelude::*;
// use pyo3::types::{PySequence, PySlice, PySliceIndices};

// /// A simple Ohlcv type holding a vector of “bars”
// #[derive(Debug)]
// struct Ohlcv {
//     bars: Vec<String>,
// }

// impl Ohlcv {
//     /// Create a new empty Ohlcv.
//     fn new() -> Self {
//         Self { bars: vec![] }
//     }

//     /// Fork creates a copy of Ohlcv.
//     pub fn fork(&self) -> Ohlcv {
//         Ohlcv {
//             bars: self.bars.clone(),
//         }
//     }
// }

// /// A context that holds a reference (in this case a shared reference)
// /// to an Ohlcv.
// #[derive(Debug)]
// struct Ctx {
//     ohlcv: Rc<RefCell<Ohlcv>>,
// }

// impl Ctx {
//     fn new(ohlcv: Rc<RefCell<Ohlcv>>) -> Self {
//         Self { ohlcv }
//     }
// }

// /// Python wrapper for Ohlcv.
// #[pyclass(unsendable)]
// #[derive(Clone)]
// struct PyOhlcv {
//     inner: Rc<RefCell<Ohlcv>>,
// }

// impl PyOhlcv {
//     /// Borrow the inner Ohlcv immutably.
//     fn get_inner(&self) -> Ref<Ohlcv> {
//         self.inner.borrow()
//     }
//     /// Borrow the inner Ohlcv mutably.
//     fn get_inner_mut(&self) -> RefMut<Ohlcv> {
//         self.inner.borrow_mut()
//     }
// }

// #[pymethods]
// impl PyOhlcv {
//     #[new]
//     fn new() -> Self {
//         // Initially empty Ohlcv.
//         PyOhlcv {
//             inner: Rc::new(RefCell::new(Ohlcv::new())),
//         }
//     }

//     /// Adds a bar string to the Ohlcv.
//     fn add_bar(&self, bar: &str) {
//         let mut ohlcv = self.get_inner_mut();
//         ohlcv.bars.push(bar.to_owned());
//     }

//     /// Returns a copy of the vector of bar strings.
//     fn get_bars(&self) -> Vec<String> {
//         self.get_inner().bars.clone()
//     }

//     fn fork(&self) -> PyOhlcv {
//         let ohlcv = self.get_inner().fork();
//         PyOhlcv {
//             inner: Rc::new(RefCell::new(ohlcv)),
//         }
//     }
// }

// /// Python wrapper for Ctx.
// #[pyclass(unsendable)]
// #[derive(Clone)]
// struct PyCtx {
//     inner: Rc<RefCell<Ctx>>,
// }

// impl PyCtx {
//     /// Get an immutable borrow on the inner Ctx.
//     fn get_inner(&self) -> Ref<Ctx> {
//         self.inner.borrow()
//     }

//     /// Get a mutable borrow on the inner Ctx.
//     fn get_inner_mut(&self) -> RefMut<Ctx> {
//         self.inner.borrow_mut()
//     }
// }

// #[pymethods]
// impl PyCtx {
//     /// Create a new context from a given PyOhlcv.
//     #[new]
//     fn new(ohlcv: PyOhlcv) -> Self {
//         let ctx = Ctx::new(ohlcv.inner.clone());
//         PyCtx {
//             inner: Rc::new(RefCell::new(ctx)),
//         }
//     }

//     /// Returns a new PyOhlcv wrapping the same inner Ohlcv.
//     fn get_ohlcv(&self) -> PyOhlcv {
//         let ohlcv_rc = self.get_inner().ohlcv.clone();
//         PyOhlcv { inner: ohlcv_rc }
//     }
// }

// /// Python wrapper for Backtest.
// #[pyclass(unsendable)]
// #[derive(Clone)]
// struct PyBacktest {
//     ctx: Rc<RefCell<Ctx>>,
// }

// #[pymethods]
// impl PyBacktest {
//     /// Create a new backtest from a PyCtx.
//     #[new]
//     fn new(ctx: PyCtx) -> Self {
//         PyBacktest {
//             ctx: ctx.inner.clone(),
//         }
//     }

//     /// Returns the Ohlcv from backtest’s context.
//     fn get_ohlcv(&self) -> PyOhlcv {
//         let ctx = self.ctx.borrow();
//         let ohlcv_rc = ctx.ohlcv.clone();
//         PyOhlcv { inner: ohlcv_rc }
//     }
// }

// struct X<'a> {
//     x: &'a i32,
// }

// fn main() {
//     let x = 0_i32;
//     let a = X { x: &x };
//     let b = X { x: &x };
// }

// #[pymodule(name = "qpace_playground_rs")]
// fn qpace_playground_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_class::<PyOhlcv>()?;
//     m.add_class::<PyCtx>()?;
//     m.add_class::<PyBacktest>()?;
//     Ok(())
// }
