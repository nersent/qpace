cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}
use crate::{
    signal::{Signal, SignalKind},
    sym::{SymInfo, Timeframe},
};

cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  #[wasm_bindgen(js_name = "SignalKind")]
  pub struct JsSignalKind {
      inner: SignalKind,
  }

  impl From<SignalKind> for JsSignalKind {
      fn from(inner: SignalKind) -> Self {
          JsSignalKind { inner }
      }
  }

  impl Into<SignalKind> for JsSignalKind {
      fn into(self) -> SignalKind {
          self.inner
      }
  }
}}

// #[cfg(feature = "bindings_wasm")]
// impl TryInto<SignalKind> for JsValue {
//     type Error = JsValue;

//     fn try_into(self) -> Result<SignalKind, Self::Error> {
//         let js_signal_kind: JsSignalKind = self.try_into();
//         Ok(js_signal_kind.into())
//     }
// }

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(js_class=Signal)]
impl Signal {
    #[wasm_bindgen(getter = kind)]
    #[inline]
    pub fn js_kind(&self) -> JsSignalKind {
        (*self.kind()).into()
    }

    #[wasm_bindgen(getter = id)]
    #[inline]
    pub fn js_id(&self) -> Option<String> {
        self.id().clone()
    }

    #[wasm_bindgen(setter = id)]
    #[inline]
    pub fn js_set_id(&mut self, id: Option<String>) {
        self.set_id(id);
    }

    #[wasm_bindgen(js_name = hold)]
    #[inline]
    pub fn js_hold(&self) -> Self {
        Self::hold()
    }

    #[wasm_bindgen(js_name = size)]
    #[inline]
    pub fn js_size(&self, size: f64) -> Self {
        Self::size(size)
    }

    #[wasm_bindgen(js_name = equityPct)]
    #[inline]
    pub fn js_equity_pct(&self, equity_pct: f64) -> Self {
        Self::equity_pct(equity_pct)
    }

    #[wasm_bindgen(js_name = closeAll)]
    #[inline]
    pub fn js_close_all(&self) -> Self {
        Self::close_all()
    }

    #[wasm_bindgen(js_name = long)]
    #[inline]
    pub fn js_long(&self) -> Self {
        Self::long()
    }

    #[wasm_bindgen(js_name = short)]
    #[inline]
    pub fn js_short(&self) -> Self {
        Self::short()
    }
}

// #[cfg(feature = "bindings_wasm")]
// impl TryInto<Signal> for wasm_bindgen::JsValue {}
