
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
      
fn fn_main_39d375(ctx: &PaceContext) -> () {  }
#[derive(Clone)]struct state_fn_gownoWDupsku_aae924{ _54_lastValue: PaceFloat }
impl Default for state_fn_gownoWDupsku_aae924 {
      fn default() -> Self { state_fn_gownoWDupsku_aae924{ _54_lastValue: PaceNa.into() } }
    }
fn fn_gownoWDupsku_aae924(ctx: &PaceContext, state: &mut state_fn_gownoWDupsku_aae924, _53_x: &PaceSeries<PaceFloat>) -> PaceFloat { ; state._54_lastValue = _53_x.get(); _53_x.get_at_offset(usize::from(PaceInt(Some(0)))) + _53_x.get_at_offset(usize::from(PaceInt(Some(1)))) }

#[cfg_attr(feature = "bindings_py", pyclass(unsendable))]
#[derive(Clone)]
struct Incr_fn_gownoWDupsku_aae924 {
    ctx: PaceContext,
    state: state_fn_gownoWDupsku_aae924,
    _53_x: PaceSeries<PaceFloat>
}


impl Incr_fn_gownoWDupsku_aae924 {
    pub fn new(ctx: PaceContext) -> Self {
        Self {
            ctx,
        state: Default::default(),
          _53_x: Default::default()
        }
    }

    pub fn next(&mut self, _53_x: PaceFloat) -> Option<PaceFloat> {
    self._53_x.next();
self._53_x.set(_53_x);
      Some(fn_gownoWDupsku_aae924(&self.ctx,&mut self.state,&self._53_x))
    }
}

#[cfg(feature = "bindings_py")]
#[pymethods]
impl Incr_fn_gownoWDupsku_aae924 {
    #[new]
    pub fn py_new(__py: Python<'_>, ctx: &Bound<'_, PyAny>) -> Self {
        let ctx: PyCtx = PyCtx::downcast_py(__py, ctx);
        let ctx: Rc<RefCell<Ctx>> = ctx.into();
        let ctx: PaceContext = ctx.into();
        Self::new(ctx)
    }

    #[pyo3(name = "next")]
    pub fn py_next(&mut self, _53_x: PaceFloat) -> Option<PaceFloat> {
        self.ctx.next()?;
        self.next(_53_x)
    }

    #[cfg(feature = "bindings_py")]
    #[pyo3(name = "collect")]
    pub fn py_collect(&mut self, py: Python<'_>, _53_x: PaceSeries<PaceFloat>) -> PyResult<Vec<PaceFloat>> {
        let mut res: Vec<PaceFloat> = Vec::new();
        loop {
            let bar_index = self.ctx.next();
            if bar_index.is_none() { break; }
            let bar_index = bar_index.unwrap();
            let call_res = self.next(_53_x.get_at(bar_index));
            if call_res.is_none() { break };
            res.push(call_res.unwrap());
        }
        Ok(res)
    }

  

    #[pyo3(name = "_54_lastValue")]
    pub fn _54_lastValue(&self) -> PaceFloat {
        self.state._54_lastValue.clone()
    }


    
}
      
fn fn_GLOBAL_nz_a47bf4(ctx: &PaceContext, _65_x: PaceFloat) -> PaceFloat { if bool::from(_65_x.is_na()) { PaceFloat(0.0) } else { _65_x } }
fn fn_GLOBAL_na_3a0a41(ctx: &PaceContext, _63_x: PaceFloat) -> PaceBool { PaceBool(Some(false)) }
fn fn_library_de0527(ctx: &PaceContext, _67_title: PaceString) -> PaceNa { PaceNa }
fn fn_GLOBAL_input_float_db9400(ctx: &PaceContext, _69_defval: PaceFloat, _70_title: PaceString, _71_minval: PaceFloat, _72_maxval: PaceFloat, _73_step: PaceFloat) -> PaceFloat { PaceFloat(1.0) }
fn fn_GLOBAL__math_abs_bbb1d2(ctx: &PaceContext, _75_x: PaceFloat) -> PaceFloat { f64::abs(f64::from(_75_x)).into() }
fn fn_GLOBAL__math_sqrt_9edea5(ctx: &PaceContext, _77_x: PaceFloat) -> PaceFloat { f64::sqrt(f64::from(_77_x)).into() }
fn fn_GLOBAL__math_floor_76819d(ctx: &PaceContext, _79_x: PaceFloat) -> PaceFloat { f64::floor(f64::from(_79_x)).into() }
fn fn_GLOBAL__math_ceil_5647e8(ctx: &PaceContext, _81_x: PaceFloat) -> PaceFloat { f64::ceil(f64::from(_81_x)).into() }
fn fn_GLOBAL__math_round_194943(ctx: &PaceContext, _83_x: PaceFloat) -> PaceFloat { f64::round(f64::from(_83_x)).into() }
fn fn_GLOBAL__math_round_e29996(ctx: &PaceContext, _85_x: PaceFloat) -> PaceFloat { f64::exp(f64::from(_85_x)).into() }
fn fn_GLOBAL__math_log_0cdc05(ctx: &PaceContext, _87_x: PaceFloat) -> PaceFloat { f64::ln(f64::from(_87_x)).into() }
fn fn_GLOBAL__math_log10_386d24(ctx: &PaceContext, _89_x: PaceFloat) -> PaceFloat { f64::log10(f64::from(_89_x)).into() }
fn fn_GLOBAL__math_pow_3591a2(ctx: &PaceContext, _91_x: PaceFloat, _92_exponent: PaceFloat) -> PaceFloat { f64::powf(f64::from(_91_x), f64::from(_91_x)).into() }
fn fn_GLOBAL__math_avg_e7546b(ctx: &PaceContext, _94_a: PaceFloat, _95_b: PaceFloat) -> PaceFloat { (_94_a + _95_b) / PaceFloat(2.0) }
#[derive(Clone)]struct state_fn_math_counter_637a1a{ _98_x: PaceInt }
impl Default for state_fn_math_counter_637a1a {
      fn default() -> Self { state_fn_math_counter_637a1a{ _98_x: 0_i64.into() } }
    }
fn fn_math_counter_637a1a(ctx: &PaceContext, state: &mut state_fn_math_counter_637a1a, _97_cond: PaceBool) -> PaceInt { ; if bool::from(_97_cond) { state._98_x = state._98_x + PaceInt(Some(1)) }; state._98_x }
#[derive(Clone)]struct state_fn_math_counter_e47679{ call_math_counter_cde931: state_fn_math_counter_637a1a }
impl Default for state_fn_math_counter_e47679 {
      fn default() -> Self { state_fn_math_counter_e47679{ call_math_counter_cde931: Default::default() } }
    }
fn fn_math_counter_e47679(ctx: &PaceContext, state: &mut state_fn_math_counter_e47679) -> PaceInt { fn_math_counter_637a1a(&ctx, &mut state.call_math_counter_cde931, PaceBool(Some(true))) }

#[cfg(feature = "bindings_py")]
#[pyfunction(name = "get_core_version")]
pub fn py_get_core_version() -> String {
  return qpace_core::get_version();
}

#[cfg(feature = "bindings_py")]
#[pymodule(name = "_lib")]
fn py_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_get_core_version, m)?)?;
    m.add_class::<Incr_fn_gownoWDupsku_aae924>()?;
    Ok(())
}
      