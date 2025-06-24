use crate::backtest::{BacktestSummary, BacktestSummaryConfig};
use crate::ctx_wasm::{WasmCtx, WasmCtxSkip};
use crate::signal::Signal;
use crate::signal_wasm::WasmSignal;
use crate::{
    backtest::{Backtest, BacktestConfig},
    trade::Trade,
};
use js_sys::{Object, Reflect};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::convert::TryFromJsValue;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = table)]
    pub fn js_console_table(obj: &JsValue);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn js_console_log(s: &str);
}

#[wasm_bindgen(js_name=Backtest)]
#[derive(Clone)]
pub struct WasmBacktest {
    inner: Rc<RefCell<Backtest>>,
    ctx: WasmCtx,
}

impl WasmBacktest {
    #[inline]
    pub fn inner(&self) -> &Rc<RefCell<Backtest>> {
        &self.inner
    }
}

#[wasm_bindgen(js_class=Backtest)]
impl WasmBacktest {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn wasm_new(
        ctx: WasmCtx,
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
            ctx,
        }
    }

    #[wasm_bindgen(getter = initialCapital)]
    #[inline]
    pub fn wasm_initial_capital(&self) -> f64 {
        self.inner.borrow().config().initial_capital()
    }

    #[wasm_bindgen(getter = processOrdersOnClose)]
    #[inline]
    pub fn wasm_process_orders_on_close(&self) -> bool {
        self.inner.borrow().config().process_orders_on_close()
    }

    #[wasm_bindgen(getter = ctx)]
    #[inline]
    pub fn wasm_ctx(&self) -> WasmCtx {
        self.ctx.clone()
    }

    #[wasm_bindgen(getter = equity)]
    #[inline]
    pub fn wasm_equity(&self) -> f64 {
        self.inner.borrow().equity()
    }

    #[wasm_bindgen(getter = netEquity)]
    #[inline]
    pub fn wasm_net_equity(&self) -> f64 {
        self.inner.borrow().net_equity()
    }

    #[wasm_bindgen(getter = equityList)]
    #[inline]
    pub fn wasm_equity_list(&self) -> Vec<f64> {
        self.inner.borrow().equity_list().to_vec()
    }

    #[wasm_bindgen(getter = netEquityList)]
    #[inline]
    pub fn wasm_net_equity_list(&self) -> Vec<f64> {
        self.inner.borrow().net_equity_list().to_vec()
    }

    #[wasm_bindgen(getter = pnlList)]
    #[inline]
    pub fn wasm_pnl_list(&self) -> Vec<f64> {
        self.inner.borrow().pnl_list()
    }

    #[wasm_bindgen(getter = openProfit)]
    #[inline]
    pub fn wasm_open_profit(&self) -> f64 {
        self.inner.borrow().open_profit()
    }

    #[wasm_bindgen(getter = netProfit)]
    #[inline]
    pub fn wasm_net_profit(&self) -> f64 {
        self.inner.borrow().net_profit()
    }

    #[wasm_bindgen(getter = grossProfit)]
    #[inline]
    pub fn wasm_gross_profit(&self) -> f64 {
        self.inner.borrow().gross_profit()
    }

    #[wasm_bindgen(getter = grossLoss)]
    #[inline]
    pub fn wasm_gross_loss(&self) -> f64 {
        self.inner.borrow().gross_loss()
    }

    #[wasm_bindgen(getter = positionSize)]
    #[inline]
    pub fn wasm_position_size(&self) -> f64 {
        self.inner.borrow().position_size()
    }

    #[wasm_bindgen(getter = trades)]
    #[inline]
    pub fn wasm_trades(&self) -> Vec<Trade> {
        self.inner.borrow().trades().into_iter().cloned().collect()
    }

    #[wasm_bindgen(js_name = "onBarOpen")]
    #[inline]
    pub fn wasm_on_bar_open(&mut self) {
        self.inner.borrow_mut().on_bar_open();
    }

    #[wasm_bindgen(js_name = "onBarClose")]
    #[inline]
    pub fn wasm_on_bar_close(&mut self) {
        self.inner.borrow_mut().on_bar_close();
    }

    #[wasm_bindgen(js_name = "signal")]
    #[inline]
    pub fn wasm_signal(&mut self, signal: WasmSignal) {
        self.inner.borrow_mut().signal(signal.into())
    }

    #[wasm_bindgen(js_name = "signalList")]
    #[inline]
    pub fn wasm_signal_list(&mut self, signals: js_sys::Array) {
        let signals: Vec<Option<WasmSignal>> = signals
            .iter()
            .map(|signal_js_value| {
                if signal_js_value.is_null() {
                    None
                } else {
                    Some(
                        WasmSignal::try_from_js_value(signal_js_value)
                            .expect_throw("Signal is not an instance of Signal"),
                    )
                }
            })
            .collect();
        self.inner
            .borrow_mut()
            .signal_list(signals.into_iter().map(|s| s.map(|s| s.into())).collect());
    }

    #[wasm_bindgen(js_name = "signalMap")]
    #[inline]
    pub fn wasm_signal_batch_map(&mut self, signals: js_sys::Map) {
        let mut _signals: HashMap<usize, Signal> = HashMap::new();
        signals.for_each(&mut |signal_js_value, bar_index_js_value| {
            let bar_index: usize = bar_index_js_value
                .as_f64()
                .expect_throw(&format!("Signal map key is not a number",))
                as usize;
            let signal = WasmSignal::try_from_js_value(signal_js_value).expect_throw(&format!(
                "Signal at bar_index {} is not an instance of Signal",
                bar_index
            ));
            _signals.insert(bar_index, signal.into());
        });
        self.inner.borrow_mut().signal_map(_signals)
    }

    #[wasm_bindgen(js_name = "skip")]
    #[inline]
    pub fn wasm_skip(&mut self, skip: WasmCtxSkip) {
        self.inner.borrow_mut().skip(skip.into());
    }

    #[wasm_bindgen(js_name = "length")]
    #[inline]
    pub fn wasm_length(&self) -> usize {
        self.inner.borrow().len()
    }

    #[wasm_bindgen(js_name = "next")]
    #[inline]
    pub fn wasm_next(&mut self) -> Option<usize> {
        let mut bt = self.inner.borrow_mut();
        let next = bt.ctx().borrow_mut().next();
        if next.is_none() {
            return None;
        }
        bt.on_bar_open();
        return Some(next.unwrap());
    }

    #[wasm_bindgen(js_name = "toPine")]
    #[inline]
    pub fn wasm_to_pine(&self) -> String {
        self.inner.borrow().to_pine()
    }

    #[wasm_bindgen(js_name = "summary")]
    #[inline]
    pub fn wasm_summary(&self, risk_free_rate: Option<f64>) -> WasmBacktestSummary {
        let risk_free_rate = risk_free_rate.unwrap_or(0.0);
        self.inner
            .borrow()
            .summary(&BacktestSummaryConfig { risk_free_rate })
            .into()
    }

    // #[wasm_bindgen(js_namespace = Symbol, js_name = iterator)]
    // #[inline]
    // pub fn js_symbol_iterator(this: &WasmBacktest) -> WasmBacktestIter {
    //     WasmBacktestIter { bt: this.clone() }
    // }
}

// #[wasm_bindgen]
// pub struct WasmBacktestIter {
//     bt: WasmBacktest,
// }

// #[wasm_bindgen]
// impl WasmBacktestIter {
//     #[wasm_bindgen]
//     pub fn next(&mut self) -> js_sys::Object {
//         let obj = js_sys::Object::new();

//         match self.bt.wasm_next() {
//             Some(bar_index) => {
//                 js_sys::Reflect::set(
//                     &obj,
//                     &JsValue::from_str("value"),
//                     &JsValue::from_f64(bar_index as f64),
//                 )
//                 .unwrap();
//                 js_sys::Reflect::set(&obj, &JsValue::from_str("done"), &JsValue::FALSE).unwrap();
//             }
//             None => {
//                 js_sys::Reflect::set(&obj, &JsValue::from_str("value"), &JsValue::UNDEFINED)
//                     .unwrap();
//                 js_sys::Reflect::set(&obj, &JsValue::from_str("done"), &JsValue::TRUE).unwrap();
//             }
//         }

//         obj
//     }
// }

#[wasm_bindgen(js_name=BacktestSummary)]
#[derive(Clone)]
pub struct WasmBacktestSummary {
    inner: BacktestSummary,
}

impl Into<WasmBacktestSummary> for BacktestSummary {
    fn into(self) -> WasmBacktestSummary {
        WasmBacktestSummary { inner: self }
    }
}

impl Into<BacktestSummary> for WasmBacktestSummary {
    fn into(self) -> BacktestSummary {
        self.inner
    }
}

#[wasm_bindgen(js_class=BacktestSummary)]
impl WasmBacktestSummary {
    #[wasm_bindgen(js_name = "display")]
    #[inline]
    pub fn wasm_display(&self) {
        let obj = Object::new();

        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("equity"),
            &JsValue::from_f64(self.inner.equity),
        );
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("netEquity"),
            &JsValue::from_f64(self.inner.net_equity),
        );
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("netProfit"),
            &JsValue::from_f64(self.inner.net_profit),
        );
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("profitFactor"),
            &JsValue::from_f64(self.inner.profit_factor),
        );
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("winRate"),
            &JsValue::from_f64(self.inner.win_rate),
        );
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("positionSize"),
            &JsValue::from_f64(self.inner.position_size),
        );
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("openTrades"),
            &JsValue::from_f64(self.inner.open_trades as f64),
        );
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("closedTrades"),
            &JsValue::from_f64(self.inner.closed_trades as f64),
        );
        js_console_table(&obj);
    }

    #[wasm_bindgen(js_name = "toJSON")]
    #[inline]
    pub fn wasm_to_json(&self) -> JsValue {
        let obj = Object::new();
        Reflect::set(
            &obj,
            &JsValue::from_str("equity"),
            &JsValue::from_f64(self.inner.equity),
        )
        .unwrap();
        obj.into()
    }
}
