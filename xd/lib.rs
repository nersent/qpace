
use std::{cell::RefCell, rc::Rc};
mod runtime;
use runtime::*;
use qpace_core::ctx::Ctx;

cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  extern crate pyo3;
  use pyo3::prelude::*;
  use pyo3::{
    pyfunction, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction, Bound, PyResult,
  };
  use pyo3::{wrap_pymodule};
  use qpace_core::ctx_py::PyCtx;
}}
      
fn fn_GLOBAL_nz_780c9d(ctx: &PaceContext, _16_x: PaceFloat) -> PaceFloat { if bool::from(fn_GLOBAL_na_708d9e(&ctx, _16_x)) { PaceFloat(0.0) } else { _16_x } }
fn fn_GLOBAL_na_708d9e(ctx: &PaceContext, _14_x: PaceFloat) -> PaceBool { PaceBool(Some(false)) }
fn fn_library_1e0504(ctx: &PaceContext, _18_title: PaceString) -> PaceNa { PaceNa }
fn fn_GLOBAL_input_float_896ca1(ctx: &PaceContext, _20_defval: PaceFloat, _21_title: PaceString, _22_minval: PaceFloat, _23_maxval: PaceFloat, _24_step: PaceFloat) -> PaceFloat { PaceFloat(1.0) }
fn fn_GLOBAL__math_abs_c9a092(ctx: &PaceContext, _26_x: PaceFloat) -> PaceFloat { f64::abs(f64::from(_26_x)).into() }
fn fn_GLOBAL__math_sqrt_2148cf(ctx: &PaceContext, _28_x: PaceFloat) -> PaceFloat { f64::sqrt(f64::from(_28_x)).into() }
fn fn_GLOBAL__math_floor_860459(ctx: &PaceContext, _30_x: PaceFloat) -> PaceFloat { f64::floor(f64::from(_30_x)).into() }
fn fn_GLOBAL__math_ceil_14885b(ctx: &PaceContext, _32_x: PaceFloat) -> PaceFloat { f64::ceil(f64::from(_32_x)).into() }
fn fn_GLOBAL__math_round_f97694(ctx: &PaceContext, _34_x: PaceFloat) -> PaceFloat { f64::round(f64::from(_34_x)).into() }
fn fn_GLOBAL__math_round_cec83a(ctx: &PaceContext, _36_x: PaceFloat) -> PaceFloat { f64::exp(f64::from(_36_x)).into() }
fn fn_GLOBAL__math_log_419b0b(ctx: &PaceContext, _38_x: PaceFloat) -> PaceFloat { f64::ln(f64::from(_38_x)).into() }
fn fn_GLOBAL__math_log10_c1706f(ctx: &PaceContext, _40_x: PaceFloat) -> PaceFloat { f64::log10(f64::from(_40_x)).into() }
fn fn_GLOBAL__math_pow_1c80e5(ctx: &PaceContext, _42_x: PaceFloat, _43_exponent: PaceFloat) -> PaceFloat { f64::powf(f64::from(_42_x), f64::from(_42_x)).into() }
fn fn_GLOBAL__math_avg_43c717(ctx: &PaceContext, _45_a: PaceFloat, _46_b: PaceFloat) -> PaceFloat { (_45_a + _46_b) / PaceFloat(2.0) }
#[derive(Clone)]struct state_fn_math_counter_1dbdad{ _49_x: PaceInt }
impl Default for state_fn_math_counter_1dbdad {
      fn default() -> Self { state_fn_math_counter_1dbdad{ _49_x: 0_i64.into() } }
    }
fn fn_math_counter_1dbdad(ctx: &PaceContext, state: &mut state_fn_math_counter_1dbdad, _48_cond: PaceBool) -> PaceInt { ; if bool::from(_48_cond) { state._49_x = state._49_x + PaceInt(Some(1)) }; state._49_x }
#[derive(Clone)]struct state_fn_math_counter_7b0742{ call_math_counter_010bb8: state_fn_math_counter_1dbdad }
impl Default for state_fn_math_counter_7b0742 {
      fn default() -> Self { state_fn_math_counter_7b0742{ call_math_counter_010bb8: Default::default() } }
    }
fn fn_math_counter_7b0742(ctx: &PaceContext, state: &mut state_fn_math_counter_7b0742) -> PaceInt { fn_math_counter_1dbdad(&ctx, &mut state.call_math_counter_010bb8, PaceBool(Some(true))) }
fn fn_GLOBAL__plot_f67823(ctx: &PaceContext, _52_x: PaceFloat, _53_title: PaceString) -> PaceInt { PaceInt(Some(1)) }
fn fn_GLOBAL__float_2e68bf(ctx: &PaceContext, _55_x: PaceFloat) -> PaceFloat { _55_x }
fn fn_GLOBAL__float_a82fc6(ctx: &PaceContext, _57_x: PaceInt) -> PaceFloat { let mut _58__x: PaceFloat = PaceFloat::from(_57_x);; _58__x }
fn fn_GLOBAL__float_e2dd84(ctx: &PaceContext, _60_x: PaceBool) -> PaceFloat { if bool::from(_60_x) { PaceFloat(1.0) } else { PaceFloat(0.0) } }

#[cfg(feature = "bindings_py")]
#[pyfunction(name = "get_core_version")]
pub fn py_get_core_version() -> String {
  return qpace_core::get_version();
}

#[cfg(feature = "bindings_py")]
#[pymodule(name = "_lib")]
fn py_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_get_core_version, m)?)?;
    
    Ok(())
}
      