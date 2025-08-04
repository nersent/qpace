use chrono::Duration;

use crate::timeframe::Timeframe;

#[inline]
pub fn sum(values: &[f64]) -> f64 {
    return values.iter().sum();
}

#[inline]
pub fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }
    return sum(values) / values.len() as f64;
}

#[inline]
pub fn var(values: &[f64]) -> f64 {
    let mean = mean(values);
    return values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
}

#[inline]
pub fn stdev(values: &[f64]) -> f64 {
    return var(values).sqrt();
}

#[inline]
#[doc = "
Calculates returns from equity (% change)
Returns without first item, because it would be NAN.
Example: [1.0, 2.0] -> [2.0] // 200%
"]
pub fn returns(equity: &[f64], skip_first: bool) -> Vec<f64> {
    let mut returns: Vec<f64> = equity
        .windows(2)
        .map(|w| {
            let previous = w[0];
            let current = w[1];
            (current - previous) / previous
        })
        .collect();
    if !skip_first {
        returns.insert(0, f64::NAN);
    }
    return returns;
}
