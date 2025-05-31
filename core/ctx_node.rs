use std::{cell::RefCell, rc::Rc};

use crate::{
    ctx::{Ctx, CtxSkip},
    ohlcv::{ArcOhlcv, Ohlcv, OhlcvBar, OhlcvReader, OhlcvWriter, RcOhlcv},
    ohlcv_node::{NodeOhlcv, NodeOhlcvBar},
    sym::Sym,
    sym_node::NodeSym,
    sym_wasm::WasmSym,
};
use chrono::{DateTime, Utc};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(iterator)]
#[derive(Clone)]
pub struct NodeCtx {
    inner: Rc<RefCell<Ctx>>,
    ohlcv: NodeOhlcv,
}

impl NodeCtx {
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

#[napi]
impl NodeCtx {
    #[napi(constructor)]
    pub fn new(ohlcv: &NodeOhlcv, sym: Option<&NodeSym>) -> Self {
        let sym = sym.cloned().unwrap_or_else(|| NodeSym::default());
        let mut ctx = Ctx::new();
        let _ohlcv: ArcOhlcv = ohlcv.into();
        ctx.set_ohlcv(_ohlcv.clone_box());
        ctx.set_sym(sym.into());
        Self {
            inner: Rc::new(RefCell::new(ctx)),
            ohlcv: ohlcv.clone(),
        }
    }

    #[napi(getter = barIndex)]
    #[inline]
    pub fn node_bar_index(&self) -> i32 {
        self.inner.borrow().bar_index() as i32
    }

    #[napi(getter = bar)]
    #[inline]
    pub fn node_bar(&self) -> NodeOhlcvBar {
        self.inner.borrow().bar().into()
    }

    #[napi(getter = isInitialized)]
    #[inline]
    pub fn node_is_initialized(&self) -> bool {
        self.inner.borrow().is_initialized()
    }

    #[napi(getter = sym)]
    #[inline]
    pub fn node_sym(&self) -> NodeSym {
        self.inner.borrow().sym().clone().into()
    }

    #[napi(getter = ohlcv)]
    #[inline]
    pub fn node_ohlcv(&self) -> NodeOhlcv {
        self.ohlcv.clone()
    }

    #[napi(js_name = "copy")]
    #[inline]
    pub fn node_copy(&self) -> Self {
        self.copy()
    }

    #[napi(js_name = "reset")]
    #[inline]
    pub fn node_reset(&self) {
        self.inner.borrow_mut().reset();
    }

    #[napi(js_name = "next")]
    #[inline]
    pub fn node_next(&self) -> Option<i32> {
        self.inner.borrow_mut().next().map(|i| i as i32)
    }

    #[napi(getter = length)]
    #[inline]
    pub fn node_length(&self) -> i32 {
        self.inner.borrow().len() as i32
    }

    #[napi(js_name = "skip")]
    pub fn node_skip(&mut self, skip: &NodeCtxSkip) {
        self.inner.borrow_mut().skip(skip.into());
    }
}

impl Generator for NodeCtx {
    type Yield = i32;
    type Next = ();
    type Return = ();

    fn next(&mut self, _v: Option<Self::Next>) -> Option<Self::Yield> {
        let next = self.inner.borrow_mut().next()?;
        return Some(next as i32);
    }
}

#[napi]
#[derive(Clone, Copy)]
pub struct NodeCtxSkip {
    inner: CtxSkip,
}

impl Into<CtxSkip> for &NodeCtxSkip {
    fn into(self) -> CtxSkip {
        self.inner
    }
}

impl From<CtxSkip> for NodeCtxSkip {
    fn from(inner: CtxSkip) -> Self {
        Self { inner }
    }
}

#[napi]
impl NodeCtxSkip {
    #[napi(js_name = "end")]
    pub fn node_end() -> Self {
        CtxSkip::End.into()
    }

    #[napi(js_name = "bars")]
    pub fn node_bars(bars: i32) -> Self {
        CtxSkip::Bars(bars as usize).into()
    }

    #[napi(js_name = "barIndex")]
    pub fn node_bar_index(bar_index: i32) -> Self {
        CtxSkip::BarIndex(bar_index as usize).into()
    }

    #[napi(js_name = "openTimeEq")]
    pub fn node_open_time_eq(open_time: DateTime<Utc>) -> Self {
        CtxSkip::OpenTimeEq(open_time).into()
    }

    #[napi(js_name = "openTimeGeq")]
    pub fn node_open_time_geq(open_time: DateTime<Utc>) -> Self {
        CtxSkip::OpenTimeGeq(open_time).into()
    }
}
