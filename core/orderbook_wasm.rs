use wasm_bindgen::prelude::*;

use crate::orderbook::{
    order_size, order_size_for_equity_pct, round_contracts, round_to_min_tick, validate_contracts,
};

#[wasm_bindgen(js_name = "roundToMinTick")]
#[inline]
pub fn wasm_round_to_min_tick(value: f64, min_tick: f64) -> f64 {
    return round_to_min_tick(value, min_tick);
}

#[wasm_bindgen(js_name = "roundContracts")]
#[inline]
pub fn wasm_round_contracts(size: f64, min_qty: f64, qty_scale: f64) -> f64 {
    return round_contracts(size, min_qty, qty_scale);
}

#[wasm_bindgen(js_name = "validateContracts")]
#[inline]
pub fn wasm_validate_contracts(size: f64, min_qty: f64) -> bool {
    return validate_contracts(size, min_qty);
}

#[wasm_bindgen(js_name = "orderSize")]
#[inline]
pub fn wasm_order_size(
    equity_pct: f64,
    equity: f64,
    exchange_rate: f64,
    instrument_price: f64,
    point_value: f64,
) -> f64 {
    return order_size(
        equity_pct,
        equity,
        exchange_rate,
        instrument_price,
        point_value,
    );
}

#[wasm_bindgen(js_name = "orderSizeForEquityPct")]
#[inline]
pub fn wasm_order_size_for_equity_pct(
    equity_pct: f64,
    equity: f64,
    current_position: f64,
    instrument_price: f64,
    point_value: f64,
    exchange_rate: f64,
) -> f64 {
    return order_size_for_equity_pct(
        equity_pct,
        equity,
        current_position,
        instrument_price,
        point_value,
        exchange_rate,
    );
}
