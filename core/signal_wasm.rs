use crate::signal::{Signal, SignalKind};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "Signal")]
#[derive(Debug, Clone, PartialEq)]
pub struct WasmSignal {
    inner: Signal,
}

impl Into<Signal> for WasmSignal {
    #[inline]
    fn into(self) -> Signal {
        self.inner
    }
}

impl From<Signal> for WasmSignal {
    #[inline]
    fn from(inner: Signal) -> Self {
        WasmSignal { inner }
    }
}

#[wasm_bindgen(js_class = Signal)]
impl WasmSignal {
    #[wasm_bindgen(js_name = Hold)]
    #[inline]
    pub fn wasm_hold() -> Self {
        Signal::hold().into()
    }

    #[wasm_bindgen(js_name = Size)]
    #[inline]
    pub fn wasm_size(size: f64) -> Self {
        Signal::size(size).into()
    }

    #[wasm_bindgen(js_name = EquityPct)]
    #[inline]
    pub fn wasm_equity_pct(equity_pct: f64) -> Self {
        Signal::equity_pct(equity_pct).into()
    }

    #[wasm_bindgen(js_name = CloseAll)]
    #[inline]
    pub fn wasm_close_all() -> Self {
        Signal::close_all().into()
    }

    #[wasm_bindgen(js_name = Long)]
    #[inline]
    pub fn wasm_long() -> Self {
        Signal::long().into()
    }

    #[wasm_bindgen(js_name = Short)]
    #[inline]
    pub fn wasm_short() -> Self {
        Signal::short().into()
    }

    #[wasm_bindgen(getter = id)]
    #[inline]
    pub fn wasm_id(&self) -> Option<String> {
        self.inner.id().cloned()
    }

    #[wasm_bindgen(setter = id)]
    #[inline]
    pub fn wasm_set_id(&mut self, id: Option<String>) {
        self.inner.set_id(id);
    }

    #[wasm_bindgen(getter = comment)]
    #[inline]
    pub fn wasm_comment(&self) -> Option<String> {
        self.inner.comment().cloned()
    }

    #[wasm_bindgen(setter = comment)]
    #[inline]
    pub fn wasm_set_comment(&mut self, comment: Option<String>) {
        self.inner.set_comment(comment);
    }
}
