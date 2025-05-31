use crate::metrics::{
    accuracy, avg_losing_trade, avg_trade, avg_win_loss_ratio, avg_winning_trade, expectancy,
    expectancy_score, f1, gross_loss_pct, gross_profit_pct, long_net_profit_pct,
    long_net_profit_ratio, net_profit_pct, omega_ratio, omega_ratio_from_returns, pnl, precision,
    profit_factor, recall, sharpe_ratio, sharpe_ratio_from_returns, short_net_profit_pct,
    sortino_ratio, sortino_ratio_from_returns, win_rate,
};
use napi_derive::napi;

#[napi(js_name = "expectancy")]
#[inline]
pub fn node_expectancy(pnl: &[f64]) -> f64 {
    expectancy(pnl)
}

#[napi(js_name = "expectancyScore")]
#[inline]
pub fn node_expectancy_score(expectancy: f64, opportunity_bars: f64) -> f64 {
    expectancy_score(expectancy, opportunity_bars)
}

#[napi(js_name = "pnl")]
#[inline]
pub fn node_pnl(qty: f64, entry_price: f64, current_price: f64) -> f64 {
    pnl(qty, entry_price, current_price)
}

#[napi(js_name = "profitFactor")]
#[inline]
pub fn node_profit_factor(gross_profit: f64, gross_loss: f64) -> f64 {
    profit_factor(gross_profit, gross_loss)
}

#[napi(js_name = "longNetProfitRatio")]
#[inline]
pub fn node_long_net_profit_ratio(long_net_profit: f64, short_net_profit: f64) -> f64 {
    long_net_profit_ratio(long_net_profit, short_net_profit)
}

#[napi(js_name = "winRate")]
#[inline]
pub fn node_win_rate(profitable_trades: i32, total_trades: i32) -> f64 {
    win_rate(profitable_trades as usize, total_trades as usize)
}

#[napi(js_name = "avgTrade")]
#[inline]
pub fn node_avg_trade(net_profit: f64, closed_trades: i32) -> f64 {
    avg_trade(net_profit, closed_trades as usize)
}

#[napi(js_name = "avgWinningTrade")]
#[inline]
pub fn node_avg_winning_trade(gross_profit: f64, winning_trades: i32) -> f64 {
    avg_winning_trade(gross_profit, winning_trades as usize)
}

#[napi(js_name = "avgLosingTrade")]
#[inline]
pub fn node_avg_losing_trade(gross_loss: f64, losing_trades: i32) -> f64 {
    avg_losing_trade(gross_loss, losing_trades as usize)
}

#[napi(js_name = "avgWinLossRatio")]
#[inline]
pub fn node_avg_win_loss_ratio(avg_winning_trade: f64, avg_losing_trade: f64) -> f64 {
    avg_win_loss_ratio(avg_winning_trade, avg_losing_trade)
}

#[napi(js_name = "sharpeRatio")]
#[inline]
pub fn node_sharpe_ratio(mean_returns: f64, std_returns: f64, risk_free_rate: f64) -> f64 {
    sharpe_ratio(mean_returns, std_returns, risk_free_rate)
}

#[napi(js_name = "sharpeRatioFromReturns")]
#[inline]
pub fn node_sharpe_ratio_from_returns(returns: &[f64], risk_free_rate: f64) -> f64 {
    sharpe_ratio_from_returns(returns, risk_free_rate)
}

#[napi(js_name = "sortinoRatio")]
#[inline]
pub fn node_sortino_ratio(
    mean_returns: f64,
    negative_returns_stdev: f64,
    risk_free_rate: f64,
) -> f64 {
    sortino_ratio(mean_returns, negative_returns_stdev, risk_free_rate)
}

#[napi(js_name = "sortinoRatioFromReturns")]
#[inline]
pub fn node_sortino_ratio_from_returns(returns: &[f64], risk_free_rate: f64) -> f64 {
    sortino_ratio_from_returns(returns, risk_free_rate)
}

#[napi(js_name = "omegaRatio")]
#[inline]
pub fn node_omega_ratio(
    positive_returns_sum: f64,
    negative_returns_sum: f64,
    risk_free_rate: f64,
) -> f64 {
    omega_ratio(positive_returns_sum, negative_returns_sum, risk_free_rate)
}

#[napi(js_name = "omegaRatioFromReturns")]
#[inline]
pub fn node_omega_ratio_from_returns(returns: &[f64], risk_free_rate: f64) -> f64 {
    omega_ratio_from_returns(returns, risk_free_rate)
}

#[napi(js_name = "netProfitPct")]
#[inline]
pub fn node_net_profit_pct(net_profit: f64, initial_capital: f64) -> f64 {
    net_profit_pct(net_profit, initial_capital)
}

#[napi(js_name = "grossProfitPct")]
#[inline]
pub fn node_gross_profit_pct(gross_profit: f64, initial_capital: f64) -> f64 {
    gross_profit_pct(gross_profit, initial_capital)
}

#[napi(js_name = "grossLossPct")]
#[inline]
pub fn node_gross_loss_pct(gross_loss: f64, initial_capital: f64) -> f64 {
    gross_loss_pct(gross_loss, initial_capital)
}

#[napi(js_name = "longNetProfitPct")]
#[inline]
pub fn node_long_net_profit_pct(long_net_profit: f64, initial_capital: f64) -> f64 {
    long_net_profit_pct(long_net_profit, initial_capital)
}

#[napi(js_name = "shortNetProfitPct")]
#[inline]
pub fn node_short_net_profit_pct(short_net_profit: f64, initial_capital: f64) -> f64 {
    short_net_profit_pct(short_net_profit, initial_capital)
}

#[napi(js_name = "accuracy")]
#[inline]
pub fn node_accuracy(tp_count: f64, fp_count: f64, fn_count: f64, tn_count: f64) -> f64 {
    accuracy(tp_count, fp_count, fn_count, tn_count)
}

#[napi(js_name = "precision")]
#[inline]
pub fn node_precision(tp_count: f64, fp_count: f64) -> f64 {
    precision(tp_count, fp_count)
}

#[napi(js_name = "recall")]
#[inline]
pub fn node_recall(tp_count: f64, fn_count: f64) -> f64 {
    recall(tp_count, fn_count)
}

#[napi(js_name = "f1")]
#[inline]
pub fn node_f1(precision: f64, recall: f64) -> f64 {
    f1(precision, recall)
}
