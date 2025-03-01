cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
  use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    types::{PySequence, PySlice, PySliceIndices},
  };
  use pyo3::exceptions::PyStopIteration;
  use crate::rs_utils::{pyslice_to_range};
  use pyo3::{types::PyDict};
  use crate::{ctx_py::PyCtx};
}}
use crate::{
    backtest::{Backtest, BacktestConfig},
    ctx::Ctx,
    ohlcv::{ArcOhlcv, Ohlcv, OhlcvBar, OhlcvLoader, OhlcvReader, OhlcvWriter},
    orderbook::{OrderBookError, OrderConfig},
    rs_utils::get_oldest_possible_datetime,
    signal::Signal,
    sym::SymInfo,
    trade::Trade,
};
use chrono::{DateTime, Utc};
use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

#[cfg(feature = "bindings_py")]
#[gen_stub_pymethods]
#[pymethods]
impl BacktestConfig {
    #[new]
    #[pyo3(signature = (initial_capital=1000.0, process_orders_on_close=false))]
    #[inline]
    pub fn py_new(initial_capital: f64, process_orders_on_close: bool) -> Self {
        Self::new(initial_capital, process_orders_on_close)
    }

    #[getter(initial_capital)]
    #[inline]
    pub fn py_initial_capital(&self) -> f64 {
        self.initial_capital()
    }

    #[getter(process_orders_on_close)]
    #[inline]
    pub fn py_process_orders_on_close(&self) -> bool {
        self.process_orders_on_close()
    }
}

#[cfg(feature = "bindings_py")]
#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "Backtest", unsendable))]
pub struct PyBacktest {
    py_ctx: PyCtx,
    bt: Rc<RefCell<Backtest>>,
}

#[cfg(feature = "bindings_py")]
impl PyBacktest {
    pub fn new(py_ctx: PyCtx, config: BacktestConfig) -> Self {
        Self {
            bt: Rc::new(RefCell::new(Backtest::new(py_ctx.clone().into(), config))),
            py_ctx,
        }
    }
}

#[cfg(feature = "bindings_py")]
impl Into<Rc<RefCell<Backtest>>> for PyBacktest {
    fn into(self) -> Rc<RefCell<Backtest>> {
        self.bt
    }
}

#[cfg(feature = "bindings_py")]
#[gen_stub_pymethods]
#[pymethods]
impl PyBacktest {
    #[new]
    #[inline]
    pub fn py_new(py_ctx: PyCtx, config: BacktestConfig) -> Self {
        Self::new(py_ctx, config)
    }

    #[getter(config)]
    #[inline]
    pub fn py_config(&self) -> BacktestConfig {
        *self.bt.borrow().config()
    }

    #[getter(ctx)]
    #[inline]
    pub fn py_ctx(&self) -> PyCtx {
        self.py_ctx.clone()
    }

    #[getter(equity)]
    #[inline]
    #[doc = "`initial capital + net profit + open profit`"]
    pub fn py_equity(&self) -> f64 {
        self.bt.borrow().equity()
    }

    #[getter(net_equity)]
    #[inline]
    #[doc = "`initial_capital + net_profit`"]
    pub fn py_net_equity(&self) -> f64 {
        self.bt.borrow().net_equity()
    }

    #[getter(equity_series)]
    #[inline]
    pub fn py_equity_series(&self) -> Vec<f64> {
        self.bt.borrow().equity_series().to_vec()
    }

    #[getter(net_equity_series)]
    #[inline]
    pub fn py_net_equity_series(&self) -> Vec<f64> {
        self.bt.borrow().net_equity_series().to_vec()
    }

    #[getter(equity_returns)]
    #[inline]
    pub fn py_equity_returns(&self) -> Vec<f64> {
        self.bt.borrow().equity_returns()
    }

    #[getter(net_equity_returns)]
    #[inline]
    pub fn py_net_equity_returns(&self) -> Vec<f64> {
        self.bt.borrow().net_equity_returns()
    }

    #[getter(pnl_series)]
    #[inline]
    pub fn py_pnl_series(&self) -> Vec<f64> {
        self.bt.borrow().pnl_series()
    }

    #[getter(open_profit)]
    #[inline]
    pub fn py_open_profit(&self) -> f64 {
        self.bt.borrow().open_profit()
    }

    #[getter(net_profit)]
    #[inline]
    #[doc = "Overall profit or loss."]
    pub fn py_net_profit(&self) -> f64 {
        self.bt.borrow().net_profit()
    }

    #[getter(gross_profit)]
    #[inline]
    #[doc = "Total value of all completed winning trades."]
    pub fn py_gross_profit(&self) -> f64 {
        self.bt.borrow().gross_profit()
    }

    #[getter(gross_loss)]
    #[inline]
    #[doc = "Total value of all completed losing trades."]
    pub fn py_gross_loss(&self) -> f64 {
        self.bt.borrow().gross_loss()
    }

    #[getter(winning_trades)]
    #[inline]
    #[doc = "Total number of winning trades."]
    pub fn py_winning_trades(&self) -> usize {
        self.bt.borrow().winning_trades()
    }

    #[getter(losing_trades)]
    #[inline]
    #[doc = "Total number of losing trades."]
    pub fn py_losing_trades(&self) -> usize {
        self.bt.borrow().losing_trades()
    }

    #[getter(position_size)]
    #[inline]
    #[doc = "Direction and size of the current market position. If the value is > 0, the market position is long. If the value is < 0, the market position is short. The absolute value is the number of contracts/shares/lots/units in trade (position size)."]
    pub fn py_position_size(&self) -> f64 {
        self.bt.borrow().position_size()
    }

    #[getter(open_trades)]
    #[inline]
    pub fn py_open_trades(&self) -> Vec<Trade> {
        self.bt.borrow().open_trades().to_vec()
    }

    #[getter(closed_trades)]
    #[inline]
    pub fn py_closed_trades(&self) -> Vec<Trade> {
        self.bt.borrow().closed_trades().to_vec()
    }

    #[getter(trades)]
    #[inline]
    pub fn py_trades(&self) -> Vec<Trade> {
        self.bt.borrow().trades().into_iter().cloned().collect()
    }

    #[getter(open_longs)]
    #[inline]
    pub fn py_open_longs(&self) -> usize {
        self.bt.borrow().open_longs()
    }

    #[getter(open_shorts)]
    #[inline]
    pub fn py_open_shorts(&self) -> usize {
        self.bt.borrow().open_shorts()
    }

    #[getter(closed_longs)]
    #[inline]
    pub fn py_closed_longs(&self) -> usize {
        self.bt.borrow().closed_longs()
    }

    #[getter(closed_shorts)]
    #[inline]
    pub fn py_closed_shorts(&self) -> usize {
        self.bt.borrow().closed_shorts()
    }

    #[getter(total_longs)]
    #[inline]
    pub fn py_total_longs(&self) -> usize {
        self.bt.borrow().total_longs()
    }

    #[getter(total_shorts)]
    #[inline]
    pub fn py_total_shorts(&self) -> usize {
        self.bt.borrow().total_shorts()
    }

    #[getter(total_trades)]
    #[inline]
    pub fn py_total_trades(&self) -> usize {
        self.bt.borrow().total_trades()
    }

    #[getter(instrument_price)]
    #[inline]
    pub fn py_instrument_size(&self) -> f64 {
        self.bt.borrow().instrument_size()
    }

    #[getter(win_rate)]
    #[inline]
    pub fn py_win_rate(&self) -> f64 {
        self.bt.borrow().win_rate()
    }

    #[getter(profit_factor)]
    #[inline]
    pub fn py_profit_factor(&self) -> f64 {
        self.bt.borrow().profit_factor()
    }

    #[pyo3(name = "on_bar_open")]
    #[inline]
    pub fn py_on_bar_open(&mut self) {
        self.bt.borrow_mut().on_bar_open();
    }

    #[pyo3(name = "on_bar_close")]
    #[inline]
    pub fn py_on_bar_close(&mut self) {
        self.bt.borrow_mut().on_bar_close();
    }

    // #[pyo3(name = "order")]
    // #[inline]
    // pub fn py_order(&mut self, order_config: OrderConfig) -> Result<usize, OrderBookError> {
    //     self.order(order_config)
    // }

    #[pyo3(name = "signal")]
    #[inline]
    pub fn py_signal(&mut self, signal: Signal) {
        self.bt.borrow_mut().signal(signal)
    }

    #[pyo3(name = "signal_batch")]
    #[inline]
    #[doc = "Processes multiple signals at once. `signals` must be aligned with all bars. `signals: [bar_index_0_signal, bar_index_1_signal, ...]`."]
    pub fn py_signal_batch(&mut self, signals: Vec<Option<Signal>>) {
        self.bt.borrow_mut().signal_batch(signals)
    }

    #[pyo3(name = "signal_batch_dict")]
    #[inline]
    #[doc = "Processes multiple signals at once. `signals: [bar_index, Signal]`."]
    pub fn py_signal_batch_dict(&mut self, signals: HashMap<usize, Signal>) {
        self.bt.borrow_mut().signal_batch_dict(signals)
    }

    #[pyo3(name = "skip_remaining_bars")]
    #[inline]
    pub fn py_skip_remaining_bars(&mut self) {
        self.bt.borrow_mut().skip_remaining_bars()
    }

    #[pyo3(name = "skip_to_bar")]
    #[inline]
    pub fn py_skip_to_bar(&mut self, bar_index: usize) {
        self.bt.borrow_mut().skip_to_bar(bar_index)
    }

    #[pyo3(name = "skip_bars")]
    #[inline]
    pub fn py_skip_bars(&mut self, bars: usize) {
        self.bt.borrow_mut().skip_bars(bars)
    }

    #[pyo3(name = "to_pine")]
    #[inline]
    pub fn py_to_pine(&self) -> String {
        self.bt.borrow().to_pine()
    }

    #[pyo3(name = "__len__")]
    #[inline]
    pub fn py_len(&self) -> usize {
        self.bt.borrow().len()
    }

    #[inline]
    #[pyo3(name = "summary")]
    pub fn py_summary(&self, py: Python<'_>) -> PyResult<PyObject> {
        let bt = self.bt.borrow();
        let dict = PyDict::new_bound(py);
        dict.set_item("equity", bt.equity())?;
        dict.set_item("net_equity", bt.net_equity())?;
        dict.set_item("net_profit", bt.net_profit())?;
        dict.set_item("profit_factor", bt.profit_factor())?;
        dict.set_item("win_rate", bt.win_rate())?;
        dict.set_item("position_size", bt.position_size())?;
        dict.set_item("closed_trades", bt.closed_trades().len())?;
        dict.set_item("open_trades", bt.open_trades().len())?;
        return Ok(dict.to_object(py));
    }

    #[pyo3(name = "__next__")]
    #[inline]
    pub fn py_next(&mut self) -> PyResult<usize> {
        let mut bt = self.bt.borrow_mut();
        let next = bt.ctx().borrow_mut().next();
        bt.on_bar_open();
        // self.on_bar_close();
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
