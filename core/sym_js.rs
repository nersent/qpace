cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}
use crate::{
    ohlcv::{OhlcvReader, OhlcvWriter},
    rs_utils::get_oldest_possible_datetime,
    sym::Sym,
};

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(js_class=Sym)]
impl Sym {
    #[wasm_bindgen(getter = minTick)]
    #[inline]
    pub fn js_min_tick(&self) -> f64 {
        self.min_tick()
    }

    #[wasm_bindgen(getter = minQty)]
    #[inline]
    pub fn js_min_qty(&self) -> f64 {
        self.min_qty()
    }

    #[wasm_bindgen(js_name = "btc_usd")]
    #[inline]
    pub fn js_btc_usd() -> Self {
        Self::btc_usd()
    }

    #[wasm_bindgen(js_name = "eth_usd")]
    #[inline]
    pub fn js_eth_usd() -> Self {
        Self::eth_usd()
    }

    #[wasm_bindgen(js_name = "sol_usd")]
    #[inline]
    pub fn js_sol_usd() -> Self {
        Self::sol_usd()
    }
}
