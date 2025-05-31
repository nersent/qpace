use crate::stats::{mean, returns, stdev, sum, var};
use napi_derive::napi;

#[napi(js_name = "sum")]
#[inline]
pub fn node_sum(values: &[f64]) -> f64 {
    return sum(values);
}

#[napi(js_name = "mean")]
#[inline]
pub fn node_mean(values: &[f64]) -> f64 {
    return mean(values);
}

#[napi(js_name = "variance")]
#[inline]
pub fn node_var(values: &[f64]) -> f64 {
    return var(values);
}

#[napi(js_name = "stdev")]
#[inline]
pub fn node_stdev(values: &[f64]) -> f64 {
    return stdev(values);
}

#[napi(js_name = "returns")]
#[inline]
pub fn node_returns(equity: &[f64], skip_first: Option<bool>) -> Vec<f64> {
    let skip_first = skip_first.unwrap_or(true);
    return returns(equity, skip_first);
}
