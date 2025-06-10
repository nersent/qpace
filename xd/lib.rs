
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
      
fn fn_main_7a798e(ctx: &PaceContext) -> () {  }
fn fn_gownoWDupsku_19f4b6(ctx: &PaceContext, _206_x: &PaceSeries<PaceFloat>) -> PaceFloat { _206_x.get_at(usize::from(PaceInt(Some(1)))) + _206_x.get() }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_gownoWDupsku_19f4b6(__py: Python<'_>, ctx: &Bound<'_, PyAny>, _206_x: PaceSeries<PaceFloat>) -> PyResult<Vec<PaceFloat>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceFloat> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_gownoWDupsku_19f4b6(&ctx, &_206_x);
  res.push(call_res);
}

return Ok(res);
   }
fn fn_GLOBAL_nz_b8efb8(ctx: &PaceContext, _217_x: PaceFloat) -> PaceFloat { if bool::from(_217_x.is_na()) { PaceFloat(0.0) } else { _217_x } }
fn fn_GLOBAL_na_294732(ctx: &PaceContext, _215_x: PaceFloat) -> PaceBool { PaceBool(Some(false)) }
fn fn_library_b967c2(ctx: &PaceContext, _219_title: PaceString) -> PaceNa { PaceNa }
fn fn_GLOBAL_input_float_210078(ctx: &PaceContext, _221_defval: PaceFloat, _222_title: PaceString, _223_minval: PaceFloat, _224_maxval: PaceFloat, _225_step: PaceFloat) -> PaceFloat { PaceFloat(1.0) }
fn fn_GLOBAL__math_abs_48269b(ctx: &PaceContext, _227_x: PaceFloat) -> PaceFloat { f64::abs(f64::from(_227_x)).into() }
fn fn_GLOBAL__math_sqrt_1741dc(ctx: &PaceContext, _229_x: PaceFloat) -> PaceFloat { f64::sqrt(f64::from(_229_x)).into() }
fn fn_GLOBAL__math_floor_340246(ctx: &PaceContext, _231_x: PaceFloat) -> PaceFloat { f64::floor(f64::from(_231_x)).into() }
fn fn_GLOBAL__math_ceil_586670(ctx: &PaceContext, _233_x: PaceFloat) -> PaceFloat { f64::ceil(f64::from(_233_x)).into() }
fn fn_GLOBAL__math_round_7d8606(ctx: &PaceContext, _235_x: PaceFloat) -> PaceFloat { f64::round(f64::from(_235_x)).into() }
fn fn_GLOBAL__math_round_2d3f1a(ctx: &PaceContext, _237_x: PaceFloat) -> PaceFloat { f64::exp(f64::from(_237_x)).into() }
fn fn_GLOBAL__math_log_233147(ctx: &PaceContext, _239_x: PaceFloat) -> PaceFloat { f64::ln(f64::from(_239_x)).into() }
fn fn_GLOBAL__math_log10_7849e7(ctx: &PaceContext, _241_x: PaceFloat) -> PaceFloat { f64::log10(f64::from(_241_x)).into() }
fn fn_GLOBAL__math_pow_6caa86(ctx: &PaceContext, _243_x: PaceFloat, _244_exponent: PaceFloat) -> PaceFloat { f64::powf(f64::from(_243_x), f64::from(_243_x)).into() }
fn fn_GLOBAL__math_avg_ea3a95(ctx: &PaceContext, _246_a: PaceFloat, _247_b: PaceFloat) -> PaceFloat { (_246_a + _247_b) / PaceFloat(2.0) }
struct state_fn_math_counter_fbd3c5{ _250_x: PaceInt }
impl Default for state_fn_math_counter_fbd3c5 {
      fn default() -> Self { state_fn_math_counter_fbd3c5{ _250_x: 0_i64.into() } }
    }
fn fn_math_counter_fbd3c5(ctx: &PaceContext, state: &mut state_fn_math_counter_fbd3c5, _249_cond: PaceBool) -> PaceInt { ; if bool::from(_249_cond) { state._250_x = state._250_x + PaceInt(Some(1)) }; state._250_x }
struct state_fn_math_counter_22cd93{ call_math_counter_80a499: state_fn_math_counter_fbd3c5 }
impl Default for state_fn_math_counter_22cd93 {
      fn default() -> Self { state_fn_math_counter_22cd93{ call_math_counter_80a499: Default::default() } }
    }
fn fn_math_counter_22cd93(ctx: &PaceContext, state: &mut state_fn_math_counter_22cd93) -> PaceInt { fn_math_counter_fbd3c5(&ctx, &mut state.call_math_counter_80a499, PaceBool(Some(true))) }

#[cfg(feature = "bindings_py")]
#[pyfunction(name = "get_core_version")]
pub fn py_get_core_version() -> String {
  return qpace_core::get_version();
}

#[cfg(feature = "bindings_py")]
#[pymodule(name = "_lib")]
fn py_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_get_core_version, m)?)?;
    m.add_function(wrap_pyfunction!(py_fn_gownoWDupsku_19f4b6, m)?)?;
    Ok(())
}
      