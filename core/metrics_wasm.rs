use crate::metrics::{
    accuracy, avg_losing_trade, avg_trade, avg_win_loss_ratio, avg_winning_trade, expectancy,
    expectancy_score, f1, gross_loss_pct, gross_profit_pct, long_net_profit_pct,
    long_net_profit_ratio, net_profit_pct, omega_ratio, omega_ratio_from_returns, pnl, precision,
    profit_factor, recall, sharpe_ratio, sharpe_ratio_from_returns, short_net_profit_pct,
    sortino_ratio, sortino_ratio_from_returns, win_rate,
};
cfg_if::cfg_if! {
    if #[cfg(feature = "bindings_wasm")] {
        use wasm_bindgen::prelude::*;
    }
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "expectancy"))]
#[inline]
pub fn wasm_expectancy(pnl: &[f64]) -> f64 {
    expectancy(pnl)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "expectancyScore"))]
#[inline]
pub fn wasm_expectancy_score(expectancy: f64, opportunity_bars: f64) -> f64 {
    expectancy_score(expectancy, opportunity_bars)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "pnl"))]
#[inline]
pub fn wasm_pnl(qty: f64, entry_price: f64, current_price: f64) -> f64 {
    pnl(qty, entry_price, current_price)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "profitFactor"))]
#[inline]
pub fn wasm_profit_factor(gross_profit: f64, gross_loss: f64) -> f64 {
    profit_factor(gross_profit, gross_loss)
}

#[cfg_attr(
    feature = "bindings_wasm",
    wasm_bindgen(js_name = "longNetProfitRatio")
)]
#[inline]
pub fn wasm_long_net_profit_ratio(long_net_profit: f64, short_net_profit: f64) -> f64 {
    long_net_profit_ratio(long_net_profit, short_net_profit)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "winRate"))]
#[inline]
pub fn wasm_win_rate(profitable_trades: usize, total_trades: usize) -> f64 {
    win_rate(profitable_trades, total_trades)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "avgTrade"))]
#[inline]
pub fn wasm_avg_trade(net_profit: f64, closed_trades: usize) -> f64 {
    avg_trade(net_profit, closed_trades)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "avgWinningTrade"))]
#[inline]
pub fn wasm_avg_winning_trade(gross_profit: f64, winning_trades: usize) -> f64 {
    avg_winning_trade(gross_profit, winning_trades)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "avgLosingTrade"))]
#[inline]
pub fn wasm_avg_losing_trade(gross_loss: f64, losing_trades: usize) -> f64 {
    avg_losing_trade(gross_loss, losing_trades)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "avgWinLossRatio"))]
#[inline]
pub fn wasm_avg_win_loss_ratio(avg_winning_trade: f64, avg_losing_trade: f64) -> f64 {
    avg_win_loss_ratio(avg_winning_trade, avg_losing_trade)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "sharpeRatio"))]
#[inline]
pub fn wasm_sharpe_ratio(mean_returns: f64, std_returns: f64, risk_free_rate: f64) -> f64 {
    sharpe_ratio(mean_returns, std_returns, risk_free_rate)
}

#[cfg_attr(
    feature = "bindings_wasm",
    wasm_bindgen(js_name = "sharpeRatioFromReturns")
)]
#[inline]
pub fn wasm_sharpe_ratio_from_returns(returns: &[f64], risk_free_rate: f64) -> f64 {
    sharpe_ratio_from_returns(returns, risk_free_rate)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "sortinoRatio"))]
#[inline]
pub fn wasm_sortino_ratio(
    mean_returns: f64,
    negative_returns_stdev: f64,
    risk_free_rate: f64,
) -> f64 {
    sortino_ratio(mean_returns, negative_returns_stdev, risk_free_rate)
}

#[cfg_attr(
    feature = "bindings_wasm",
    wasm_bindgen(js_name = "sortinoRatioFromReturns")
)]
#[inline]
pub fn wasm_sortino_ratio_from_returns(returns: &[f64], risk_free_rate: f64) -> f64 {
    sortino_ratio_from_returns(returns, risk_free_rate)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "omegaRatio"))]
#[inline]
pub fn wasm_omega_ratio(
    positive_returns_sum: f64,
    negative_returns_sum: f64,
    risk_free_rate: f64,
) -> f64 {
    omega_ratio(positive_returns_sum, negative_returns_sum, risk_free_rate)
}

#[cfg_attr(
    feature = "bindings_wasm",
    wasm_bindgen(js_name = "omegaRatioFromReturns")
)]
#[inline]
pub fn wasm_omega_ratio_from_returns(returns: &[f64], risk_free_rate: f64) -> f64 {
    omega_ratio_from_returns(returns, risk_free_rate)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "netProfitPct"))]
#[inline]
pub fn wasm_net_profit_pct(net_profit: f64, initial_capital: f64) -> f64 {
    net_profit_pct(net_profit, initial_capital)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "grossProfitPct"))]
#[inline]
pub fn wasm_gross_profit_pct(gross_profit: f64, initial_capital: f64) -> f64 {
    gross_profit_pct(gross_profit, initial_capital)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "grossLossPct"))]
#[inline]
pub fn wasm_gross_loss_pct(gross_loss: f64, initial_capital: f64) -> f64 {
    gross_loss_pct(gross_loss, initial_capital)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "longNetProfitPct"))]
#[inline]
pub fn wasm_long_net_profit_pct(long_net_profit: f64, initial_capital: f64) -> f64 {
    long_net_profit_pct(long_net_profit, initial_capital)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "shortNetProfitPct"))]
#[inline]
pub fn wasm_short_net_profit_pct(short_net_profit: f64, initial_capital: f64) -> f64 {
    short_net_profit_pct(short_net_profit, initial_capital)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "accuracy"))]
#[inline]
pub fn wasm_accuracy(tp_count: f64, fp_count: f64, fn_count: f64, tn_count: f64) -> f64 {
    accuracy(tp_count, fp_count, fn_count, tn_count)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "precision"))]
#[inline]
pub fn wasm_precision(tp_count: f64, fp_count: f64) -> f64 {
    precision(tp_count, fp_count)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "recall"))]
#[inline]
pub fn wasm_recall(tp_count: f64, fn_count: f64) -> f64 {
    recall(tp_count, fn_count)
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "f1"))]
#[inline]
pub fn wasm_f1(precision: f64, recall: f64) -> f64 {
    f1(precision, recall)
}
