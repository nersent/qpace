use crate::trade::{Trade, TradeDirection, TradeEvent};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "TradeDirection")]
#[derive(Debug, Clone)]
pub struct WasmTradeDirection {
    inner: TradeDirection,
}

impl Into<WasmTradeDirection> for TradeDirection {
    #[inline]
    fn into(self) -> WasmTradeDirection {
        WasmTradeDirection { inner: self }
    }
}

impl Into<TradeDirection> for WasmTradeDirection {
    #[inline]
    fn into(self) -> TradeDirection {
        self.inner
    }
}

#[wasm_bindgen(js_class = TradeDirection)]
impl WasmTradeDirection {
    #[wasm_bindgen(js_name = "Long")]
    #[inline]
    pub fn wasm_long(&self) -> Self {
        TradeDirection::Long.into()
    }

    #[wasm_bindgen(js_name = "Short")]
    #[inline]
    pub fn wasm_short(&self) -> Self {
        TradeDirection::Short.into()
    }

    #[wasm_bindgen(js_name = "toNumber")]
    #[inline]
    pub fn wasm_to_number(&self) -> f64 {
        self.inner.into()
    }

    #[wasm_bindgen(js_name = "fromNumber")]
    #[inline]
    pub fn wasm_from_number(value: f64) -> Self {
        TradeDirection::from(value).into()
    }
}

#[wasm_bindgen(js_class=TradeEvent)]
impl TradeEvent {
    #[wasm_bindgen(getter = id)]
    #[inline]
    pub fn wasm_id(&self) -> Option<String> {
        self.id().cloned()
    }

    #[wasm_bindgen(getter = orderBarIndex)]
    #[inline]
    pub fn wasm_order_bar_index(&self) -> usize {
        self.order_bar_index()
    }

    #[wasm_bindgen(getter = fillBarIndex)]
    #[inline]
    pub fn wasm_fill_bar_index(&self) -> usize {
        self.fill_bar_index()
    }

    #[wasm_bindgen(getter = price)]
    #[inline]
    pub fn wasm_price(&self) -> f64 {
        self.price()
    }

    #[wasm_bindgen(getter = comment)]
    #[inline]
    pub fn wasm_comment(&self) -> Option<String> {
        self.comment().cloned()
    }
}

#[wasm_bindgen(js_class=Trade)]
impl Trade {
    #[wasm_bindgen(getter = size)]
    #[inline]
    pub fn wasm_size(&self) -> f64 {
        self.size()
    }

    #[wasm_bindgen(getter = entry)]
    #[inline]
    pub fn wasm_entry(&self) -> Option<TradeEvent> {
        self.entry().cloned()
    }

    #[wasm_bindgen(getter = exit)]
    #[inline]
    pub fn wasm_exit(&self) -> Option<TradeEvent> {
        self.exit().cloned()
    }

    #[wasm_bindgen(getter = pnl)]
    #[inline]
    pub fn wasm_pnl(&self) -> f64 {
        self.pnl()
    }

    #[wasm_bindgen(getter = direction)]
    #[inline]
    pub fn wasm_direction(&self) -> WasmTradeDirection {
        self.direction().into()
    }

    #[wasm_bindgen(getter = isActive)]
    #[inline]
    pub fn wasm_is_active(&self) -> bool {
        self.is_active()
    }

    #[wasm_bindgen(getter = isClosed)]
    #[inline]
    pub fn wasm_is_closed(&self) -> bool {
        self.is_closed()
    }
}
