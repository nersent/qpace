
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
      
fn fn_main_f8bd22(ctx: &PaceContext) -> () { let mut _1_xd: PaceFloat = fn_gowno_a6e670(&ctx) * PaceFloat::from(PaceInt(Some(5))); }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_main_f8bd22(__py: Python<'_>, ctx: &Bound<'_, PyAny>) -> PyResult<Vec<()>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<()> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_main_f8bd22(&ctx);
  res.push(call_res);
}

return Ok(res);
   }
fn fn_gowno_a6e670(ctx: &PaceContext) -> PaceFloat { PaceFloat(69.420) }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_gowno_a6e670(__py: Python<'_>, ctx: &Bound<'_, PyAny>) -> PyResult<Vec<PaceFloat>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceFloat> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_gowno_a6e670(&ctx);
  res.push(call_res);
}

return Ok(res);
   }
fn fn_GLOBAL_nz_bc2a51(ctx: &PaceContext, _13_x: PaceFloat) -> PaceFloat { if bool::from(_13_x.is_na()) { PaceFloat(0.0) } else { _13_x } }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_GLOBAL_nz_bc2a51(__py: Python<'_>, ctx: &Bound<'_, PyAny>, _13_x: PaceFloat) -> PyResult<Vec<PaceFloat>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceFloat> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_GLOBAL_nz_bc2a51(&ctx, _13_x);
  res.push(call_res);
}

return Ok(res);
   }
fn fn_GLOBAL_na_1b61ac(ctx: &PaceContext, _11_x: PaceFloat) -> PaceBool { PaceBool(Some(false)) }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_GLOBAL_na_1b61ac(__py: Python<'_>, ctx: &Bound<'_, PyAny>, _11_x: PaceFloat) -> PyResult<Vec<PaceBool>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceBool> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_GLOBAL_na_1b61ac(&ctx, _11_x);
  res.push(call_res);
}

return Ok(res);
   }
fn fn_GLOBAL_input_float_ac22d8(ctx: &PaceContext, _15_defval: PaceFloat, _16_title: PaceString, _17_minval: PaceFloat, _18_maxval: PaceFloat, _19_step: PaceFloat) -> PaceFloat { PaceFloat(1.0) }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_GLOBAL_input_float_ac22d8(__py: Python<'_>, ctx: &Bound<'_, PyAny>, _15_defval: PaceFloat, _16_title: PaceString, _17_minval: PaceFloat, _18_maxval: PaceFloat, _19_step: PaceFloat) -> PyResult<Vec<PaceFloat>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceFloat> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_GLOBAL_input_float_ac22d8(&ctx, _15_defval, _16_title, _17_minval, _18_maxval, _19_step);
  res.push(call_res);
}

return Ok(res);
   }
fn fn_GLOBAL__math_abs_4d279f(ctx: &PaceContext, _21_x: PaceFloat) -> PaceFloat { f64::abs(f64::from(_21_x)).into() }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_GLOBAL__math_abs_4d279f(__py: Python<'_>, ctx: &Bound<'_, PyAny>, _21_x: PaceFloat) -> PyResult<Vec<PaceFloat>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceFloat> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_GLOBAL__math_abs_4d279f(&ctx, _21_x);
  res.push(call_res);
}

return Ok(res);
   }
fn fn_GLOBAL__math_sqrt_6817ac(ctx: &PaceContext, _23_x: PaceFloat) -> PaceFloat { f64::sqrt(f64::from(_23_x)).into() }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_GLOBAL__math_sqrt_6817ac(__py: Python<'_>, ctx: &Bound<'_, PyAny>, _23_x: PaceFloat) -> PyResult<Vec<PaceFloat>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceFloat> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_GLOBAL__math_sqrt_6817ac(&ctx, _23_x);
  res.push(call_res);
}

return Ok(res);
   }
fn fn_GLOBAL__math_floor_af6cda(ctx: &PaceContext, _25_x: PaceFloat) -> PaceFloat { f64::floor(f64::from(_25_x)).into() }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_GLOBAL__math_floor_af6cda(__py: Python<'_>, ctx: &Bound<'_, PyAny>, _25_x: PaceFloat) -> PyResult<Vec<PaceFloat>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceFloat> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_GLOBAL__math_floor_af6cda(&ctx, _25_x);
  res.push(call_res);
}

return Ok(res);
   }
fn fn_GLOBAL__math_ceil_7a0c88(ctx: &PaceContext, _27_x: PaceFloat) -> PaceFloat { f64::ceil(f64::from(_27_x)).into() }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_GLOBAL__math_ceil_7a0c88(__py: Python<'_>, ctx: &Bound<'_, PyAny>, _27_x: PaceFloat) -> PyResult<Vec<PaceFloat>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceFloat> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_GLOBAL__math_ceil_7a0c88(&ctx, _27_x);
  res.push(call_res);
}

return Ok(res);
   }
fn fn_GLOBAL__math_round_2c1475(ctx: &PaceContext, _29_x: PaceFloat) -> PaceFloat { f64::round(f64::from(_29_x)).into() }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_GLOBAL__math_round_2c1475(__py: Python<'_>, ctx: &Bound<'_, PyAny>, _29_x: PaceFloat) -> PyResult<Vec<PaceFloat>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceFloat> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_GLOBAL__math_round_2c1475(&ctx, _29_x);
  res.push(call_res);
}

return Ok(res);
   }
fn fn_GLOBAL__math_round_074a99(ctx: &PaceContext, _31_x: PaceFloat) -> PaceFloat { f64::exp(f64::from(_31_x)).into() }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_GLOBAL__math_round_074a99(__py: Python<'_>, ctx: &Bound<'_, PyAny>, _31_x: PaceFloat) -> PyResult<Vec<PaceFloat>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceFloat> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_GLOBAL__math_round_074a99(&ctx, _31_x);
  res.push(call_res);
}

return Ok(res);
   }
fn fn_GLOBAL__math_log_c01e65(ctx: &PaceContext, _33_x: PaceFloat) -> PaceFloat { f64::ln(f64::from(_33_x)).into() }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_GLOBAL__math_log_c01e65(__py: Python<'_>, ctx: &Bound<'_, PyAny>, _33_x: PaceFloat) -> PyResult<Vec<PaceFloat>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceFloat> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_GLOBAL__math_log_c01e65(&ctx, _33_x);
  res.push(call_res);
}

return Ok(res);
   }
fn fn_GLOBAL__math_log10_c76c26(ctx: &PaceContext, _35_x: PaceFloat) -> PaceFloat { f64::log10(f64::from(_35_x)).into() }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_GLOBAL__math_log10_c76c26(__py: Python<'_>, ctx: &Bound<'_, PyAny>, _35_x: PaceFloat) -> PyResult<Vec<PaceFloat>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceFloat> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_GLOBAL__math_log10_c76c26(&ctx, _35_x);
  res.push(call_res);
}

return Ok(res);
   }
fn fn_GLOBAL__math_pow_b2248e(ctx: &PaceContext, _37_x: PaceFloat, _38_exponent: PaceFloat) -> PaceFloat { f64::powf(f64::from(_37_x), f64::from(_37_x)).into() }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_GLOBAL__math_pow_b2248e(__py: Python<'_>, ctx: &Bound<'_, PyAny>, _37_x: PaceFloat, _38_exponent: PaceFloat) -> PyResult<Vec<PaceFloat>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceFloat> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_GLOBAL__math_pow_b2248e(&ctx, _37_x, _38_exponent);
  res.push(call_res);
}

return Ok(res);
   }
fn fn_GLOBAL__math_avg_6a9342(ctx: &PaceContext, _40_a: PaceFloat, _41_b: PaceFloat) -> PaceFloat { (_40_a + _41_b) / PaceFloat(2.0) }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_GLOBAL__math_avg_6a9342(__py: Python<'_>, ctx: &Bound<'_, PyAny>, _40_a: PaceFloat, _41_b: PaceFloat) -> PyResult<Vec<PaceFloat>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceFloat> = Vec::new();

loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_GLOBAL__math_avg_6a9342(&ctx, _40_a, _41_b);
  res.push(call_res);
}

return Ok(res);
   }
struct state_fn_math_counter_5d2f35{ _44_x: PaceInt }
impl Default for state_fn_math_counter_5d2f35 {
      fn default() -> Self { state_fn_math_counter_5d2f35{ _44_x: 0_i64.into() } }
    }
fn fn_math_counter_5d2f35(ctx: &PaceContext, state: &mut state_fn_math_counter_5d2f35, _43_cond: PaceBool) -> PaceInt { ; if bool::from(_43_cond) { state._44_x = state._44_x + PaceInt(Some(1)) }; state._44_x }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_math_counter_5d2f35(__py: Python<'_>, ctx: &Bound<'_, PyAny>, _43_cond: PaceBool) -> PyResult<Vec<PaceInt>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceInt> = Vec::new();
let mut state: state_fn_math_counter_5d2f35 = Default::default();
loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_math_counter_5d2f35(&ctx, &mut state, _43_cond);
  res.push(call_res);
}

return Ok(res);
   }
struct state_fn_math_counter_b409c7{ call_math_counter_25d544: state_fn_math_counter_5d2f35 }
impl Default for state_fn_math_counter_b409c7 {
      fn default() -> Self { state_fn_math_counter_b409c7{ call_math_counter_25d544: Default::default() } }
    }
fn fn_math_counter_b409c7(ctx: &PaceContext, state: &mut state_fn_math_counter_b409c7) -> PaceInt { fn_math_counter_5d2f35(&ctx, &mut state.call_math_counter_25d544, PaceBool(Some(true))) }
#[cfg(feature = "bindings_py")]
#[pyfunction]fn py_fn_math_counter_b409c7(__py: Python<'_>, ctx: &Bound<'_, PyAny>) -> PyResult<Vec<PaceInt>> { 
let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
let ctx: Rc<RefCell<Ctx>> = ctx.into();
let ctx: PaceContext = ctx.into();
let mut res: Vec<PaceInt> = Vec::new();
let mut state: state_fn_math_counter_b409c7 = Default::default();
loop {
  let bar_index = ctx.next();
  if bar_index.is_none() {
    break;
  }
  let mut call_res = fn_math_counter_b409c7(&ctx, &mut state);
  res.push(call_res);
}

return Ok(res);
   }

#[cfg(feature = "bindings_py")]
#[pymodule(name = "qpace_script_e0ffe1")]
fn py_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_fn_main_f8bd22, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_gowno_a6e670, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_GLOBAL_nz_bc2a51, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_GLOBAL_na_1b61ac, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_GLOBAL_input_float_ac22d8, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_GLOBAL__math_abs_4d279f, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_GLOBAL__math_sqrt_6817ac, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_GLOBAL__math_floor_af6cda, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_GLOBAL__math_ceil_7a0c88, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_GLOBAL__math_round_2c1475, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_GLOBAL__math_round_074a99, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_GLOBAL__math_log_c01e65, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_GLOBAL__math_log10_c76c26, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_GLOBAL__math_pow_b2248e, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_GLOBAL__math_avg_6a9342, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_math_counter_5d2f35, m)?)?;
m.add_function(wrap_pyfunction!(py_fn_math_counter_b409c7, m)?)?;
    Ok(())
}
      