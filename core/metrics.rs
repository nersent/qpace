use crate::stats::{mean, stdev, sum};

#[inline]
pub fn expectancy(pnl: &[f64]) -> f64 {
    let mut losses: Vec<f64> = vec![];
    let mut wins: Vec<f64> = vec![];

    for _pnl in pnl {
        let _pnl = *_pnl;
        if _pnl > 0.0 {
            wins.push(_pnl);
        } else {
            losses.push(_pnl);
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

#[inline]
pub fn expectancy_score(expectancy: f64, opportunity_bars: f64) -> f64 {
    return expectancy * opportunity_bars;
}

#[inline]
pub fn pnl(qty: f64, entry_price: f64, current_price: f64) -> f64 {
    return (current_price - entry_price) * qty;
}

#[inline]
pub fn profit_factor(gross_profit: f64, gross_loss: f64) -> f64 {
    if gross_loss == 0.0 {
        return f64::NAN;
    }
    return gross_profit / gross_loss;
}

#[inline]
pub fn long_net_profit_ratio(long_net_profit: f64, short_net_profit: f64) -> f64 {
    if short_net_profit == 0.0 {
        return f64::NAN;
    }
    return long_net_profit / short_net_profit * -1.0;
}

#[inline]
pub fn win_rate(profitable_trades: usize, total_trades: usize) -> f64 {
    if total_trades == 0 {
        return f64::NAN;
    }
    return (profitable_trades as f64) / (total_trades as f64);
}

#[inline]
pub fn avg_trade(net_profit: f64, closed_trades: usize) -> f64 {
    if closed_trades == 0 {
        return f64::NAN;
    }
    return net_profit / (closed_trades as f64);
}

#[inline]
pub fn avg_winning_trade(gross_profit: f64, winning_trades: usize) -> f64 {
    if winning_trades == 0 {
        return f64::NAN;
    }
    return gross_profit / (winning_trades as f64);
}

#[inline]
pub fn avg_losing_trade(gross_loss: f64, losing_trades: usize) -> f64 {
    if losing_trades == 0 {
        return f64::NAN;
    }
    return gross_loss / (losing_trades as f64);
}

#[inline]
pub fn avg_win_loss_ratio(avg_winning_trade: f64, avg_losing_trade: f64) -> f64 {
    if avg_losing_trade == 0.0 {
        return f64::NAN;
    }
    return avg_winning_trade / avg_losing_trade;
}

#[inline]
pub fn sharpe_ratio(mean_returns: f64, std_returns: f64, risk_free_rate: f64) -> f64 {
    if std_returns == 0.0 {
        return f64::NAN;
    }
    return (mean_returns - risk_free_rate) / std_returns;
}

#[inline]
pub fn sharpe_ratio_from_returns(returns: &[f64], risk_free_rate: f64) -> f64 {
    return sharpe_ratio(mean(returns), stdev(returns), risk_free_rate);
}

#[inline]
pub fn sortino_ratio(mean_returns: f64, negative_returns_stdev: f64, risk_free_rate: f64) -> f64 {
    if negative_returns_stdev == 0.0 {
        return f64::NAN;
    }
    return (mean_returns - risk_free_rate) / negative_returns_stdev;
}

#[inline]
pub fn sortino_ratio_from_returns(returns: &[f64], risk_free_rate: f64) -> f64 {
    let negative_returns: Vec<f64> = returns.iter().filter(|&&x| x < 0.0).cloned().collect();
    return sortino_ratio(mean(returns), stdev(&negative_returns), risk_free_rate);
}

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

#[inline]
pub fn omega_ratio_from_returns(returns: &[f64], risk_free_rate: f64) -> f64 {
    let positive_returns: Vec<f64> = returns.iter().filter(|&&x| x > 0.0).cloned().collect();
    let negative_returns: Vec<f64> = returns.iter().filter(|&&x| x < 0.0).cloned().collect();
    return omega_ratio(
        sum(&positive_returns),
        sum(&negative_returns),
        risk_free_rate,
    );
}

#[inline]
pub fn net_profit_pct(net_profit: f64, initial_capital: f64) -> f64 {
    return net_profit / initial_capital;
}

#[inline]
pub fn gross_profit_pct(gross_profit: f64, initial_capital: f64) -> f64 {
    return gross_profit / initial_capital;
}

#[inline]
pub fn gross_loss_pct(gross_loss: f64, initial_capital: f64) -> f64 {
    return gross_loss / initial_capital;
}

#[inline]
pub fn long_net_profit_pct(long_net_profit: f64, initial_capital: f64) -> f64 {
    return long_net_profit / initial_capital;
}

#[inline]
pub fn short_net_profit_pct(short_net_profit: f64, initial_capital: f64) -> f64 {
    return short_net_profit / initial_capital;
}

#[inline]
pub fn accuracy(tp_count: f64, fp_count: f64, fn_count: f64, tn_count: f64) -> f64 {
    return (tp_count + tn_count) / (tp_count + fp_count + fn_count + tn_count);
}

#[inline]
pub fn precision(tp_count: f64, fp_count: f64) -> f64 {
    return tp_count / (tp_count + fp_count);
}

#[inline]
pub fn recall(tp_count: f64, fn_count: f64) -> f64 {
    return tp_count / (tp_count + fn_count);
}

#[inline]
pub fn f1(precision: f64, recall: f64) -> f64 {
    if precision + recall == 0.0 {
        return f64::NAN;
    }
    return 2.0 * (precision * recall) / (precision + recall);
}

// #[inline]
// pub fn max_drawdown_pct(max_dd: f64, net_equity_max: f64) -> f64 {
//     return max_dd / net_equity_max;
// }

// #[inline]
// pub fn max_run_up_pct(max_run_up: f64, bar_equity_max: f64) -> f64 {
//     return max_run_up / bar_equity_max;
// }
