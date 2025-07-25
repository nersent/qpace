use crate::stats::{mean, returns, stdev, sum, var};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "sum")]
#[inline]
pub fn wasm_sum(values: &[f64]) -> f64 {
    return sum(values);
}

#[wasm_bindgen(js_name = "mean")]
#[inline]
pub fn wasm_mean(values: &[f64]) -> f64 {
    return mean(values);
}

#[wasm_bindgen(js_name = "variance")]
#[inline]
pub fn wasm_var(values: &[f64]) -> f64 {
    return var(values);
}

#[wasm_bindgen(js_name = "stdev")]
#[inline]
pub fn wasm_stdev(values: &[f64]) -> f64 {
    return stdev(values);
}

#[wasm_bindgen(js_name = "returns")]
#[inline]
pub fn wasm_returns(equity: &[f64], skip_first: Option<bool>) -> Vec<f64> {
    let skip_first = skip_first.unwrap_or(true);
    return returns(equity, skip_first);
}
