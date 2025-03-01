use chrono::{DateTime, Utc};
use itertools::izip;

use crate::{ohlcv::OhlcvBar, rs_utils::get_oldest_possible_datetime};

cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
    use pyo3::prelude::*;
    use pyo3_stub_gen::{derive::gen_stub_pyfunction};
}}
cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
    use wasm_bindgen::prelude::*;
}}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen)]
#[inline]
pub fn hl2(high: f64, low: f64) -> f64 {
    return (high + low) / 2.0;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen)]
#[inline]
pub fn hlc3(high: f64, low: f64, close: f64) -> f64 {
    return (high + low + close) / 3.0;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen)]
#[inline]
pub fn hlcc4(high: f64, low: f64, close: f64) -> f64 {
    return (high + low + close + close) / 4.0;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = roundContracts))]
#[doc = "Rounds `size` to the nearest multiple of the minimum order quantity."]
#[inline]
pub fn round_contracts(size: f64, min_qty: f64) -> f64 {
    if min_qty.is_nan() {
        return size;
    }
    // 0.000001
    let round_val = 1000000.0;
    return ((size * round_val) + f64::EPSILON).round() / round_val;
    // return (size * round_val).floor() / round_val;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = validateContracts))]
#[doc = "Checks if `size` is a valid order quantity by comparing it to the minimum order quantity."]
#[inline]
pub fn validate_contracts(size: f64, min_qty: f64) -> bool {
    return min_qty.is_nan() || !size.is_nan() && size.abs() >= min_qty;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = roundToMinTick))]
#[inline]
pub fn round_to_min_tick(value: f64, min_tick: f64) -> f64 {
    if value.is_nan() {
        return 0.0;
    }
    if min_tick.is_nan() {
        return value;
    }
    return (value / min_tick).round() * min_tick;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = orderSize))]
#[inline]
#[doc = "
the calculated order size, rounded down to the smallest trade quantity. For stocks, futures, CFDs, and forex that minimum quantity is 1. For Bitcoin (BTCUSD) it's 0.000001 and Ethereum (ETHUSD) uses 0.0001.

# Parameters

* `equity_pct` - A `f64` representing the percentage of the current strategy equity to invest in each order. This percentage is derived from either the `default_qty_value` setting or the manual 'Order size' option within the strategy's settings window.

* `equity` - A `f64` representing the strategy's current equity. This is the sum of the initial capital, closed net profit, and open position profit. Note that this includes unrealized profits/losses, which may affect the calculated order size if the open position's result changes significantly when it's closed.

* `exchange_rate` - A `f64` used for currency conversion, if necessary. If the strategy currency and the instrument currency are the same, this should be 1. Otherwise, provide the conversion rate between the two currencies.

* `instrument_price` - A `f64` representing the last available price at the time the order is generated. This is typically the close price of the bar on which the order is generated, unless using options like 'Recalculate After Order Filled' or 'Recalculate On Every Tick', which may use a different price within the bar.

* `point_value` - A `f64` that denotes the currency amount of one full point of price movement for the instrument. For example, it is 1 for stocks and 20 for the E-mini Nasdaq 100 futures.

# Returns

Returns a `f64` representing the calculated order size, rounded down to the smallest trade quantity based on the instrument type.
"]
pub fn order_size(
    equity_pct: f64,
    equity: f64,
    exchange_rate: f64,
    instrument_price: f64,
    point_value: f64,
) -> f64 {
    return (equity_pct * equity * exchange_rate) / (instrument_price * point_value);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = orderSizeForEquityPct))]
#[inline]
pub fn order_size_for_equity_pct(
    equity_pct: f64,
    equity: f64,
    current_position: f64,
    instrument_price: f64,
    point_value: f64,
    exchange_rate: f64,
) -> f64 {
    let equity_order_size = order_size(
        equity_pct.abs(),
        equity,
        exchange_rate,
        instrument_price,
        point_value,
    );

    let sign = equity_pct.signum();

    let order_size = equity_order_size * sign - current_position;

    return order_size;
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen)]
#[inline]
pub fn sum(values: &[f64]) -> f64 {
    return values.iter().sum();
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen)]
#[inline]
pub fn mean(values: &[f64]) -> f64 {
    return sum(values) / (values.len() as f64);
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = varFromMean))]
#[inline]
pub fn var_from_mean(values: &[f64], mean: f64) -> f64 {
    return values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = variance))]
#[inline]
pub fn var(values: &[f64]) -> f64 {
    return var_from_mean(values, mean(values));
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = stdevFromVar))]
#[inline]
pub fn stdev_from_var(var: f64) -> f64 {
    return var.sqrt();
}

#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = stdev))]
#[inline]
pub fn stdev(values: &[f64]) -> f64 {
    return stdev_from_var(var(values));
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = pctChange))]
#[inline]
pub fn pct_change(current: f64, previous: f64) -> f64 {
    if previous == 0.0 || current == 0.0 {
        return 0.0;
    }
    return (current / previous) - 1.0;
}

#[inline]
pub fn returns(equity: &[f64], pad: bool) -> Vec<f64> {
    let mut returns: Vec<f64> = equity
        .windows(2)
        .map(|w| {
            let previous = w[0];
            let current = w[1];
            (current - previous) / previous
        })
        .collect();
    if pad {
        returns.insert(0, f64::NAN);
    }
    return returns;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "returns", signature = (equity, pad=false)))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = returns))]
#[inline]
#[doc = "
Calculates returns from equity (% change)
Returns without first item, because it would be NAN.
Example: [1.0, 2.0] -> [2.0] // 200%
"]
pub fn py_js_returns(equity: Vec<f64>, pad: bool) -> Vec<f64> {
    return returns(&equity, pad);
}

#[inline]
pub fn expectancy(pnl_series: &[f64]) -> f64 {
    let mut losses: Vec<f64> = vec![];
    let mut wins: Vec<f64> = vec![];

    for pnl in pnl_series {
        if pnl > &0.0 {
            wins.push(*pnl);
        } else {
            losses.push(*pnl);
        }
    }

    if wins.len() > 1 {
        wins.sort_by(|a, b| b.partial_cmp(a).unwrap());
        wins.pop();
    }

    let avg_winning_trade = wins.iter().sum::<f64>() / wins.len() as f64;
    let avg_losing_trade = losses.iter().sum::<f64>() / losses.len() as f64;

    let winning_trades = wins.len();
    let losing_trades = losses.len();

    let nst = winning_trades + losing_trades;
    let aw = avg_winning_trade;
    let al = avg_losing_trade;

    // # probability of winning
    // # PW = <wins> ⁄ NST
    // #   where <wins> is total wins excluding maximum win
    // if nst == 0:
    //     pw = 0.0
    // else:
    //     pw = winning_trades / nst
    // # probability of losing
    // # PL = <non-scratch losses> ⁄ NST
    // if nst == 0:
    //     pl = 1.0
    // else:
    //     pl = losing_trades / nst

    // # NST × 365 ⁄ studydays   (opportunities to trade in a year)
    // opportunity = nst * (365 / studydays)

    // expectancy = (aw * pw + al * pl) / abs(al)
    // expectancy_score = expectancy * opportunity
    let pw = if nst == 0 {
        0.0
    } else {
        winning_trades as f64 / nst as f64
    };

    let pl = if nst == 0 {
        1.0
    } else {
        losing_trades as f64 / nst as f64
    };

    let expectancy = (aw * pw + al * pl) / al.abs();
    return expectancy;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = expectancyScore))]
#[inline]
pub fn expectancy_score(expectancy: f64, opportunity_bars: f64) -> f64 {
    return expectancy * opportunity_bars;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction(name = "expectancy"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = expectancy))]
#[inline]
pub fn py_expectancy(pnl_series: Vec<f64>) -> f64 {
    return expectancy(&pnl_series);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = pnl))]
#[inline]
pub fn pnl(qty: f64, entry_price: f64, current_price: f64) -> f64 {
    return (current_price - entry_price) * qty;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = profitFactor))]
#[inline]
pub fn profit_factor(gross_profit: f64, gross_loss: f64) -> f64 {
    if gross_loss == 0.0 {
        return f64::NAN;
    }
    return gross_profit / gross_loss;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = longNetProfitRatio))]
#[inline]
pub fn long_net_profit_ratio(long_net_profit: f64, short_net_profit: f64) -> f64 {
    if short_net_profit == 0.0 {
        return f64::NAN;
    }
    return long_net_profit / short_net_profit * -1.0;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = winRate))]
#[inline]
pub fn win_rate(profitable_trades: usize, total_trades: usize) -> f64 {
    if total_trades == 0 {
        return f64::NAN;
    }
    return (profitable_trades as f64) / (total_trades as f64);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = avgTrade))]
#[inline]
pub fn avg_trade(net_profit: f64, closed_trades: usize) -> f64 {
    if closed_trades == 0 {
        return f64::NAN;
    }
    return net_profit / (closed_trades as f64);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = avgWinningTrade))]
#[inline]
pub fn avg_winning_trade(gross_profit: f64, winning_trades: usize) -> f64 {
    if winning_trades == 0 {
        return f64::NAN;
    }
    return gross_profit / (winning_trades as f64);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = avgLosingTrade))]
#[inline]
pub fn avg_losing_trade(gross_loss: f64, losing_trades: usize) -> f64 {
    if losing_trades == 0 {
        return f64::NAN;
    }
    return gross_loss / (losing_trades as f64);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = avgWinLossRatio))]
#[inline]
pub fn avg_win_loss_ratio(avg_winning_trade: f64, avg_losing_trade: f64) -> f64 {
    if avg_losing_trade == 0.0 {
        return f64::NAN;
    }
    return avg_winning_trade / avg_losing_trade;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = omegaRatio))]
#[inline]
pub fn omega_ratio(
    positive_returns_sum: f64,
    negative_returns_sum: f64,
    risk_free_rate: f64,
) -> f64 {
    if negative_returns_sum == 0.0 {
        return f64::NAN;
    }
    return (positive_returns_sum - risk_free_rate) / negative_returns_sum;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = sharpeRatio))]
#[inline]
pub fn sharpe_ratio(mean_returns: f64, std_returns: f64, risk_free_rate: f64) -> f64 {
    if std_returns == 0.0 {
        return f64::NAN;
    }
    return (mean_returns - risk_free_rate) / std_returns;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = sortinoRatio))]
#[inline]
pub fn sortino_ratio(mean_returns: f64, negative_returns_stdev: f64, risk_free_rate: f64) -> f64 {
    if negative_returns_stdev == 0.0 {
        return f64::NAN;
    }
    return (mean_returns - risk_free_rate) / negative_returns_stdev;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = netProfitPct))]
#[inline]
pub fn net_profit_pct(net_profit: f64, initial_capital: f64) -> f64 {
    return 100.0 * net_profit / initial_capital;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = grossProfitPct))]
#[inline]
pub fn gross_profit_pct(gross_profit: f64, initial_capital: f64) -> f64 {
    return 100.0 * gross_profit / initial_capital;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = grossLossPct))]
#[inline]
pub fn gross_loss_pct(gross_loss: f64, initial_capital: f64) -> f64 {
    return 100.0 * gross_loss / initial_capital;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = longNetProfitPct))]
#[inline]
pub fn long_net_profit_pct(long_net_profit: f64, initial_capital: f64) -> f64 {
    return long_net_profit / initial_capital;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = shortNetProfitPct))]
#[inline]
pub fn short_net_profit_pct(short_net_profit: f64, initial_capital: f64) -> f64 {
    return short_net_profit / initial_capital;
}

#[inline]
#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = maxDrawdownPct))]
pub fn max_drawdown_pct(max_dd: f64, net_equity_max: f64) -> f64 {
    return max_dd / net_equity_max;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = maxRunUpPct))]
#[inline]
pub fn max_run_up_pct(max_run_up: f64, bar_equity_max: f64) -> f64 {
    return max_run_up / bar_equity_max;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = kellyCriterion))]
#[doc = "https://python.plainenglish.io/the-kelly-criterion-maximizing-returns-through-optimal-betting-32781a768ffb"]
#[inline]
pub fn kelly_criterion(win_prob: f64, profit_factor: f64) -> f64 {
    let q = 1.0 - win_prob;
    return (profit_factor * win_prob - q) / profit_factor;
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen)]
#[doc = "https://pmc.ncbi.nlm.nih.gov/articles/PMC4614595/"]
#[inline]
pub fn accuracy(tp_count: f64, fp_count: f64, fn_count: f64, tn_count: f64) -> f64 {
    return (tp_count + tn_count) / (tp_count + fp_count + fn_count + tn_count);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen)]
#[doc = "https://pmc.ncbi.nlm.nih.gov/articles/PMC4614595/"]
#[inline]
pub fn sensitivity(tp_count: f64, fp_count: f64) -> f64 {
    return tp_count / (tp_count + fp_count);
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction(module = "utils"))]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen)]
#[doc = "https://pmc.ncbi.nlm.nih.gov/articles/PMC4614595/"]
#[inline]
pub fn specificity(fp_count: f64, tn_count: f64) -> f64 {
    return tn_count / (tn_count + fp_count);
}
