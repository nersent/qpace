cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
  use crate::ohlcv_js::{JsOhlcv};
}}
use crate::{
    ctx::Ctx,
    ohlcv::{Ohlcv, OhlcvBar, OhlcvReader, OhlcvWriter},
    rs_utils::get_oldest_possible_datetime,
    sym::Sym,
};
use std::{cell::RefCell, rc::Rc};

#[cfg(feature = "bindings_wasm")]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name=Ctx))]
#[derive(Clone)]
pub struct JsCtx {
    ctx: Rc<RefCell<Ctx>>,
    ohlcv: JsOhlcv,
}

#[cfg(feature = "bindings_wasm")]
impl JsCtx {
    #[inline]
    pub fn fork(&self) -> Self {
        Self {
            ctx: Rc::new(RefCell::new(self.ctx.borrow().fork())),
            ohlcv: self.ohlcv.clone(),
        }
    }
}

#[cfg(feature = "bindings_wasm")]
impl Into<Rc<RefCell<Ctx>>> for JsCtx {
    fn into(self) -> Rc<RefCell<Ctx>> {
        self.ctx
    }
}

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(js_class=Ctx)]
impl JsCtx {
    #[wasm_bindgen(constructor)]
    pub fn js_new(ohlcv: JsOhlcv, sym: Option<Sym>) -> Self {
        let sym = sym.unwrap_or_else(|| Sym::default());
        let timeframe = ohlcv.js_timeframe();
        let mut ctx = Ctx::new();
        ctx.set_ohlcv(ohlcv.clone_box());
        ctx.set_sym(sym);
        ctx.set_timeframe(timeframe.into());
        Self {
            ctx: Rc::new(RefCell::new(ctx)),
            ohlcv,
        }
    }

    #[wasm_bindgen(getter = barIndex)]
    #[inline]
    pub fn js_bar_index(&self) -> usize {
        self.ctx.borrow().bar_index()
    }

    #[wasm_bindgen(getter = bar)]
    #[inline]
    pub fn js_bar(&self) -> OhlcvBar {
        *self.ctx.borrow().bar()
    }

    #[wasm_bindgen(getter = isInitialized)]
    #[inline]
    pub fn js_is_initialized(&self) -> bool {
        self.ctx.borrow().is_initialized()
    }

    #[wasm_bindgen(getter = sym)]
    #[inline]
    pub fn js_sym(&self) -> Sym {
        self.ctx.borrow().sym().clone()
    }

    #[wasm_bindgen(getter = ohlcv)]
    #[inline]
    pub fn js_ohlcv(&self) -> JsOhlcv {
        self.ohlcv.clone()
    }

    #[wasm_bindgen(js_name = fork)]
    #[inline]
    #[doc = "Creates a new instance starting from first bar. Reuses same OHLCV and symbol."]
    pub fn js_fork(&self) -> Self {
        self.fork()
    }

    #[wasm_bindgen(js_name = reset)]
    #[inline]
    #[doc = "Resets the context to the first bar and marks it as uninitialized."]
    pub fn js_reset(&self) {
        self.ctx.borrow_mut().reset();
    }

    #[wasm_bindgen(js_name = next)]
    #[inline]
    pub fn js_next(&self) -> Option<usize> {
        self.ctx.borrow_mut().next()
    }

    #[wasm_bindgen(getter = length)]
    #[inline]
    pub fn js_len(&self) -> usize {
        self.ctx.borrow().len()
    }
}
