extern crate num_traits;
#[macro_use]
extern crate num_derive;

cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  extern crate pyo3;
  use pyo3::prelude::*;
  use pyo3::{
    pyfunction, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction, Bound, PyResult,
  };
  use pyo3::{types::PyDateTime, wrap_pymodule};
  use pyo3_stub_gen::{define_stub_info_gatherer, derive::gen_stub_pyfunction};
  use backtest_py::PyBacktest;
  use ctx_py::PyCtx;
}}
cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}
use backtest::{Backtest, BacktestConfig};
use ctx::Ctx;
use ohlcv::OhlcvBar;
use ohlcv_py::PyOhlcv;
use orderbook::OrderConfig;
use signal::{Signal, SignalKind};
use sym::Sym;
use timeframe::Timeframe;
use trade::{Trade, TradeDirection, TradeEvent};

pub mod backtest;
pub mod backtest_js;
pub mod backtest_py;
mod backtest_test;
pub mod ctx;
pub mod ctx_js;
pub mod ctx_py;
pub mod ohlcv;
pub mod ohlcv_js;
pub mod ohlcv_py;
pub mod orderbook;
pub mod rs_utils;
pub mod signal;
pub mod signal_js;
pub mod signal_py;
pub mod sym;
pub mod sym_js;
pub mod sym_py;
pub mod test_utils;
pub mod timeframe;
pub mod timeframe_js;
pub mod trade;
pub mod trade_js;
pub mod trade_py;
pub mod utils;

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "get_version"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = getVersion))]
pub fn get_version() -> String {
    return "0.0.1".to_string();
}

cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
#[pymodule(name = "qpace_core")]
fn py_lib_mod(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<OhlcvBar>()?;
    m.add_class::<PyOhlcv>()?;
    m.add_class::<Timeframe>()?;
    m.add_class::<Sym>()?;
    m.add_class::<PyCtx>()?;
    m.add_class::<SignalKind>()?;
    m.add_class::<Signal>()?;
    m.add_class::<Trade>()?;
    m.add_class::<TradeDirection>()?;
    m.add_class::<TradeEvent>()?;
    m.add_class::<OrderConfig>()?;
    m.add_class::<BacktestConfig>()?;
    m.add_class::<PyBacktest>()?;
    m.add_function(wrap_pyfunction!(get_version, m)?)?;
    m.add_function(wrap_pyfunction!(utils::hl2, m)?)?;
    m.add_function(wrap_pyfunction!(utils::py_js_returns, m)?)?;
    m.add_function(wrap_pyfunction!(utils::py_expectancy, m)?)?;
    m.add_function(wrap_pyfunction!(utils::expectancy_score, m)?)?;
    m.add_function(wrap_pyfunction!(utils::pnl, m)?)?;
    m.add_function(wrap_pyfunction!(utils::profit_factor, m)?)?;
    m.add_function(wrap_pyfunction!(utils::long_net_profit_ratio, m)?)?;
    m.add_function(wrap_pyfunction!(utils::win_rate, m)?)?;
    m.add_function(wrap_pyfunction!(utils::avg_trade, m)?)?;
    m.add_function(wrap_pyfunction!(utils::avg_winning_trade, m)?)?;
    m.add_function(wrap_pyfunction!(utils::avg_losing_trade, m)?)?;
    m.add_function(wrap_pyfunction!(utils::avg_win_loss_ratio, m)?)?;
    m.add_function(wrap_pyfunction!(utils::omega_ratio, m)?)?;
    m.add_function(wrap_pyfunction!(utils::sharpe_ratio, m)?)?;
    m.add_function(wrap_pyfunction!(utils::sortino_ratio, m)?)?;
    m.add_function(wrap_pyfunction!(utils::net_profit_pct, m)?)?;
    m.add_function(wrap_pyfunction!(utils::gross_profit_pct, m)?)?;
    m.add_function(wrap_pyfunction!(utils::gross_loss_pct, m)?)?;
    m.add_function(wrap_pyfunction!(utils::long_net_profit_pct, m)?)?;
    m.add_function(wrap_pyfunction!(utils::short_net_profit_pct, m)?)?;
    m.add_function(wrap_pyfunction!(utils::max_drawdown_pct, m)?)?;
    m.add_function(wrap_pyfunction!(utils::max_run_up_pct, m)?)?;
    m.add_function(wrap_pyfunction!(utils::kelly_criterion, m)?)?;
    m.add_function(wrap_pyfunction!(utils::sensitivity, m)?)?;
    m.add_function(wrap_pyfunction!(utils::round_contracts, m)?)?;
    m.add_function(wrap_pyfunction!(utils::validate_contracts, m)?)?;
    m.add_function(wrap_pyfunction!(utils::round_to_min_tick, m)?)?;
    m.add_function(wrap_pyfunction!(utils::order_size, m)?)?;
    m.add_function(wrap_pyfunction!(utils::order_size_for_equity_pct, m)?)?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
}}
