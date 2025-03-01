cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
}}
cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}
use crate::{
    ctx::Ctx,
    orderbook::{OrderBook, OrderBookConfig, OrderBookError, OrderConfig},
    rs_utils::Float64Utils,
    signal::{Signal, SignalKind},
    trade::{Trade, TradeError, TradeEvent},
    utils::{order_size_for_equity_pct, profit_factor, returns, round_to_min_tick, win_rate},
};
use core::f64;
use serde_json::json;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "BacktestConfig"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "BacktestConfig"))]
#[derive(Debug, Clone, Copy)]
pub struct BacktestConfig {
    initial_capital: f64,
    process_orders_on_close: bool,
    debug: bool,
}

impl Default for BacktestConfig {
    fn default() -> Self {
        Self {
            initial_capital: 1000.0,
            process_orders_on_close: false,
            debug: false,
        }
    }
}
impl BacktestConfig {
    pub fn new(initial_capital: f64, process_orders_on_close: bool) -> Self {
        return Self {
            initial_capital,
            process_orders_on_close,
            debug: false,
        };
    }

    #[inline]
    pub fn initial_capital(&self) -> f64 {
        self.initial_capital
    }

    #[inline]
    pub fn process_orders_on_close(&self) -> bool {
        self.process_orders_on_close
    }
}

pub struct Backtest {
    ctx: Rc<RefCell<Ctx>>,
    orderbook: Rc<RefCell<OrderBook>>,
    config: BacktestConfig,
    closed_trades: VecDeque<Trade>,
    open_trades: VecDeque<Trade>,
    //
    instrument_price: f64,
    initial_capital: f64,
    equity: Vec<f64>,
    net_equity: Vec<f64>,
    open_profit: f64,
    net_profit: f64,
    gross_profit: f64,
    gross_loss: f64,
    winning_trades: usize,
    losing_trades: usize,
    position_size: f64,
    open_longs: usize,
    open_shorts: usize,
    closed_longs: usize,
    closed_shorts: usize,
    first_entry_bar_index: Option<usize>,
    prev_equity_pct: f64,
}

impl Backtest {
    #[inline]
    pub fn new(ctx: Rc<RefCell<Ctx>>, config: BacktestConfig) -> Self {
        let min_qty = ctx.borrow().sym_info().min_qty();
        let initial_capital = config.initial_capital;
        let mut s = Self {
            ctx,
            config,
            orderbook: Rc::new(RefCell::new(OrderBook::new(OrderBookConfig {
                min_size: min_qty,
                debug: config.debug,
                ..Default::default()
            }))),
            instrument_price: f64::NAN,
            open_trades: VecDeque::new(),
            closed_trades: VecDeque::new(),
            initial_capital,
            equity: vec![],
            net_equity: vec![],
            net_profit: 0.0,
            open_profit: 0.0,
            gross_loss: 0.0,
            gross_profit: 0.0,
            losing_trades: 0,
            winning_trades: 0,
            position_size: 0.0,
            closed_longs: 0,
            closed_shorts: 0,
            open_longs: 0,
            open_shorts: 0,
            first_entry_bar_index: None,
            prev_equity_pct: 0.0,
        };
        return s;
    }

    // #[inline]
    // fn init(&mut self) {
    //     self.set_price();
    //     self.set_metrics().unwrap();
    // }
    #[inline]
    pub fn config(&self) -> &BacktestConfig {
        &self.config
    }

    #[inline]
    pub fn ctx(&self) -> Rc<RefCell<Ctx>> {
        self.ctx.clone()
    }

    #[inline]
    pub fn equity(&self) -> f64 {
        self.equity.last().cloned().unwrap_or(f64::NAN)
    }

    #[inline]
    pub fn net_equity(&self) -> f64 {
        self.net_equity.last().cloned().unwrap_or(f64::NAN)
    }

    #[inline]
    pub fn equity_series(&self) -> &[f64] {
        &self.equity
    }

    #[inline]
    pub fn net_equity_series(&self) -> &[f64] {
        &self.net_equity
    }

    #[inline]
    pub fn equity_returns(&self) -> Vec<f64> {
        returns(&self.equity, true)
    }

    #[inline]
    pub fn net_equity_returns(&self) -> Vec<f64> {
        returns(&self.net_equity, true)
    }

    #[inline]
    pub fn pnl_series(&self) -> Vec<f64> {
        let mut pnl_series: Vec<f64> = vec![];
        for trade in &self.closed_trades {
            pnl_series.push(trade.pnl());
        }
        return pnl_series;
    }

    #[inline]
    pub fn open_profit(&self) -> f64 {
        self.open_profit
    }

    #[inline]
    pub fn net_profit(&self) -> f64 {
        self.net_profit
    }

    #[inline]
    pub fn gross_profit(&self) -> f64 {
        self.gross_profit
    }

    #[inline]
    pub fn gross_loss(&self) -> f64 {
        self.gross_loss
    }

    #[inline]
    pub fn winning_trades(&self) -> usize {
        self.winning_trades
    }

    #[inline]
    pub fn losing_trades(&self) -> usize {
        self.losing_trades
    }

    #[inline]
    pub fn position_size(&self) -> f64 {
        self.position_size
    }

    #[inline]
    pub fn open_trades(&self) -> &[Trade] {
        self.open_trades.as_slices().0
    }

    #[inline]
    pub fn closed_trades(&self) -> &[Trade] {
        self.closed_trades.as_slices().0
    }

    #[inline]
    pub fn trades(&self) -> Vec<&Trade> {
        self.open_trades
            .iter()
            .chain(self.closed_trades.iter())
            .collect::<Vec<_>>()
    }

    #[inline]
    pub fn open_longs(&self) -> usize {
        self.open_longs
    }

    #[inline]
    pub fn open_shorts(&self) -> usize {
        self.open_shorts
    }

    #[inline]
    pub fn closed_longs(&self) -> usize {
        self.closed_longs
    }

    #[inline]
    pub fn closed_shorts(&self) -> usize {
        self.closed_shorts
    }

    #[inline]
    pub fn total_longs(&self) -> usize {
        self.open_longs + self.closed_longs
    }

    #[inline]
    pub fn total_shorts(&self) -> usize {
        self.open_shorts + self.closed_shorts
    }

    #[inline]
    pub fn total_trades(&self) -> usize {
        self.open_trades.len() + self.closed_trades.len()
    }

    #[inline]
    pub fn first_entry_bar_index(&self) -> Option<usize> {
        self.first_entry_bar_index
    }

    #[inline]
    pub fn instrument_size(&self) -> f64 {
        self.instrument_price
    }

    #[inline]
    pub fn win_rate(&self) -> f64 {
        win_rate(self.winning_trades(), self.total_trades())
    }

    #[inline]
    pub fn profit_factor(&self) -> f64 {
        profit_factor(self.gross_profit(), self.gross_loss())
    }

    fn set_price(&mut self) {
        let ctx = self.ctx.borrow();
        let bar = ctx.bar();
        let sym_info = ctx.sym_info();

        let orderbook_price = if self.config.process_orders_on_close {
            bar.close()
        } else {
            bar.open()
        };

        self.instrument_price = sym_info.round_to_min_tick(bar.close());
        self.orderbook
            .borrow_mut()
            .set_price(sym_info.round_to_min_tick(orderbook_price));
    }

    fn set_position_size(&mut self, size: f64) {
        let ctx = self.ctx.borrow();
        let sym_info = ctx.sym_info();
        self.position_size = sym_info.round_contracts(size);
    }

    fn set_metrics(&mut self) -> Result<(), TradeError> {
        let net_equity = self.initial_capital + self.net_profit;

        let mut open_profit = 0.0;

        for trade in &mut self.open_trades {
            trade.set_pnl_from_price(self.instrument_price)?;
            open_profit += trade.pnl();
        }

        let equity = net_equity + open_profit;

        self.equity.last_mut().map(|v| *v = equity);
        self.net_equity.last_mut().map(|v| *v = net_equity);
        self.open_profit = open_profit;

        return Ok(());
    }

    #[inline]
    fn create_trade_event(&self) -> TradeEvent {
        let bar_index = self.ctx.borrow().bar_index();
        let fill_bar_index = bar_index;
        let order_bar_index = if self.config.process_orders_on_close {
            fill_bar_index
        } else {
            fill_bar_index - 1
        };

        let mut trade = TradeEvent::default();
        trade
            .set_fill_bar_index(fill_bar_index)
            .set_order_bar_index(order_bar_index)
            .set_price(self.orderbook.borrow().price());

        return trade;
    }

    fn compute_equity_pct(&mut self, equity_pct: f64) -> Option<OrderConfig> {
        let ctx = self.ctx.borrow();
        if self.equity() > 0.0 {
            if !equity_pct.compare(self.prev_equity_pct) {
                // if true {
                self.prev_equity_pct = equity_pct;

                let order_size = order_size_for_equity_pct(
                    equity_pct,
                    self.equity(),
                    self.position_size(),
                    self.instrument_price,
                    1.0,
                    1.0,
                );

                let order_size = ctx.sym_info().round_contracts(order_size);

                if order_size == 0.0 {
                    return None;
                }

                self.prev_equity_pct = equity_pct;
                return Some(OrderConfig::new(
                    order_size,
                    Some(format!("{:?}", ctx.bar_index())),
                ));
            }
        }

        return None;
    }

    fn next(&mut self) {
        self.equity.push(f64::NAN);
        self.net_equity.push(f64::NAN);
    }

    pub fn on_bar_open(&mut self) {
        self.next();
        // println!(
        //     "[{:?}] {} {:?}",
        //     self.ctx.bar_index(),
        //     self.equity.values.len(),
        //     self.equity.get()
        // );
        self.set_price();
        if !self.config.process_orders_on_close {
            self.process_orderbook().unwrap();
        }
    }

    pub fn on_bar_close(&mut self) {
        self.set_price();
        if self.config.process_orders_on_close {
            self.process_orderbook().unwrap();
        }
    }

    fn on_trade_open(&mut self, size: f64, entry_id: Option<String>) -> Result<(), TradeError> {
        let mut trade = Trade::new();
        trade.set_size(size)?;

        let mut event = self.create_trade_event();

        if self.first_entry_bar_index.is_none() {
            self.first_entry_bar_index = Some(event.fill_bar_index());
        }

        event.set_id(entry_id);
        trade.set_entry(event)?;

        /*
        if trade.direction() == TradeDirection::Long {
            self.open_longs += 1;
        } else {
            self.open_shorts += 1;
        }
        */

        self.set_position_size(self.position_size + trade.size());
        self.open_trades.push_back(trade);

        return Ok(());
    }

    fn on_trade_close(
        &mut self,
        trade: &mut Trade,
        exit_id: Option<String>,
    ) -> Result<(), TradeError> {
        let mut event = self.create_trade_event();
        event.set_id(exit_id);

        trade.set_pnl_from_price(event.price())?;
        trade.set_exit(event)?;

        self.net_profit = self.net_profit + trade.pnl();

        if trade.pnl() > 0.0 {
            self.winning_trades += 1;
            self.gross_profit += trade.pnl();
        } else {
            self.losing_trades += 1;
            self.gross_loss += trade.pnl().abs();
        }

        /*
        if trade.direction() == TradeDirection::Long {
            self.open_longs -= 1;
            self.closed_longs += 1;
        } else {
            self.open_shorts -= 1;
            self.closed_shorts += 1;
        }
        */

        self.set_position_size(self.position_size - trade.size());

        self.closed_trades.push_back(trade.clone());

        return Ok(());
    }

    fn process_orderbook(&mut self) -> Result<(), TradeError> {
        loop {
            let order_id = self.orderbook.borrow_mut().pop_front();
            if order_id.is_none() {
                break;
            }
            let order_id = order_id.unwrap();
            let order = {
                let orderbook = self.orderbook.borrow();
                orderbook
                    .get_order(order_id)
                    .expect("Order should exist")
                    .clone()
            };

            if self.config.debug {
                self.log("process_orderbook", format!("{:?}", order));
            }

            let mut fill_size = order.size();
            let mut open_trade_index = 0;

            loop {
                if fill_size == 0.0 || open_trade_index >= self.open_trades.len() {
                    break;
                }

                let open_trade = &mut self.open_trades[open_trade_index];

                if open_trade.size().signum() != order.size().signum() {
                    // close entire trade
                    if open_trade.size().abs() <= fill_size.abs() {
                        let mut closed_trade = self.open_trades.remove(open_trade_index).unwrap();
                        fill_size += closed_trade.size();

                        self.on_trade_close(&mut closed_trade, order.tag().clone())?;

                        continue;
                    }
                    // partially closed trade
                    else {
                        let open_partial_size = open_trade.size() + fill_size;

                        open_trade.set_size(open_partial_size)?;

                        let closed_partial_size = fill_size.abs() * open_trade.size().signum();
                        let mut closed_trade = open_trade.clone();
                        closed_trade.set_size(closed_partial_size)?;
                        self.on_trade_close(&mut closed_trade, order.tag().clone())?;

                        fill_size = 0.0;
                    }
                }

                open_trade_index += 1;
            }

            if self.ctx.borrow().sym_info().validate_contracts(fill_size) {
                self.on_trade_open(fill_size, order.tag().clone())?;
            }
        }

        self.set_metrics()?;

        return Ok(());
    }

    pub fn create_hold(
        ctx: Rc<RefCell<Ctx>>,
        config: BacktestConfig,
        range: (usize, usize),
        qty: f64,
    ) -> Backtest {
        let (entry, exit) = range;

        let mut bt = Backtest::new(ctx.clone(), config);

        for bar_index in ctx.borrow_mut().into_iter() {
            bt.on_bar_open();

            if bar_index == entry {
                let order = OrderConfig::new(qty, Some("hold_entry".to_string()));
                bt.order(order).unwrap();
            } else if bar_index == exit {
                let order = OrderConfig::new(-qty, Some("hold_exit".to_string()));
                bt.order(order).unwrap();
            }

            bt.on_bar_close();
        }
        return bt;
    }

    fn log(&self, ctx: &str, msg: String) {
        let obj = json!({
            "bar_index":self.ctx.borrow().bar_index(),
            "position":
            self.position_size,
        });
        println!(
            "\n[{}]: {}\n{}",
            ctx,
            msg,
            serde_json::to_string(&obj).unwrap()
        );
    }

    #[inline]
    pub fn order(&mut self, order_config: OrderConfig) -> Result<usize, OrderBookError> {
        if self.config.debug {
            self.log("order", format!("{:?}", order_config));
        }
        return self.orderbook.borrow_mut().enqueue(order_config);
    }

    #[inline]
    pub fn signal(&mut self, signal: Signal) {
        let order: Option<OrderConfig> = match signal.kind() {
            SignalKind::EquityPct(pct) => self.compute_equity_pct(*pct),
            SignalKind::Size(size) => Some(OrderConfig::new(*size, None)),
            SignalKind::CloseAll() => Some(OrderConfig::new(-self.position_size, None)),
            _ => None,
        };
        if order.is_some() {
            self.orderbook.borrow_mut().enqueue(order.unwrap()).unwrap();
        }
    }

    #[inline]
    pub fn signal_batch(&mut self, signals: Vec<Option<Signal>>) {
        for signal in signals {
            let next = self.ctx.borrow_mut().next();
            if next.is_none() {
                break;
            }
            self.on_bar_open();
            if signal.is_some() {
                self.signal(signal.unwrap());
            }
            self.on_bar_close();
        }
    }

    #[inline]
    pub fn signal_batch_dict(&mut self, signals: HashMap<usize, Signal>) {
        let signals: Vec<Option<Signal>> = (0..self.ctx.borrow().len())
            .map(|i| signals.get(&i).cloned())
            .collect();
        self.signal_batch(signals);
    }

    #[inline]
    pub fn skip_remaining_bars(&mut self) {
        while self.ctx.borrow_mut().next().is_some() {
            self.on_bar_open();
            self.on_bar_close();
        }
    }

    #[inline]
    pub fn skip_to_bar(&mut self, bar_index: usize) {
        while self.ctx.borrow().bar_index() < bar_index {
            let next = self.ctx.borrow_mut().next();
            if next.is_none() {
                break;
            }
            self.on_bar_open();
            self.on_bar_close();
        }
    }

    #[inline]
    pub fn skip_bars(&mut self, bars: usize) {
        let bar_index = self.ctx.borrow().bar_index();
        self.skip_to_bar(bar_index + bars);
    }

    pub fn to_pine(&self) -> String {
        let initial_capital = self.initial_capital;
        let mut pine = "".to_string();
        pine += "//@version=5";
        pine += &format!("\nstrategy(\"Strategy export\", overlay=true, initial_capital={initial_capital}, default_qty_type = strategy.percent_of_equity, default_qty_value = 100)");
        pine += &format!("\n// Generated at {:?}", chrono::offset::Utc::now());
        pine += &format!(
            "
type Trade
    string id
    int entry_open_time_ms
    int exit_open_time_ms
    float size
        "
        );
        pine += &format!("\n// Generated at {:?}", chrono::offset::Utc::now());

        fn trade_to_pine(ctx: &Ctx, trade: &Trade, id: String) -> String {
            let entry_open_time_ms = trade
                .entry()
                .as_ref()
                .map(|r| {
                    ctx.ohlcv()
                        .bar(r.order_bar_index())
                        .open_time_ms()
                        .to_string()
                })
                .unwrap_or("na".to_string());
            let exit_open_time_ms = trade
                .exit()
                .as_ref()
                .map(|r| {
                    ctx.ohlcv()
                        .bar(r.order_bar_index())
                        .open_time_ms()
                        .to_string()
                })
                .unwrap_or("na".to_string());
            let size = trade.size();
            return format!("Trade.new(id=\"{id}\", entry_open_time_ms={entry_open_time_ms}, exit_open_time_ms={exit_open_time_ms}, size={size})");
        }

        let pine_trades = self
            .trades()
            .iter()
            .enumerate()
            .map(|(i, trade)| trade_to_pine(&self.ctx.borrow(), trade, format!("{i}")))
            .collect::<Vec<String>>();
        pine += &format!("\ntrades = array.from<Trade>({})", pine_trades.join(","));

        pine += &format!(
            "
open_time_ms = time

for i = 0 to array.size(trades) - 1
    item = array.get(trades, i)
    qty = math.abs(item.size)

    if item.entry_open_time_ms == open_time_ms
        _dir = item.size > 0 ? strategy.long : strategy.short
        strategy.order(id=item.id, direction=_dir, qty=qty)

    if item.exit_open_time_ms == open_time_ms
        _dir = item.size > 0 ? strategy.short : strategy.long
        strategy.order(id=item.id, direction=_dir, qty=qty)
    "
        );

        return pine;
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.ctx.borrow().len()
    }

    #[inline]
    pub fn run_and_dump_debug(&mut self, signals: Vec<Option<Signal>>) -> BacktestDebugDump {
        let mut dump = BacktestDebugDump::default();
        for signal in signals {
            let next = self.ctx.borrow_mut().next();
            if next.is_none() {
                break;
            }
            self.on_bar_open();
            if signal.is_some() {
                self.signal(signal.unwrap());
            }
            self.on_bar_close();
            dump.next(self);
        }
        return dump;
    }
}

pub struct BacktestDebugDump {
    pub equity: Vec<f64>,
    pub net_equity: Vec<f64>,
    pub position_size: Vec<f64>,
    pub open_profit: Vec<f64>,
    pub net_profit: Vec<f64>,
    pub gross_profit: Vec<f64>,
    pub gross_loss: Vec<f64>,
    pub winning_trades: Vec<usize>,
    pub losing_trades: Vec<usize>,
    pub trades: Vec<Vec<Trade>>,
}

impl Default for BacktestDebugDump {
    fn default() -> Self {
        Self {
            equity: vec![],
            net_equity: vec![],
            position_size: vec![],
            open_profit: vec![],
            net_profit: vec![],
            gross_profit: vec![],
            gross_loss: vec![],
            winning_trades: vec![],
            losing_trades: vec![],
            trades: vec![],
        }
    }
}

impl BacktestDebugDump {
    pub fn next(&mut self, bt: &mut Backtest) {
        self.equity.push(bt.equity());
        self.net_equity.push(bt.net_equity());
        self.position_size.push(bt.position_size());
        self.open_profit.push(bt.open_profit());
        self.net_profit.push(bt.net_profit());
        self.gross_profit.push(bt.gross_profit());
        self.gross_loss.push(bt.gross_loss());
        self.winning_trades.push(bt.winning_trades());
        self.losing_trades.push(bt.losing_trades());
        self.trades.push(bt.trades().into_iter().cloned().collect());
    }

    pub fn assert_length(&self, len: usize) {
        assert_eq!(self.equity.len(), len, "equity");
        assert_eq!(self.net_equity.len(), len, "net_equity");
        assert_eq!(self.position_size.len(), len, "position_size");
        assert_eq!(self.open_profit.len(), len, "open_profit");
        assert_eq!(self.net_profit.len(), len, "net_profit");
        assert_eq!(self.gross_profit.len(), len, "gross_profit");
        assert_eq!(self.gross_loss.len(), len, "gross_loss");
        assert_eq!(self.winning_trades.len(), len, "winning_trades");
        assert_eq!(self.losing_trades.len(), len, "losing_trades");
        assert_eq!(self.trades.len(), len, "trades");
    }

    pub fn assert_compare_at(&self, other: &BacktestDebugDump, idx: usize) {
        let price_eps = 1.0;
        let position_size_eps = 0.000001;
        assert!(
            self.equity[idx].compare_with_precision(other.equity[idx], price_eps),
            "[{}] equity: {:?} != {:?}",
            idx,
            self.equity[idx],
            other.equity[idx]
        );
        assert!(
            self.net_equity[idx].compare_with_precision(other.net_equity[idx], price_eps),
            "[{}] net_equity: {:?} != {:?}",
            idx,
            self.net_equity[idx],
            other.net_equity[idx]
        );
        assert!(
            self.position_size[idx]
                .compare_with_precision(other.position_size[idx], position_size_eps),
            "[{}] position_size: {:?} != {:?}",
            idx,
            self.position_size[idx],
            other.position_size[idx]
        );
        assert!(
            self.open_profit[idx].compare_with_precision(other.open_profit[idx], price_eps),
            "[{}] open_profit: {:?} != {:?}",
            idx,
            self.open_profit[idx],
            other.open_profit[idx]
        );
        assert!(
            self.net_profit[idx].compare_with_precision(other.net_profit[idx], price_eps),
            "[{}] net_profit: {:?} != {:?}",
            idx,
            self.net_profit[idx],
            other.net_profit[idx]
        );
        assert!(
            self.gross_profit[idx].compare_with_precision(other.gross_profit[idx], price_eps),
            "[{}] gross_profit: {:?} != {:?}",
            idx,
            self.gross_profit[idx],
            other.gross_profit[idx]
        );
        assert!(
            self.gross_loss[idx].compare_with_precision(other.gross_loss[idx], price_eps),
            "[{}] gross_loss: {:?} != {:?}",
            idx,
            self.gross_loss[idx],
            other.gross_loss[idx]
        );
        assert_eq!(
            self.winning_trades[idx], other.winning_trades[idx],
            "[{}] winning_trades: {:?} != {:?}",
            idx, self.winning_trades[idx], other.winning_trades[idx]
        );
        assert_eq!(
            self.losing_trades[idx], other.losing_trades[idx],
            "[{}] losing_trades: {:?} != {:?}",
            idx, self.losing_trades[idx], other.losing_trades[idx]
        );
    }

    pub fn assert_compare(&mut self, other: &BacktestDebugDump) {
        self.assert_length(other.equity.len());
        for i in 0..self.equity.len() {
            self.assert_compare_at(other, i);
        }
    }
}
