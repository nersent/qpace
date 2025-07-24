use napi_derive::napi;

use crate::orderbook::{
    order_size, order_size_for_equity_pct, round_contracts, round_to_min_tick, validate_contracts,
};

#[napi(js_name = "roundToMinTick")]
#[inline]
pub fn node_round_to_min_tick(value: f64, min_tick: f64) -> f64 {
    return round_to_min_tick(value, min_tick);
}

#[napi(js_name = "roundContracts")]
#[inline]
pub fn node_round_contracts(size: f64, min_qty: f64, qty_scale: f64) -> f64 {
    return round_contracts(size, min_qty, qty_scale);
}

#[napi(js_name = "validateContracts")]
#[inline]
pub fn node_validate_contracts(size: f64, min_qty: f64) -> bool {
    return validate_contracts(size, min_qty);
}

#[napi(js_name = "orderSize")]
#[inline]
pub fn node_order_size(
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

#[napi(js_name = "orderSizeForEquityPct")]
#[inline]
pub fn node_order_size_for_equity_pct(
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
