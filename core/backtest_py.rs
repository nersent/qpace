use crate::backtest::{BacktestSummary, BacktestSummaryConfig};
use crate::ctx_py::{PyCtx, PyCtxSkip};
use crate::signal_py::PySignal;
use crate::{
    backtest::{Backtest, BacktestConfig},
    trade::Trade,
};
use pyo3::exceptions::PyStopIteration;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

#[gen_stub_pyclass]
#[pyclass(name = "Backtest", unsendable)]
#[derive(Clone)]
pub struct PyBacktest {
    inner: Rc<RefCell<Backtest>>,
    ctx: PyCtx,
}

impl PyBacktest {
    #[inline]
    pub fn inner(&self) -> &Rc<RefCell<Backtest>> {
        &self.inner
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyBacktest {
    #[pyo3(signature = (ctx, initial_capital=1000.0, process_orders_on_close=false, debug=false))]
    #[new]
    #[inline]
    pub fn py_new(
        ctx: PyCtx,
        initial_capital: f64,
        process_orders_on_close: bool,
        debug: bool,
    ) -> Self {
        let mut config = BacktestConfig::default();
        config.set_initial_capital(initial_capital);
        config.set_process_orders_on_close(process_orders_on_close);
        config.set_debug(debug);
        Self {
            inner: Rc::new(RefCell::new(Backtest::new(ctx.inner().clone(), config))),
            ctx,
        }
    }

    #[getter(initial_capital)]
    #[inline]
    pub fn py_initial_capital(&self) -> f64 {
        self.inner.borrow().config().initial_capital()
    }

    #[getter(process_orders_on_close)]
    #[inline]
    pub fn py_process_orders_on_close(&self) -> bool {
        self.inner.borrow().config().process_orders_on_close()
    }

    #[getter(ctx)]
    #[inline]
    pub fn py_ctx(&self) -> PyCtx {
        self.ctx.clone()
    }

    #[getter(equity)]
    #[inline]
    pub fn py_equity(&self) -> f64 {
        self.inner.borrow().equity()
    }

    #[getter(net_equity)]
    #[inline]
    #[doc = "`initial_capital + net_profit`"]
    pub fn py_net_equity(&self) -> f64 {
        self.inner.borrow().net_equity()
    }

    #[getter(equity_list)]
    #[inline]
    pub fn py_equity_list(&self) -> Vec<f64> {
        self.inner.borrow().equity_list().to_vec()
    }

    #[getter(net_equity_list)]
    #[inline]
    pub fn py_net_equity_list(&self) -> Vec<f64> {
        self.inner.borrow().net_equity_list().to_vec()
    }

    #[getter(pnl_list)]
    #[inline]
    pub fn py_pnl_list(&self) -> Vec<f64> {
        self.inner.borrow().pnl_list()
    }

    #[getter(open_profit)]
    #[inline]
    pub fn py_open_profit(&self) -> f64 {
        self.inner.borrow().open_profit()
    }

    #[getter(net_profit)]
    #[inline]
    pub fn py_net_profit(&self) -> f64 {
        self.inner.borrow().net_profit()
    }

    #[getter(gross_profit)]
    #[inline]
    pub fn py_gross_profit(&self) -> f64 {
        self.inner.borrow().gross_profit()
    }

    #[getter(gross_loss)]
    #[inline]
    pub fn py_gross_loss(&self) -> f64 {
        self.inner.borrow().gross_loss()
    }

    #[getter(position_size)]
    #[inline]
    pub fn py_position_size(&self) -> f64 {
        self.inner.borrow().position_size()
    }

    #[getter(trades)]
    #[inline]
    pub fn py_trades(&self) -> Vec<Trade> {
        self.inner.borrow().trades().into_iter().cloned().collect()
    }

    #[pyo3(name = "on_bar_open")]
    #[inline]
    pub fn py_on_bar_open(&mut self) {
        self.inner.borrow_mut().on_bar_open();
    }

    #[pyo3(name = "on_bar_close")]
    #[inline]
    pub fn py_on_bar_close(&mut self) {
        self.inner.borrow_mut().on_bar_close();
    }

    #[pyo3(name = "signal")]
    #[inline]
    pub fn py_signal(&mut self, signal: PySignal) {
        self.inner.borrow_mut().signal(signal.into())
    }

    #[pyo3(name = "signal_list")]
    #[inline]
    pub fn py_signal_list(&mut self, signals: Vec<Option<PySignal>>) {
        self.inner
            .borrow_mut()
            .signal_list(signals.into_iter().map(|s| s.map(|s| s.into())).collect());
    }

    #[pyo3(name = "signal_dict")]
    #[inline]
    pub fn py_signal_dict(&mut self, signals: HashMap<usize, PySignal>) {
        self.inner
            .borrow_mut()
            .signal_map(signals.into_iter().map(|(k, v)| (k, v.into())).collect());
    }

    #[pyo3(name = "skip")]
    #[inline]
    pub fn py_skip(&mut self, skip: PyCtxSkip) {
        self.inner.borrow_mut().skip(skip.into());
    }

    #[pyo3(name = "__len__")]
    #[inline]
    pub fn py_len(&self) -> usize {
        self.inner.borrow().len()
    }

    #[pyo3(name = "__next__")]
    #[inline]
    pub fn py_next(&mut self) -> PyResult<usize> {
        let mut bt = self.inner.borrow_mut();
        let next = bt.ctx().borrow_mut().next();
        if next.is_none() {
            return Err(PyStopIteration::new_err("No more items"));
        }
        bt.on_bar_open();
        return Ok(next.unwrap());
    }

    #[pyo3(name = "__iter__")]
    #[inline]
    pub fn py_iter(s: PyRefMut<Self>) -> PyRefMut<Self> {
        s
    }

    #[pyo3(name = "to_pine")]
    #[inline]
    pub fn py_to_pine(&self) -> String {
        self.inner.borrow().to_pine()
    }

    #[pyo3(name = "summary", signature = (risk_free_rate=0.0))]
    #[inline]
    pub fn py_summary(&self, risk_free_rate: f64) -> PyBacktestSummary {
        self.inner
            .borrow()
            .summary(&BacktestSummaryConfig { risk_free_rate })
            .into()
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "BacktestSummary", unsendable)]
#[derive(Clone)]
pub struct PyBacktestSummary {
    inner: BacktestSummary,
}

impl Into<PyBacktestSummary> for BacktestSummary {
    fn into(self) -> PyBacktestSummary {
        PyBacktestSummary { inner: self }
    }
}

impl Into<BacktestSummary> for PyBacktestSummary {
    fn into(self) -> BacktestSummary {
        self.inner
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyBacktestSummary {
    #[pyo3(name = "print")]
    #[inline]
    pub fn py_print(&self) {
        self.inner.print(None);
    }

    #[pyo3(name = "to_dict")]
    #[inline]
    pub fn py_to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("equity", self.inner.equity)?;
        dict.set_item("net_equity", self.inner.net_equity)?;
        dict.set_item("net_profit", self.inner.net_profit)?;
        dict.set_item("profit_factor", self.inner.profit_factor)?;
        dict.set_item("win_rate", self.inner.win_rate)?;
        dict.set_item("position_size", self.inner.position_size)?;
        dict.set_item("closed_trades", self.inner.closed_trades)?;
        dict.set_item("open_trades", self.inner.open_trades)?;
        return Ok(dict.to_object(py));
    }
}
