cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}
use crate::{
    ctx::Ctx,
    ohlcv::{ArcOhlcv, Ohlcv, OhlcvBar, OhlcvLoader, OhlcvReader, OhlcvWriter},
    rs_utils::get_oldest_possible_datetime,
    sym::SymInfo,
};
use std::{cell::RefCell, rc::Rc};

#[cfg(feature = "bindings_wasm")]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name=Ctx))]
#[derive(Clone)]
pub struct JsCtx {
    ctx: Rc<RefCell<Ctx>>,
}

#[cfg(feature = "bindings_wasm")]
impl JsCtx {
    #[inline]
    pub fn new(ohlcv: Box<dyn OhlcvReader>, sym_info: SymInfo) -> Self {
        Self {
            ctx: Rc::new(RefCell::new(Ctx::new(ohlcv, sym_info))),
        }
    }

    #[inline]
    pub fn fork(&self) -> Self {
        Self {
            ctx: Rc::new(RefCell::new(self.ctx.borrow().fork())),
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
    #[wasm_bindgen(js_name = fromArcOhlcv)]
    #[inline]
    pub fn js_from_arc_ohlcv(ohlcv: ArcOhlcv, sym_info: Option<SymInfo>) -> Self {
        let sym_info = sym_info.unwrap_or_else(|| SymInfo::default());
        Self::new(ohlcv.clone_box(), sym_info)
    }

    #[wasm_bindgen(js_name = fromOhlcv)]
    #[inline]
    pub fn js_from_ohlcv(ohlcv: OhlcvLoader, sym_info: Option<SymInfo>) -> Self {
        let sym_info = sym_info.unwrap_or_else(|| SymInfo::default());
        Self::new(ohlcv.clone_box(), sym_info)
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

    #[wasm_bindgen(getter = symInfo)]
    #[inline]
    pub fn js_sym_info(&self) -> SymInfo {
        *self.ctx.borrow().sym_info()
    }

    #[wasm_bindgen(getter = ohlcv)]
    #[inline]
    pub fn js_ohlcv(&self) -> Option<OhlcvLoader> {
        self.ctx.borrow().ohlcv().into()
    }

    #[wasm_bindgen(getter = arcOhlcv)]
    #[inline]
    pub fn js_arc_ohlcv(&self) -> Option<ArcOhlcv> {
        self.ctx.borrow().ohlcv().into()
    }

    #[wasm_bindgen(js_name = fork)]
    #[inline]
    pub fn js_fork(&self) -> Self {
        self.fork()
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
