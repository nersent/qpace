cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}
use crate::trade::{Trade, TradeDirection, TradeEvent};

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(js_class=TradeEvent)]
impl TradeEvent {
    #[wasm_bindgen(getter = id)]
    #[inline]
    pub fn js_id(&self) -> Option<String> {
        self.id().cloned()
    }

    #[wasm_bindgen(getter = orderBarIndex)]
    #[inline]
    pub fn js_order_bar_index(&self) -> usize {
        self.order_bar_index()
    }

    #[wasm_bindgen(getter = fillBarIndex)]
    #[inline]
    pub fn js_fill_bar_index(&self) -> usize {
        self.fill_bar_index()
    }

    #[wasm_bindgen(getter = price)]
    #[inline]
    pub fn js_price(&self) -> f64 {
        self.price()
    }

    #[wasm_bindgen(getter = comment)]
    #[inline]
    pub fn js_comment(&self) -> Option<String> {
        self.comment().cloned()
    }
}

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(js_class=Trade)]
impl Trade {
    #[wasm_bindgen(getter = size)]
    #[inline]
    pub fn js_size(&self) -> f64 {
        self.size()
    }

    #[wasm_bindgen(getter = entry)]
    #[inline]
    pub fn js_entry(&self) -> Option<TradeEvent> {
        self.entry().cloned()
    }

    #[wasm_bindgen(getter = exit)]
    #[inline]
    pub fn js_exit(&self) -> Option<TradeEvent> {
        self.exit().cloned()
    }

    #[wasm_bindgen(getter = pnl)]
    #[inline]
    pub fn js_pnl(&self) -> f64 {
        self.pnl()
    }

    #[wasm_bindgen(getter = direction)]
    #[inline]
    pub fn js_direction(&self) -> TradeDirection {
        self.direction()
    }

    #[wasm_bindgen(getter = isActive)]
    #[inline]
    pub fn js_is_active(&self) -> bool {
        self.is_active()
    }

    #[wasm_bindgen(getter = isClosed)]
    #[inline]
    pub fn js_is_closed(&self) -> bool {
        self.is_closed()
    }
}
