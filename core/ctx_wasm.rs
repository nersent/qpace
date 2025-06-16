use crate::{
    ctx::{Ctx, CtxSkip},
    ohlcv::{OhlcvBar, OhlcvReader, RcOhlcv},
    ohlcv_wasm::WasmOhlcv,
    sym_wasm::WasmSym,
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=Ctx)]
#[derive(Clone)]
pub struct WasmCtx {
    inner: Rc<RefCell<Ctx>>,
    ohlcv: WasmOhlcv,
}

impl Into<Rc<RefCell<Ctx>>> for WasmCtx {
    fn into(self) -> Rc<RefCell<Ctx>> {
        self.inner
    }
}

impl WasmCtx {
    #[inline]
    pub fn copy(&self) -> Self {
        Self {
            inner: Rc::new(RefCell::new(self.inner.borrow().copy())),
            ohlcv: self.ohlcv.clone(),
        }
    }

    #[inline]
    pub fn inner(&self) -> &Rc<RefCell<Ctx>> {
        &self.inner
    }
}

#[wasm_bindgen(js_class=Ctx)]
impl WasmCtx {
    #[wasm_bindgen(constructor)]
    pub fn wasm_new(ohlcv: Option<WasmOhlcv>, sym: Option<WasmSym>) -> Self {
        let ohlcv: WasmOhlcv = ohlcv.unwrap_or_else(|| WasmOhlcv::default());
        let sym = sym.unwrap_or_else(|| WasmSym::default());
        let mut ctx = Ctx::new();
        let _ohlcv: RcOhlcv = ohlcv.clone().into();
        ctx.set_ohlcv(_ohlcv.clone_box());
        ctx.set_sym(sym.into());
        Self {
            inner: Rc::new(RefCell::new(ctx)),
            ohlcv,
        }
    }

    #[wasm_bindgen(getter = barIndex)]
    #[inline]
    pub fn wasm_bar_index(&self) -> usize {
        self.inner.borrow().bar_index()
    }

    #[wasm_bindgen(getter = bar)]
    #[inline]
    pub fn wasm_bar(&self) -> OhlcvBar {
        self.inner.borrow().bar()
    }

    #[wasm_bindgen(getter = isInitialized)]
    #[inline]
    pub fn wasm_is_initialized(&self) -> bool {
        self.inner.borrow().is_initialized()
    }

    #[wasm_bindgen(getter = sym)]
    #[inline]
    pub fn wasm_sym(&self) -> WasmSym {
        self.inner.borrow().sym().clone().into()
    }

    #[wasm_bindgen(getter = ohlcv)]
    #[inline]
    pub fn wasm_ohlcv(&self) -> WasmOhlcv {
        self.ohlcv.clone()
    }

    #[wasm_bindgen(js_name = fork)]
    #[inline]
    pub fn js_copy(&self) -> Self {
        self.copy()
    }

    #[wasm_bindgen(js_name = reset)]
    #[inline]
    pub fn wasm_reset(&self) {
        self.inner.borrow_mut().reset();
    }

    #[wasm_bindgen(js_name = next)]
    #[inline]
    pub fn wasm_next(&self) -> Option<usize> {
        self.inner.borrow_mut().next()
    }

    #[wasm_bindgen(getter = length)]
    #[inline]
    pub fn wasm_length(&self) -> usize {
        self.inner.borrow().len()
    }
}

#[wasm_bindgen(js_name=CtxSkip)]
#[derive(Clone, Copy)]
pub struct WasmCtxSkip {
    inner: CtxSkip,
}

impl Into<CtxSkip> for WasmCtxSkip {
    fn into(self) -> CtxSkip {
        self.inner
    }
}

impl From<CtxSkip> for WasmCtxSkip {
    fn from(inner: CtxSkip) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen(js_class=CtxSkip)]
impl WasmCtxSkip {
    #[wasm_bindgen(js_name = "end")]
    pub fn node_end() -> Self {
        CtxSkip::End.into()
    }

    #[wasm_bindgen(js_name = "bars")]
    pub fn node_bars(bars: i32) -> Self {
        CtxSkip::Bars(bars as usize).into()
    }

    #[wasm_bindgen(js_name = "barIndex")]
    pub fn node_bar_index(bar_index: i32) -> Self {
        CtxSkip::BarIndex(bar_index as usize).into()
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_name = "openTimeEq")]
    pub fn node_open_time_eq(open_time: js_sys::Date) -> Self {
        CtxSkip::OpenTimeEq(open_time.into()).into()
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_name = "openTimeGeq")]
    pub fn node_open_time_geq(open_time: js_sys::Date) -> Self {
        CtxSkip::OpenTimeGeq(open_time.into()).into()
    }
}
