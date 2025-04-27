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
  use comfy_table::{Table as ComfyTable, Cell, Row, ContentArrangement};
  use comfy_table::{
      presets::{UTF8_FULL, UTF8_FULL_CONDENSED},
      Attribute, CellAlignment, Color,
    };
  use textplots::{Chart, Plot, Shape};
}}
use crate::{
    backtest::{Backtest, BacktestConfig},
    ctx::Ctx,
    ohlcv::{Ohlcv, OhlcvBar, OhlcvReader, OhlcvWriter},
    orderbook::{OrderBookError, OrderConfig},
    rs_utils::{get_oldest_possible_datetime, with_suffix},
    signal::Signal,
    sym::Sym,
    trade::Trade,
    utils::{sharpe_ratio_from_equity, sortino_ratio_from_equity},
};
use chrono::{DateTime, Utc};
use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

#[cfg(feature = "bindings_py")]
#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "Backtest", unsendable))]
#[derive(Clone)]
pub struct PyBacktest {
    py_ctx: PyCtx,
    bt: Rc<RefCell<Backtest>>,
}

#[cfg(feature = "bindings_py")]
impl PyBacktest {
    #[inline]
    pub fn new(ctx: PyCtx, config: BacktestConfig) -> Self {
        Self {
            bt: Rc::new(RefCell::new(Backtest::new(ctx.clone().into(), config))),
            py_ctx: ctx,
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
    #[pyo3(signature = (ctx, initial_capital=1000.0, process_orders_on_close=false))]
    #[new]
    #[inline]
    pub fn py_new(ctx: PyCtx, initial_capital: f64, process_orders_on_close: bool) -> Self {
        let mut config = BacktestConfig::default();
        config.set_initial_capital(initial_capital);
        config.set_process_orders_on_close(process_orders_on_close);
        Self::new(ctx, config)
    }

    #[getter(initial_capital)]
    #[inline]
    pub fn py_initial_capital(&self) -> f64 {
        self.bt.borrow().config().initial_capital()
    }

    #[getter(process_orders_on_close)]
    #[inline]
    pub fn py_process_orders_on_close(&self) -> bool {
        self.bt.borrow().config().process_orders_on_close()
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

    #[getter(equity_list)]
    #[inline]
    pub fn py_equity_list(&self) -> Vec<f64> {
        self.bt.borrow().equity_series().to_vec()
    }

    #[getter(net_equity_list)]
    #[inline]
    pub fn py_net_equity_list(&self) -> Vec<f64> {
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

    #[getter(pnl_list)]
    #[inline]
    pub fn py_pnl_list(&self) -> Vec<f64> {
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

    #[pyo3(name = "skip", signature = (bars=None, bar_index=None))]
    #[inline]
    pub fn py_skip(&mut self, bars: Option<usize>, bar_index: Option<usize>) {
        if bars.is_none() && bar_index.is_none() {
            self.bt.borrow_mut().skip_remaining_bars()
        } else if bars.is_some() {
            self.bt.borrow_mut().skip_bars(bars.unwrap())
        } else if bar_index.is_some() {
            self.bt.borrow_mut().skip_to_bar(bar_index.unwrap())
        }
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
    #[getter(metrics)]
    pub fn py_metrics(&self, py: Python<'_>) -> PyResult<PyObject> {
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

    fn print_metrics(&self) {
        let bt = self.bt.borrow();

        let currency = "USD";
        let f_price = with_suffix(&format!(" {}", currency));
        let f_percent = with_suffix("%");
        let f = |price: f64, percent: f64| format!("{}\n{}", f_price(price), f_percent(percent));
        let f_raw = |value: f64| format!("{:0.2}", value);

        let rfr = 0.0;

        let mut table = ComfyTable::new();
        table
            .set_content_arrangement(ContentArrangement::DynamicFullWidth)
            .set_header(vec!["Metric", "Value"]);

        table.add_row(Row::from(vec![
            Cell::new("Net Profit"),
            Cell::new(f(bt.net_profit(), bt.net_profit_pct() * 100.0)),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Gross Profit"),
            Cell::new(f(bt.gross_profit(), bt.gross_profit_pct() * 100.0)),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Gross Loss"),
            Cell::new(f(bt.gross_loss(), bt.gross_loss_pct() * 100.0)),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Sharpe Ratio"),
            Cell::new(format!(
                "{:0.3}",
                sharpe_ratio_from_equity(bt.equity_series().to_vec(), rfr)
            )),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Sortino Ratio"),
            Cell::new(format!(
                "{:0.3}",
                sortino_ratio_from_equity(bt.equity_series().to_vec(), rfr)
            )),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Profit Factor"),
            Cell::new(format!("{:0.3}", bt.profit_factor())),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Open P/L"),
            Cell::new(f_price(bt.open_profit())),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Total Closed Trades"),
            Cell::new(bt.closed_trades().len().to_string()),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Number Winning Trades"),
            Cell::new(bt.winning_trades().to_string()),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Number Losing Trades"),
            Cell::new(bt.losing_trades().to_string()),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("% Profitable"),
            Cell::new(f_percent(bt.win_rate() * 100.0)),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Avg Trade"),
            Cell::new(f_price(bt.avg_trade())),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Avg Winning Trade"),
            Cell::new(f_price(bt.avg_winning_trade())),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Avg Losing Trade"),
            Cell::new(f_price(bt.avg_losing_trade())),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Ratio Avg Win / Avg Loss"),
            Cell::new(f_raw(bt.avg_win_loss_ratio())),
        ]));

        // Print the table
        println!("{}", table);
    }

    #[pyo3(name = "print", signature = (plot=Some((120, 60))))]
    pub fn py_print(&self, plot: Option<(u32, u32)>) {
        self.print_metrics();

        let bt = self.bt.borrow();
        let currency = "USD";
        let f_price = with_suffix(&format!(" {}", currency));
        let f_percent = with_suffix("%");
        let f = |price: f64, percent: f64| format!("{} {}", f_price(price), f_percent(percent));
        let f_raw = |value: f64| format!("{:0.2}", value);

        let value_cell = |text: &str, theme: i32| {
            let mut cell = Cell::new(text)
                .set_alignment(CellAlignment::Left)
                .add_attribute(Attribute::Bold);

            cell = cell.fg(Color::White);

            match theme {
                1 => cell = cell.fg(Color::Green),
                -1 => cell = cell.fg(Color::Red),
                _ => {}
            }

            cell
        };

        let mut table = ComfyTable::new();
        table
            .load_preset(UTF8_FULL_CONDENSED)
            .apply_modifier(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::DynamicFullWidth);

        // Add a header row
        table.add_row(Row::from(vec![
            Cell::new("Net Profit").add_attribute(Attribute::Bold),
            Cell::new("Total Closed Trades").add_attribute(Attribute::Bold),
            Cell::new("% Profitable").add_attribute(Attribute::Bold),
            Cell::new("Profit Factor").add_attribute(Attribute::Bold),
            // Cell::new("Max Drawdown").add_attribute(Attribute::Bold),
            Cell::new("Avg Trade").add_attribute(Attribute::Bold),
        ]));

        table.add_row(vec![
            value_cell(
                &f(bt.net_profit(), bt.net_profit_pct() * 100.0),
                match bt.net_profit() {
                    x if x > 0.0 => 1,
                    x if x < 0.0 => -1,
                    _ => 0,
                },
            ),
            value_cell(&bt.closed_trades().len().to_string(), 0),
            value_cell(
                &f_percent(bt.win_rate() * 100.0),
                match bt.win_rate() {
                    x if x > 0.5 => 1,
                    x if x < 0.5 => -1,
                    _ => 0,
                },
            ),
            value_cell(
                &format!("{:0.3}", bt.profit_factor()),
                match bt.profit_factor() {
                    x if x > 1.0 => 1,
                    x if x < 1.0 => -1,
                    _ => 0,
                },
            ),
            // value_cell(
            //     &f(self.max_drawdown, self.max_drawdown_percent * 100.0),
            //     match self.max_drawdown {
            //         x if x < 0.2 => 1,
            //         x if x > 0.2 => -1,
            //         _ => 0,
            //     },
            // ),
            value_cell(&f_price(bt.avg_trade()), 0),
        ]);

        println!("{}", table);

        if plot.is_some() {
            self.text_plot_equity(plot);
        }
    }
}

#[cfg(feature = "bindings_py")]
impl PyBacktest {
    fn text_plot_equity(&self, size: Option<(u32, u32)>) {
        let bt = self.bt.borrow();
        let net_equity_series: Vec<f64> = bt.net_equity_series().to_vec();
        let net_equity_line: Vec<(f32, f32)> = net_equity_series
            .iter()
            .enumerate()
            .map(|(i, &value)| (i as f32 + 1.0, value as f32))
            .collect();
        let (w, h) = size.unwrap_or((120, 60));
        Chart::new(w, h, 1.0, net_equity_series.len() as f32)
            .lineplot(&Shape::Lines(&net_equity_line))
            .nice();
    }
}
