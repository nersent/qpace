cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
  use crate::ctx_js::{JsCtx};
  use wasm_bindgen::convert::TryFromJsValue;
  use js_sys::{Object, Reflect};
}}
use crate::{
    backtest::{Backtest, BacktestConfig},
    ctx::Ctx,
    ohlcv::{Ohlcv, OhlcvBar, OhlcvReader, OhlcvWriter},
    orderbook::{OrderBookError, OrderConfig},
    rs_utils::get_oldest_possible_datetime,
    signal::Signal,
    sym::Sym,
    trade::Trade,
};
use chrono::{DateTime, Utc};
use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = table)]
    pub fn js_console_table(obj: &JsValue);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn js_console_log(s: &str);
}

#[cfg(feature = "bindings_wasm")]
impl JsBacktest {
    #[inline]
    pub fn new(ctx: PyCtx, config: BacktestConfig) -> Self {
        Self {
            bt: Rc::new(RefCell::new(Backtest::new(js_ctx.clone().into(), config))),
            js_ctx,
        }
    }
}
#[cfg(feature = "bindings_wasm")]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "Backtest"))]
pub struct JsBacktest {
    js_ctx: JsCtx,
    bt: Rc<RefCell<Backtest>>,
}

#[cfg(feature = "bindings_wasm")]
impl Into<Rc<RefCell<Backtest>>> for JsBacktest {
    fn into(self) -> Rc<RefCell<Backtest>> {
        self.bt
    }
}

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(js_class=Backtest)]
impl JsBacktest {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn js_new(
        js_ctx: JsCtx,
        initial_capital: Option<f64>,
        process_orders_on_close: Option<bool>,
    ) -> Self {
        let mut config = BacktestConfig::default();
        if let Some(initial_capital) = initial_capital {
            config.set_initial_capital(initial_capital);
        }
        if let Some(process_orders_on_close) = process_orders_on_close {
            config.set_process_orders_on_close(process_orders_on_close);
        }
        Self::new(ctx, config)
    }

    #[wasm_bindgen(getter = initialCapital)]
    #[inline]
    pub fn js_initial_capital(&self) -> f64 {
        self.bt.borrow().config().initial_capital()
    }

    #[wasm_bindgen(getter = processOrdersOnClose)]
    #[inline]
    pub fn js_process_orders_on_close(&self) -> bool {
        self.bt.borrow().config().process_orders_on_close()
    }

    #[wasm_bindgen(getter = ctx)]
    #[inline]
    pub fn js_ctx(&self) -> JsCtx {
        self.js_ctx.clone()
    }

    #[wasm_bindgen(js_name = "next")]
    #[inline]
    pub fn js_next(&mut self) -> Option<usize> {
        let mut bt = self.bt.borrow_mut();
        let next = bt.ctx().borrow_mut().next();
        bt.on_bar_open();
        return next;
    }

    #[wasm_bindgen(js_name = "onBarOpen")]
    #[inline]
    pub fn js_on_bar_open(&mut self) {
        self.bt.borrow_mut().on_bar_open();
    }

    #[wasm_bindgen(js_name = "onBarClose")]
    #[inline]
    pub fn js_on_bar_close(&mut self) {
        self.bt.borrow_mut().on_bar_close();
    }

    #[wasm_bindgen(getter = equity)]
    #[inline]
    #[doc = "`initial capital + net profit + open profit`"]
    pub fn js_equity(&self) -> f64 {
        self.bt.borrow().equity()
    }

    #[wasm_bindgen(getter = netEquity)]
    #[inline]
    #[doc = "`initial_capital + net_profit`"]
    pub fn js_net_equity(&self) -> f64 {
        self.bt.borrow().net_equity()
    }

    #[wasm_bindgen(getter = equityList)]
    #[inline]
    pub fn js_equity_series(&self) -> Vec<f64> {
        self.bt.borrow().equity_series().to_vec()
    }

    #[wasm_bindgen(getter = netEquityList)]
    #[inline]
    pub fn js_net_equity_series(&self) -> Vec<f64> {
        self.bt.borrow().net_equity_series().to_vec()
    }

    #[wasm_bindgen(getter = equityReturns)]
    #[inline]
    pub fn js_equity_returns(&self) -> Vec<f64> {
        self.bt.borrow().equity_returns()
    }

    #[wasm_bindgen(getter = netEquityReturns)]
    #[inline]
    pub fn js_net_equity_returns(&self) -> Vec<f64> {
        self.bt.borrow().net_equity_returns()
    }

    #[wasm_bindgen(getter = pnlList)]
    #[inline]
    pub fn js_pnl_series(&self) -> Vec<f64> {
        self.bt.borrow().pnl_series()
    }

    #[wasm_bindgen(getter = openProfit)]
    #[inline]
    pub fn js_open_profit(&self) -> f64 {
        self.bt.borrow().open_profit()
    }

    #[wasm_bindgen(getter = netProfit)]
    #[inline]
    #[doc = "Overall profit or loss."]
    pub fn js_net_profit(&self) -> f64 {
        self.bt.borrow().net_profit()
    }

    #[wasm_bindgen(getter = grossProfit)]
    #[inline]
    #[doc = "Total value of all completed winning trades."]
    pub fn js_gross_profit(&self) -> f64 {
        self.bt.borrow().gross_profit()
    }

    #[wasm_bindgen(getter = grossLoss)]
    #[inline]
    #[doc = "Total value of all completed losing trades."]
    pub fn js_gross_loss(&self) -> f64 {
        self.bt.borrow().gross_loss()
    }

    #[wasm_bindgen(getter = winningTrades)]
    #[inline]
    #[doc = "Total number of winning trades."]
    pub fn js_winning_trades(&self) -> usize {
        self.bt.borrow().winning_trades()
    }

    #[wasm_bindgen(getter = losingTrades)]
    #[inline]
    #[doc = "Total number of losing trades."]
    pub fn js_losing_trades(&self) -> usize {
        self.bt.borrow().losing_trades()
    }

    #[wasm_bindgen(getter = positionSize)]
    #[inline]
    #[doc = "Direction and size of the current market position. If the value is > 0, the market position is long. If the value is < 0, the market position is short. The absolute value is the number of contracts/shares/lots/units in trade (position size)."]
    pub fn js_position_size(&self) -> f64 {
        self.bt.borrow().position_size()
    }

    #[wasm_bindgen(getter = openTrades)]
    #[inline]
    pub fn js_open_trades(&self) -> Vec<Trade> {
        self.bt.borrow().open_trades().to_vec()
    }

    #[wasm_bindgen(getter = closedTrades)]
    #[inline]
    pub fn js_closed_trades(&self) -> Vec<Trade> {
        self.bt.borrow().closed_trades().to_vec()
    }

    #[wasm_bindgen(getter = trades)]
    #[inline]
    pub fn js_trades(&self) -> Vec<Trade> {
        self.bt.borrow().trades().into_iter().cloned().collect()
    }

    #[wasm_bindgen(getter = openLongs)]
    #[inline]
    pub fn js_open_longs(&self) -> usize {
        self.bt.borrow().open_longs()
    }

    #[wasm_bindgen(getter = openShorts)]
    #[inline]
    pub fn js_open_shorts(&self) -> usize {
        self.bt.borrow().open_shorts()
    }

    #[wasm_bindgen(getter = closedLongs)]
    #[inline]
    pub fn js_closed_longs(&self) -> usize {
        self.bt.borrow().closed_longs()
    }

    #[wasm_bindgen(getter = closedShorts)]
    #[inline]
    pub fn js_closed_shorts(&self) -> usize {
        self.bt.borrow().closed_shorts()
    }

    #[wasm_bindgen(getter = totalTongs)]
    #[inline]
    pub fn js_total_longs(&self) -> usize {
        self.bt.borrow().total_longs()
    }

    #[wasm_bindgen(getter = totalShorts)]
    #[inline]
    pub fn js_total_shorts(&self) -> usize {
        self.bt.borrow().total_shorts()
    }

    #[wasm_bindgen(getter = totalTrades)]
    #[inline]
    pub fn js_total_trades(&self) -> usize {
        self.bt.borrow().total_trades()
    }

    #[wasm_bindgen(getter = instrumentPrice)]
    #[inline]
    pub fn js_instrument_size(&self) -> f64 {
        self.bt.borrow().instrument_size()
    }

    #[wasm_bindgen(getter = winRate)]
    #[inline]
    pub fn js_win_rate(&self) -> f64 {
        self.bt.borrow().win_rate()
    }

    #[wasm_bindgen(getter = profitFactor)]
    #[inline]
    pub fn js_profit_factor(&self) -> f64 {
        self.bt.borrow().profit_factor()
    }

    #[wasm_bindgen(js_name = "signal")]
    #[inline]
    pub fn js_signal(&mut self, signal: Signal) {
        self.bt.borrow_mut().signal(signal)
    }

    #[wasm_bindgen(js_name = "signalBatch")]
    #[inline]
    #[doc = "Processes multiple signals at once. `signals` must be aligned with all bars. `signals: [bar_index_0_signal, bar_index_1_signal, ...Signal[]]`."]
    pub fn js_signal_batch(&mut self, signals: js_sys::Array) {
        let signals: Vec<Option<Signal>> = signals
            .iter()
            .map(|signal_js_value| {
                if signal_js_value.is_null() {
                    None
                } else {
                    Some(
                        Signal::try_from_js_value(signal_js_value)
                            .expect_throw("Signal is not an instance of Signal"),
                    )
                }
            })
            .collect();
        self.bt.borrow_mut().signal_batch(signals)
    }

    #[wasm_bindgen(js_name = "signalBatchMap")]
    #[inline]
    #[doc = "Processes multiple signals at once. `signals: Map<bar_index: number, Signal>`."]
    pub fn js_signal_batch_dict(&mut self, signals: js_sys::Map) {
        let mut _signals: HashMap<usize, Signal> = HashMap::new();
        signals.for_each(&mut |signal_js_value, bar_index_js_value| {
            let bar_index: usize = bar_index_js_value
                .as_f64()
                .expect_throw(&format!("Signal map key is not a number",))
                as usize;
            let signal: Signal = Signal::try_from_js_value(signal_js_value).expect_throw(&format!(
                "Signal at bar_index {} is not an instance of Signal",
                bar_index
            ));
            _signals.insert(bar_index, signal);
        });
        self.bt.borrow_mut().signal_batch_dict(_signals)
    }

    #[wasm_bindgen(js_name = "skipTo")]
    #[inline]
    pub fn js_skip_to(&mut self, bar_index: usize) {
        self.bt.borrow_mut().skip_to_bar(bar_index)
    }

    #[wasm_bindgen(js_name = "skip")]
    #[inline]
    pub fn js_skip(&mut self, bars: Option<usize>) {
        if bars.is_none() {
            self.bt.borrow_mut().skip_remaining_bars()
        } else {
            self.bt.borrow_mut().skip_bars(bars.unwrap())
        }
    }

    #[wasm_bindgen(getter = length)]
    #[inline]
    pub fn js_length(&self) -> usize {
        self.bt.borrow().len()
    }

    #[wasm_bindgen(js_name = "toPine")]
    #[inline]
    pub fn js_to_pine(&self) -> String {
        self.bt.borrow().to_pine()
    }

    #[wasm_bindgen(getter = metrics)]
    pub fn js_metrics(&self) -> JsValue {
        let bt = self.bt.borrow();
        let obj = Object::new();

        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("equity"),
            &JsValue::from_f64(bt.equity()),
        );
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("netEquity"),
            &JsValue::from_f64(bt.net_equity()),
        );
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("netProfit"),
            &JsValue::from_f64(bt.net_profit()),
        );
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("profitFactor"),
            &JsValue::from_f64(bt.profit_factor()),
        );
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("winRate"),
            &JsValue::from_f64(bt.win_rate()),
        );
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("positionSize"),
            &JsValue::from_f64(bt.position_size()),
        );
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("openTrades"),
            &JsValue::from_f64(bt.open_trades().len() as f64),
        );
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("closedTrades"),
            &JsValue::from_f64(bt.closed_trades().len() as f64),
        );

        obj.into()
    }

    #[wasm_bindgen(js_name = "print")]
    pub fn js_print(&self) {
        let metrics = self.js_metrics();
        js_console_table(&metrics);
    }
}
