use crate::{
    backtest::{Backtest, BacktestConfig},
    ctx_node::{NodeCtx, NodeCtxSkip},
    signal_node::NodeSignal,
    trade_node::NodeTrade,
};
use napi::bindgen_prelude::*;
use napi::Result;
use napi_derive::napi;
use std::{cell::RefCell, rc::Rc};

#[napi(iterator)]
#[derive(Clone)]
pub struct NodeBacktest {
    inner: Rc<RefCell<Backtest>>,
    ctx: NodeCtx,
}

impl NodeBacktest {
    #[inline]
    pub fn inner(&self) -> &Rc<RefCell<Backtest>> {
        &self.inner
    }
}

#[napi]
impl NodeBacktest {
    #[napi(constructor)]
    #[inline]
    pub fn node_new(
        ctx: &NodeCtx,
        initial_capital: Option<f64>,
        process_orders_on_close: Option<bool>,
    ) -> Self {
        let initial_capital = initial_capital.unwrap_or(1000.0);
        let process_orders_on_close = process_orders_on_close.unwrap_or(false);
        let mut config = BacktestConfig::default();
        config.set_initial_capital(initial_capital);
        config.set_process_orders_on_close(process_orders_on_close);
        Self {
            inner: Rc::new(RefCell::new(Backtest::new(ctx.inner().clone(), config))),
            ctx: ctx.clone(),
        }
    }

    #[napi(getter = initialCapital)]
    #[inline]
    pub fn node_initial_capital(&self) -> f64 {
        self.inner.borrow().config().initial_capital()
    }

    #[napi(getter = processOrdersOnClose)]
    #[inline]
    pub fn node_process_orders_on_close(&self) -> bool {
        self.inner.borrow().config().process_orders_on_close()
    }

    #[napi(getter = ctx)]
    #[inline]
    pub fn node_ctx(&self) -> NodeCtx {
        self.ctx.clone()
    }

    #[napi(getter = equity)]
    #[inline]
    pub fn node_equity(&self) -> f64 {
        self.inner.borrow().equity()
    }

    #[napi(getter = equityList)]
    #[inline]
    pub fn node_equity_list(&self) -> Vec<f64> {
        self.inner.borrow().equity_list().to_vec()
    }

    #[napi(getter = netEquity)]
    #[inline]
    pub fn node_net_equity(&self) -> f64 {
        self.inner.borrow().net_equity()
    }

    #[napi(getter = netEquityList)]
    #[inline]
    pub fn node_net_equity_list(&self) -> Vec<f64> {
        self.inner.borrow().net_equity_list().to_vec()
    }

    #[napi(getter = pnlList)]
    #[inline]
    pub fn node_pnl_list(&self) -> Vec<f64> {
        self.inner.borrow().pnl_list()
    }

    #[napi(getter = openProfit)]
    #[inline]
    pub fn node_open_profit(&self) -> f64 {
        self.inner.borrow().open_profit()
    }

    #[napi(getter = netProfit)]
    #[inline]
    pub fn node_net_profit(&self) -> f64 {
        self.inner.borrow().net_profit()
    }

    #[napi(getter = netProfitPct)]
    #[inline]
    pub fn node_net_profit_pct(&self) -> f64 {
        self.inner.borrow().net_profit_pct()
    }

    #[napi(getter = grossProfit)]
    #[inline]
    pub fn node_gross_profit(&self) -> f64 {
        self.inner.borrow().gross_profit()
    }

    #[napi(getter = grossProfitPct)]
    #[inline]
    pub fn node_gross_profit_pct(&self) -> f64 {
        self.inner.borrow().gross_profit_pct()
    }

    #[napi(getter = grossLoss)]
    #[inline]
    pub fn node_gross_loss(&self) -> f64 {
        self.inner.borrow().gross_loss()
    }

    #[napi(getter = grossLossPct)]
    #[inline]
    pub fn node_gross_loss_pct(&self) -> f64 {
        self.inner.borrow().gross_loss_pct()
    }

    #[napi(getter = winRate)]
    #[inline]
    pub fn node_win_rate(&self) -> f64 {
        self.inner.borrow().win_rate()
    }

    #[napi(getter = profitFactor)]
    #[inline]
    pub fn node_profit_factor(&self) -> f64 {
        self.inner.borrow().profit_factor()
    }

    #[napi(getter = avgTrade)]
    #[inline]
    pub fn node_avg_trade(&self) -> f64 {
        self.inner.borrow().avg_trade()
    }

    #[napi(getter = avgWinningTrade)]
    #[inline]
    pub fn node_avg_winning_trade(&self) -> f64 {
        self.inner.borrow().avg_winning_trade()
    }

    #[napi(getter = avgLosingTrade)]
    #[inline]
    pub fn node_avg_losing_trade(&self) -> f64 {
        self.inner.borrow().avg_losing_trade()
    }

    #[napi(getter = avgWinLossRatio)]
    #[inline]
    pub fn node_avg_win_loss_ratio(&self) -> f64 {
        self.inner.borrow().avg_win_loss_ratio()
    }

    #[napi(getter = returnsList)]
    #[inline]
    pub fn node_returns_list(&self) -> Vec<f64> {
        self.inner.borrow().returns_list().to_vec()
    }

    #[napi(js_name = "sharpeRatio")]
    #[inline]
    pub fn node_sharpe_ratio(&self, rfr: f64) -> f64 {
        self.inner.borrow().sharpe_ratio(rfr)
    }

    #[napi(js_name = "sortinoRatio")]
    #[inline]
    pub fn node_sortino_ratio(&self, rfr: f64) -> f64 {
        self.inner.borrow().sortino_ratio(rfr)
    }

    #[napi(js_name = winningTradesCount)]
    #[inline]
    pub fn node_winning_trades_count(&self) -> usize {
        self.inner.borrow().winning_trades_count()
    }

    #[napi(getter = losingTradesCount)]
    #[inline]
    pub fn node_losing_trades_count(&self) -> usize {
        self.inner.borrow().losing_trades_count()
    }

    #[napi(getter = positionSize)]
    #[inline]
    pub fn node_position_size(&self) -> f64 {
        self.inner.borrow().position_size()
    }

    #[napi(getter = openTrades)]
    #[inline]
    pub fn node_open_trades(&self) -> Vec<NodeTrade> {
        self.inner
            .borrow()
            .open_trades()
            .iter()
            .map(|trade| (*trade).clone().into())
            .collect()
    }

    #[napi(getter = closedTrades)]
    #[inline]
    pub fn node_closed_trades(&self) -> Vec<NodeTrade> {
        self.inner
            .borrow()
            .closed_trades()
            .iter()
            .map(|trade| (*trade).clone().into())
            .collect()
    }

    #[napi(getter = trades)]
    #[inline]
    pub fn node_trades(&self) -> Vec<NodeTrade> {
        self.inner
            .borrow()
            .trades()
            .iter()
            .map(|trade| (*trade).clone().into())
            .collect()
    }

    #[napi(getter = openLongsCount)]
    #[inline]
    pub fn node_open_longs_count(&self) -> usize {
        self.inner.borrow().open_longs_count()
    }

    #[napi(getter = openShortsCount)]
    #[inline]
    pub fn node_open_shorts_count(&self) -> usize {
        self.inner.borrow().open_shorts_count()
    }

    #[napi(getter = closedLongsCount)]
    #[inline]
    pub fn node_closed_longs_count(&self) -> usize {
        self.inner.borrow().closed_longs_count()
    }

    #[napi(getter = closedShortsCount)]
    #[inline]
    pub fn node_closed_shorts_count(&self) -> usize {
        self.inner.borrow().closed_shorts_count()
    }

    #[napi(getter = firstEntryBarIndex)]
    #[inline]
    pub fn node_first_entry_bar_index(&self) -> Option<usize> {
        self.inner.borrow().first_entry_bar_index()
    }

    #[napi(getter = instrumentSize)]
    #[inline]
    pub fn node_instrument_size(&self) -> f64 {
        self.inner.borrow().instrument_size()
    }

    #[napi(js_name = "onBarOpen")]
    #[inline]
    pub fn node_on_bar_open(&mut self) {
        self.inner.borrow_mut().on_bar_open();
    }

    #[napi(js_name = "onBarClose")]
    #[inline]
    pub fn node_on_bar_close(&mut self) {
        self.inner.borrow_mut().on_bar_close();
    }

    #[napi(js_name = "signal")]
    #[inline]
    pub fn node_signal(&mut self, signal: &NodeSignal) {
        self.inner.borrow_mut().signal(signal.clone().into())
    }

    #[napi(js_name = "skip")]
    #[inline]
    pub fn node_skip(&mut self, skip: &NodeCtxSkip) {
        self.inner.borrow_mut().skip(skip.into());
    }

    #[napi(js_name = "length")]
    #[inline]
    pub fn node_length(&self) -> i32 {
        self.inner.borrow().len() as i32
    }

    #[napi(js_name = "next")]
    #[inline]
    pub fn node_next(&mut self) -> Option<i32> {
        let mut bt = self.inner.borrow_mut();
        let next = bt.ctx().borrow_mut().next();
        if next.is_none() {
            return None;
        }
        bt.on_bar_open();
        return Some(next.unwrap() as i32);
    }

    #[napi(js_name = "toPine")]
    #[inline]
    pub fn node_to_pine(&self) -> String {
        self.inner.borrow().to_pine()
    }

    #[napi(js_name = "display")]
    #[inline]
    pub fn node_print(&self) {
        self.inner.borrow().display(None);
    }
    // #[napi(js_name = "summary")]
    // #[inline]
    // pub fn node_summary(&self, risk_free_rate: Option<f64>) -> NodeBacktestSummary {
    //     let risk_free_rate = risk_free_rate.unwrap_or(0.0);
    //     self.inner
    //         .borrow()
    //         .summary(&BacktestSummaryConfig { risk_free_rate })
    //         .into()
    // }
}

impl Generator for NodeBacktest {
    type Yield = i32;
    type Next = ();
    type Return = ();

    fn next(&mut self, _v: Option<Self::Next>) -> Option<Self::Yield> {
        let mut bt = self.inner.borrow_mut();
        let next = bt.ctx().borrow_mut().next()?;
        bt.on_bar_open();
        bt.on_bar_close();
        return Some(next as i32);
    }
}

// #[napi]
// #[derive(Clone)]
// pub struct NodeBacktestSummary {
//     inner: BacktestSummary,
// }

// impl Into<NodeBacktestSummary> for BacktestSummary {
//     fn into(self) -> NodeBacktestSummary {
//         NodeBacktestSummary { inner: self }
//     }
// }

// impl Into<BacktestSummary> for NodeBacktestSummary {
//     fn into(self) -> BacktestSummary {
//         self.inner
//     }
// }

// #[napi]
// impl NodeBacktestSummary {
//     #[napi(js_name = "display")]
//     #[inline]
//     pub fn node_print(&self) {
//         self.inner.display(None);
//     }

//     #[napi(js_name = "toJSON")]
//     #[inline]
//     pub fn node_to_json(&self, env: Env) -> Result<Object> {
//         let mut js_obj = Object::new(&env)?;
//         js_obj.set("equity", self.inner.equity)?;
//         Ok(js_obj)
//     }
// }
