use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyfunction;

use crate::orderbook::{
    order_size, order_size_for_equity_pct, round_contracts, round_to_min_tick, validate_contracts,
};

#[gen_stub_pyfunction]
#[pyfunction(name = "round_to_min_tick")]
#[inline]
pub fn py_round_to_min_tick(value: f64, min_tick: f64) -> f64 {
    return round_to_min_tick(value, min_tick);
}

#[gen_stub_pyfunction]
#[pyfunction(name = "round_contracts")]
#[inline]
pub fn py_round_contracts(size: f64, min_qty: f64, price_scale: f64) -> f64 {
    return round_contracts(size, min_qty, price_scale);
}

#[gen_stub_pyfunction]
#[pyfunction(name = "validate_contracts")]
#[inline]
pub fn py_validate_contracts(size: f64, min_qty: f64) -> bool {
    return validate_contracts(size, min_qty);
}

#[gen_stub_pyfunction]
#[pyfunction(name = "order_size")]
#[inline]
pub fn py_order_size(
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

#[gen_stub_pyfunction]
#[pyfunction(name = "order_size_for_equity_pct")]
#[inline]
pub fn py_order_size_for_equity_pct(
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
