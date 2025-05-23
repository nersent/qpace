use crate::metrics::{
    accuracy, avg_losing_trade, avg_trade, avg_win_loss_ratio, avg_winning_trade, expectancy,
    expectancy_score, f1, gross_loss_pct, gross_profit_pct, long_net_profit_pct,
    long_net_profit_ratio, net_profit_pct, omega_ratio, omega_ratio_from_returns, pnl, precision,
    profit_factor, recall, sharpe_ratio, sharpe_ratio_from_returns, short_net_profit_pct,
    sortino_ratio, sortino_ratio_from_returns, win_rate,
};
cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
    use pyo3::prelude::*;
    use pyo3_stub_gen::{derive::gen_stub_pyfunction};
}}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "expectancy"))]
#[inline]
pub fn py_expectancy(pnl: Vec<f64>) -> f64 {
    return expectancy(&pnl);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "expectancy_score"))]
#[inline]
pub fn py_expectancy_score(expectancy: f64, opportunity_bars: f64) -> f64 {
    return expectancy_score(expectancy, opportunity_bars);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "pnl"))]
#[inline]
pub fn py_pnl(qty: f64, entry_price: f64, current_price: f64) -> f64 {
    return pnl(qty, entry_price, current_price);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "profit_factor"))]
#[inline]
pub fn py_profit_factor(gross_profit: f64, gross_loss: f64) -> f64 {
    return profit_factor(gross_profit, gross_loss);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "long_net_profit_ratio"))]
#[inline]
pub fn py_long_net_profit_ratio(long_net_profit: f64, short_net_profit: f64) -> f64 {
    return long_net_profit_ratio(long_net_profit, short_net_profit);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "win_rate"))]
#[inline]
pub fn py_win_rate(profitable_trades: usize, total_trades: usize) -> f64 {
    return win_rate(profitable_trades, total_trades);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "avg_trade"))]
#[inline]
pub fn py_avg_trade(net_profit: f64, closed_trades: usize) -> f64 {
    return avg_trade(net_profit, closed_trades);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "avg_winning_trade"))]
#[inline]
pub fn py_avg_winning_trade(gross_profit: f64, winning_trades: usize) -> f64 {
    return avg_winning_trade(gross_profit, winning_trades);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "avg_losing_trade"))]
#[inline]
pub fn py_avg_losing_trade(gross_loss: f64, losing_trades: usize) -> f64 {
    return avg_losing_trade(gross_loss, losing_trades);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "avg_win_loss_ratio"))]
#[inline]
pub fn py_avg_win_loss_ratio(avg_winning_trade: f64, avg_losing_trade: f64) -> f64 {
    return avg_win_loss_ratio(avg_winning_trade, avg_losing_trade);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "sharpe_ratio"))]
#[inline]
pub fn py_sharpe_ratio(mean_returns: f64, std_returns: f64, risk_free_rate: f64) -> f64 {
    return sharpe_ratio(mean_returns, std_returns, risk_free_rate);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(
    feature = "bindings_py",
    pyfunction(name = "sharpe_ratio_from_returns")
)]
#[inline]
pub fn py_sharpe_ratio_from_returns(returns: Vec<f64>, risk_free_rate: f64) -> f64 {
    return sharpe_ratio_from_returns(&returns, risk_free_rate);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "sortino_ratio"))]
#[inline]
pub fn py_sortino_ratio(
    mean_returns: f64,
    negative_returns_stdev: f64,
    risk_free_rate: f64,
) -> f64 {
    return sortino_ratio(mean_returns, negative_returns_stdev, risk_free_rate);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(
    feature = "bindings_py",
    pyfunction(name = "sortino_ratio_from_returns")
)]
#[inline]
pub fn py_sortino_ratio_from_returns(returns: Vec<f64>, risk_free_rate: f64) -> f64 {
    return sortino_ratio_from_returns(&returns, risk_free_rate);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "omega_ratio"))]
#[inline]
pub fn py_omega_ratio(
    positive_returns_sum: f64,
    negative_returns_sum: f64,
    risk_free_rate: f64,
) -> f64 {
    return omega_ratio(positive_returns_sum, negative_returns_sum, risk_free_rate);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "omega_ratio_from_returns"))]
#[inline]
pub fn py_omega_ratio_from_returns(returns: Vec<f64>, risk_free_rate: f64) -> f64 {
    return omega_ratio_from_returns(&returns, risk_free_rate);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "net_profit_pct"))]
#[inline]
pub fn py_net_profit_pct(net_profit: f64, initial_capital: f64) -> f64 {
    return net_profit_pct(net_profit, initial_capital);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "gross_profit_pct"))]
#[inline]
pub fn py_gross_profit_pct(gross_profit: f64, initial_capital: f64) -> f64 {
    return gross_profit_pct(gross_profit, initial_capital);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "gross_loss_pct"))]
#[inline]
pub fn py_gross_loss_pct(gross_loss: f64, initial_capital: f64) -> f64 {
    return gross_loss_pct(gross_loss, initial_capital);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "long_net_profit_pct"))]
#[inline]
pub fn py_long_net_profit_pct(long_net_profit: f64, initial_capital: f64) -> f64 {
    return long_net_profit_pct(long_net_profit, initial_capital);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "short_net_profit_pct"))]
#[inline]
pub fn py_short_net_profit_pct(short_net_profit: f64, initial_capital: f64) -> f64 {
    return short_net_profit_pct(short_net_profit, initial_capital);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "accuracy"))]
#[inline]
pub fn py_accuracy(tp_count: f64, fp_count: f64, fn_count: f64, tn_count: f64) -> f64 {
    return accuracy(tp_count, fp_count, fn_count, tn_count);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "precision"))]
#[inline]
pub fn py_precision(tp_count: f64, fp_count: f64) -> f64 {
    return precision(tp_count, fp_count);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "recall"))]
#[inline]
pub fn py_recall(tp_count: f64, fn_count: f64) -> f64 {
    return recall(tp_count, fn_count);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "f1"))]
#[inline]
pub fn py_f1(precision: f64, recall: f64) -> f64 {
    return f1(precision, recall);
}
