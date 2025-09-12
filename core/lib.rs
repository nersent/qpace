#[cfg(feature = "polars")]
extern crate polars;

#[cfg(feature = "bindings_node")]
#[macro_use]
extern crate napi_derive;

#[cfg(feature = "bindings_node")]
use napi::bindgen_prelude::*;
#[cfg(feature = "bindings_node")]
use napi_derive::napi;

extern crate num_traits;

#[macro_use]
extern crate num_derive;

mod backtest_test;

pub mod backtest;
pub mod ctx;
pub mod legacy;
pub mod metrics;
pub mod ohlcv;
pub mod orderbook;
pub mod plot;
pub mod signal;
pub mod stats;
pub mod sym;
pub mod timeframe;
pub mod trade;
pub mod utils;
use trade::{Trade, TradeDirection, TradeEvent};

cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  extern crate pyo3;
  use pyo3::prelude::*;
  use pyo3::{
    pyfunction, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction, Bound, PyResult,
  };
  use pyo3::{wrap_pymodule};
  use pyo3_stub_gen::{define_stub_info_gatherer, derive::gen_stub_pyfunction};
  pub mod timeframe_py;
  pub mod sym_py;
  pub mod stats_py;
  pub mod metrics_py;
  pub mod ohlcv_py;
  pub mod ctx_py;
  pub mod trade_py;
  pub mod signal_py;
  pub mod orderbook_py;
  pub mod backtest_py;
  pub mod plot_py;
  use timeframe_py::PyTimeframe;
  use sym_py::PySym;
  use sym_py::PySymKind;
  use ohlcv_py::PyOhlcv;
  use ctx_py::PyCtx;
  use backtest_py::{PyBacktest};
  use signal_py::PySignal;
}}
// cfg_if::cfg_if! { if #[cfg(all(feature = "bindings_wasm", target_arch = "wasm32"))] {
cfg_if::cfg_if! { if #[cfg(all(feature = "bindings_wasm"))] {
  use wasm_bindgen::prelude::*;
  pub mod timeframe_wasm;
  pub mod sym_wasm;
  pub mod stats_wasm;
  pub mod metrics_wasm;
  pub mod ohlcv_wasm;
  pub mod ctx_wasm;
  pub mod trade_wasm;
  pub mod signal_wasm;
  pub mod orderbook_wasm;
  pub mod backtest_wasm;
}}

// napi-rs can't expand `napi` macro if we enclose in `cfg_if`
#[cfg(feature = "bindings_node")]
pub mod backtest_node;
#[cfg(feature = "bindings_node")]
pub mod ctx_node;
#[cfg(feature = "bindings_node")]
pub mod metrics_node;
#[cfg(feature = "bindings_node")]
pub mod ohlcv_node;
#[cfg(feature = "bindings_node")]
pub mod orderbook_node;
#[cfg(feature = "bindings_node")]
pub mod signal_node;
#[cfg(feature = "bindings_node")]
pub mod stats_node;
#[cfg(feature = "bindings_node")]
pub mod sym_node;
#[cfg(feature = "bindings_node")]
pub mod timeframe_node;
#[cfg(feature = "bindings_node")]
pub mod trade_node;

#[cfg_attr(feature = "bindings_py", pyfunction(name = "_get_core_version"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = _getCoreVersion))]
#[inline]
pub fn get_version() -> String {
    return env!("CARGO_PKG_VERSION").to_string();
}

#[cfg_attr(feature = "bindings_node", napi(js_name = _getCoreVersion))]
#[inline]
pub fn node_get_version() -> String {
    return get_version();
}

cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
#[pymodule(name = "qpace_core")]
fn py_lib_mod(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTimeframe>()?;
    m.add_class::<PySym>()?;
    m.add_class::<PySymKind>()?;
    m.add_class::<PyOhlcv>()?;
    m.add_class::<PyCtx>()?;
    m.add_class::<Trade>()?;
    m.add_class::<TradeEvent>()?;
    m.add_class::<TradeDirection>()?;
    m.add_class::<PyBacktest>()?;
    m.add_class::<PySignal>()?;
    m.add_class::<ohlcv::OhlcvBar>()?;
    m.add_class::<plot_py::PyLineStyle>()?;
    m.add_class::<plot_py::PyPosition>()?;
    m.add_function(wrap_pyfunction!(get_version, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_expectancy, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_expectancy_score, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_pnl, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_profit_factor, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_long_net_profit_ratio, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_win_rate, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_avg_trade, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_avg_winning_trade, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_avg_losing_trade, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_avg_win_loss_ratio, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_sharpe_ratio, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_sortino_ratio, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_sharpe_ratio_from_returns, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_sortino_ratio_from_returns, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_omega_ratio_from_returns, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_omega_ratio, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_net_profit_pct, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_gross_profit_pct, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_gross_loss_pct, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_long_net_profit_pct, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_short_net_profit_pct, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_accuracy, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_precision, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_recall, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_f1, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py::py_annualization_factor, m)?)?;
    Ok(())
}
define_stub_info_gatherer!(stub_info);
}}

// cfg_if::cfg_if! { if #[cfg(feature = "bindings_node")] {
//   use napi::bindgen_prelude::JsObjectValue;
// #[napi(module_exports)]
// pub fn node_exports(mut export: napi::bindgen_prelude::Object) -> napi::bindgen_prelude::Result<()> {
//   let symbol = napi::bindgen_prelude::Symbol::for_desc("NAPI_RS_SYMBOL");
//   export.set_named_property("NAPI_RS_SYMBOL", symbol)?;
//   Ok(())
// }
// }}
