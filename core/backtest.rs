use crate::{
    ctx::{Ctx, CtxSkip},
    legacy::Float64Utils,
    metrics::{
        avg_losing_trade, avg_trade, avg_win_loss_ratio, avg_winning_trade, gross_loss_pct,
        gross_profit_pct, net_profit_pct, profit_factor, sharpe_ratio_from_returns,
        sortino_ratio_from_returns, win_rate,
    },
    orderbook::{
        order_size_for_equity_pct, round_contracts, round_to_min_tick, validate_contracts,
        OrderBook, OrderBookConfig, OrderBookError, OrderConfig,
    },
    signal::{Signal, SignalKind},
    stats::returns,
    sym::Sym,
    trade::{Trade, TradeError, TradeEvent},
    utils::with_suffix,
};
use core::f64;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

cfg_if::cfg_if! { if #[cfg(feature = "pretty_table")] {
use comfy_table::{
    presets::{UTF8_FULL, UTF8_FULL_CONDENSED},
    Attribute, CellAlignment, Color,
};
use comfy_table::{Cell, ContentArrangement, Row, Table as ComfyTable};
use textplots::{Chart, Plot, Shape};
}}

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
    pub fn set_initial_capital(&mut self, initial_capital: f64) {
        self.initial_capital = initial_capital;
    }

    #[inline]
    pub fn process_orders_on_close(&self) -> bool {
        self.process_orders_on_close
    }

    #[inline]
    pub fn set_process_orders_on_close(&mut self, process_orders_on_close: bool) {
        self.process_orders_on_close = process_orders_on_close;
    }

    #[inline]
    pub fn debug(&self) -> bool {
        self.debug
    }

    #[inline]
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
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
        let min_qty = ctx.borrow().sym().min_qty();
        let initial_capital = config.initial_capital;
        Self {
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
        }
    }

    #[inline]
    pub fn config(&self) -> &BacktestConfig {
        &self.config
    }

    #[inline]
    pub fn ctx(&self) -> Rc<RefCell<Ctx>> {
        self.ctx.clone()
    }

    #[inline]
    pub fn initial_capital(&self) -> f64 {
        self.initial_capital
    }

    #[inline]
    pub fn equity(&self) -> f64 {
        self.equity.last().cloned().unwrap_or(self.initial_capital)
    }

    #[inline]
    pub fn equity_list(&self) -> &[f64] {
        &self.equity
    }

    #[inline]
    pub fn net_equity(&self) -> f64 {
        self.net_equity
            .last()
            .cloned()
            .unwrap_or(self.initial_capital)
    }

    #[inline]
    pub fn net_equity_list(&self) -> &[f64] {
        &self.net_equity
    }

    #[inline]
    pub fn pnl_list(&self) -> Vec<f64> {
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
    pub fn net_profit_pct(&self) -> f64 {
        net_profit_pct(self.net_profit, self.initial_capital)
    }

    #[inline]
    pub fn gross_profit(&self) -> f64 {
        self.gross_profit
    }

    #[inline]
    pub fn gross_profit_pct(&self) -> f64 {
        gross_profit_pct(self.gross_profit, self.initial_capital)
    }

    #[inline]
    pub fn gross_loss(&self) -> f64 {
        self.gross_loss
    }

    #[inline]
    pub fn gross_loss_pct(&self) -> f64 {
        gross_loss_pct(self.gross_loss, self.initial_capital)
    }

    #[inline]
    pub fn win_rate(&self) -> f64 {
        win_rate(self.winning_trades, self.losing_trades)
    }

    #[inline]
    pub fn profit_factor(&self) -> f64 {
        profit_factor(self.gross_profit, self.gross_loss)
    }

    #[inline]
    pub fn avg_trade(&self) -> f64 {
        avg_trade(self.net_profit, self.closed_trades.len())
    }

    #[inline]
    pub fn avg_winning_trade(&self) -> f64 {
        avg_winning_trade(self.gross_profit, self.winning_trades)
    }

    #[inline]
    pub fn avg_losing_trade(&self) -> f64 {
        avg_losing_trade(self.gross_loss, self.losing_trades)
    }

    #[inline]
    pub fn avg_win_loss_ratio(&self) -> f64 {
        avg_win_loss_ratio(self.avg_winning_trade(), self.avg_losing_trade())
    }

    #[inline]
    pub fn returns_list(&self) -> Vec<f64> {
        returns(&self.equity, true)
    }

    #[inline]
    pub fn sharpe_ratio(&self, rfr: f64) -> f64 {
        sharpe_ratio_from_returns(&self.returns_list(), rfr)
    }

    #[inline]
    pub fn sortino_ratio(&self, rfr: f64) -> f64 {
        sortino_ratio_from_returns(&self.returns_list(), rfr)
    }

    #[inline]
    pub fn winning_trades_count(&self) -> usize {
        self.winning_trades
    }

    #[inline]
    pub fn losing_trades_count(&self) -> usize {
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
    pub fn open_longs_count(&self) -> usize {
        self.open_longs
    }

    #[inline]
    pub fn open_shorts_count(&self) -> usize {
        self.open_shorts
    }

    #[inline]
    pub fn closed_longs_count(&self) -> usize {
        self.closed_longs
    }

    #[inline]
    pub fn closed_shorts_count(&self) -> usize {
        self.closed_shorts
    }

    #[inline]
    pub fn first_entry_bar_index(&self) -> Option<usize> {
        self.first_entry_bar_index
    }

    #[inline]
    pub fn instrument_size(&self) -> f64 {
        self.instrument_price
    }

    pub fn set_price(&mut self) {
        let ctx = self.ctx.borrow();
        let bar = ctx.bar();
        let sym_info = ctx.sym();

        let orderbook_price = if self.config.process_orders_on_close {
            bar.close()
        } else {
            bar.open()
        };

        self.instrument_price = round_to_min_tick(bar.close(), sym_info.min_tick());
        self.orderbook
            .borrow_mut()
            .set_price(round_to_min_tick(orderbook_price, sym_info.min_tick()));
    }

    pub fn set_position_size(&mut self, size: f64) {
        let ctx = self.ctx.borrow();
        let sym_info = ctx.sym();
        self.position_size = round_contracts(size, sym_info.min_qty(), sym_info.price_scale());
    }

    pub fn set_metrics(&mut self) -> Result<(), TradeError> {
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
    pub fn create_trade_event(&self) -> TradeEvent {
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

    pub fn compute_equity_pct(&mut self, equity_pct: f64) -> Option<OrderConfig> {
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

                let order_size =
                    round_contracts(order_size, ctx.sym().min_qty(), ctx.sym().price_scale());

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

    pub fn next(&mut self) {
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

    pub fn on_trade_open(&mut self, size: f64, entry_id: Option<String>) -> Result<(), TradeError> {
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

    pub fn on_trade_close(
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

    pub fn process_orderbook(&mut self) -> Result<(), TradeError> {
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

            if validate_contracts(fill_size, self.ctx.borrow().sym().min_qty()) {
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

    pub fn log(&self, ctx: &str, msg: String) {
        //     let obj = json!({
        //         "bar_index":self.ctx.borrow().bar_index(),
        //         "position":
        //         self.position_size,
        //     });
        //     println!(
        //         "\n[{}]: {}\n{}",
        //         ctx,
        //         msg,
        //         serde_json::to_string(&obj).unwrap()
        //     );
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
    pub fn signal_list(&mut self, signals: Vec<Option<Signal>>) {
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
    pub fn signal_map(&mut self, signals: HashMap<usize, Signal>) {
        let signals: Vec<Option<Signal>> = (0..self.ctx.borrow().len())
            .map(|i| signals.get(&i).cloned())
            .collect();
        self.signal_list(signals);
    }

    #[inline]
    pub fn skip(&mut self, skip: CtxSkip) {
        let target = skip.get_target_bar_index(&self.ctx.borrow());
        let count = self.ctx.borrow().bar_index() - target;
        for _ in 0..count {
            let next = self.ctx.borrow_mut().next();
            if next.is_none() {
                break;
            }
            self.on_bar_open();
            self.on_bar_close();
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.ctx.borrow().len()
    }

    pub fn to_pine(&self) -> String {
        let initial_capital = self.initial_capital;
        let mut pine = "".to_string();
        pine += "//@version=5";
        pine += &format!("\nstrategy(\"Strategy export\", overlay=true, initial_capital={initial_capital}, default_qty_type = strategy.percent_of_equity, default_qty_value = 100)");
        pine += &format!("\n// Generated at {:?}", chrono::offset::Utc::now());
        pine += &format!("\n// by https://qpace.dev - The technical analysis framework\n\n");
        pine += &format!(
            "
type Trade
    string id
    int entry_open_time_ms
    int exit_open_time_ms
    float size
        "
        );

        fn trade_to_pine(ctx: &Ctx, trade: &Trade, id: String) -> String {
            let entry_open_time_ms = trade
                .entry()
                .as_ref()
                .map(|r| {
                    ctx.ohlcv()
                        .get(r.order_bar_index())
                        .unwrap()
                        .open_time()
                        .unwrap()
                        .timestamp_millis()
                        .to_string()
                })
                .unwrap_or("na".to_string());
            let exit_open_time_ms = trade
                .exit()
                .as_ref()
                .map(|r| {
                    ctx.ohlcv()
                        .get(r.order_bar_index())
                        .unwrap()
                        .open_time()
                        .unwrap()
                        .timestamp_millis()
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
    pub fn dump_bar(&self) -> BacktestBarDump {
        BacktestBarDump::dump(self)
    }

    #[cfg(feature = "pretty_table")]
    pub fn print_table(&self) {
        let rfr = 0.0;
        let sym = self.ctx.borrow().sym().clone();
        let f_price = with_suffix(&format!(" {}", sym._currency()));
        let f_percent = with_suffix("%");
        let f = |price: f64, percent: f64| format!("{}\n{}", f_price(price), f_percent(percent));
        let f_raw = |value: f64| format!("{:0.2}", value);

        let mut table = ComfyTable::new();
        table
            .set_content_arrangement(ContentArrangement::DynamicFullWidth)
            .set_header(vec!["Metric", "Value"]);

        table.add_row(Row::from(vec![
            Cell::new("Net Profit"),
            Cell::new(f(self.net_profit, self.net_profit_pct() * 100.0)),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Gross Profit"),
            Cell::new(f(self.gross_profit, self.gross_profit_pct() * 100.0)),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Gross Loss"),
            Cell::new(f(self.gross_loss, self.gross_loss_pct() * 100.0)),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Sharpe Ratio"),
            Cell::new(format!("{:0.3}", self.sharpe_ratio(rfr))),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Sortino Ratio"),
            Cell::new(format!("{:0.3}", self.sortino_ratio(rfr))),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Profit Factor"),
            Cell::new(format!("{:0.3}", self.profit_factor())),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Open P/L"),
            Cell::new(f_price(self.open_profit)),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Total Closed Trades"),
            Cell::new(self.closed_trades.len().to_string()),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Number Winning Trades"),
            Cell::new(self.winning_trades_count().to_string()),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Number Losing Trades"),
            Cell::new(self.losing_trades_count().to_string()),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("% Profitable"),
            Cell::new(f_percent(self.win_rate() * 100.0)),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Avg Trade"),
            Cell::new(f_price(self.avg_trade())),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Avg Winning Trade"),
            Cell::new(f_price(self.avg_winning_trade())),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Avg Losing Trade"),
            Cell::new(f_price(self.avg_losing_trade())),
        ]));

        table.add_row(Row::from(vec![
            Cell::new("Ratio Avg Win / Avg Loss"),
            Cell::new(f_raw(self.avg_win_loss_ratio())),
        ]));

        // Print the table
        println!("{}", table);
    }

    #[cfg(feature = "pretty_table")]
    pub fn display(&self, plot: Option<(u32, u32)>) {
        self.print_table();
        let sym = self.ctx.borrow().sym().clone();
        let plot = plot.unwrap_or((120_u32, 60_u32));
        let f_price = with_suffix(&format!(" {}", sym._currency()));
        let f_percent = with_suffix("%");
        let f = |price: f64, percent: f64| format!("{} {}", f_price(price), f_percent(percent));
        let f_raw = |value: f64| format!("{:0.2}", value);

        let value_cell = |text: &str, theme: i32| {
            let mut cell = Cell::new(text)
                .set_alignment(CellAlignment::Left)
                .add_attribute(Attribute::Bold);

            cell = cell.fg(Color::White);

            match theme {
                1 => cell = cell.fg(Color::Green),
                -1 => cell = cell.fg(Color::Red),
                _ => {}
            }

            cell
        };

        let mut table = ComfyTable::new();
        table
            .load_preset(UTF8_FULL_CONDENSED)
            .apply_modifier(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::DynamicFullWidth);

        // Add a header row
        table.add_row(Row::from(vec![
            Cell::new("Net Profit").add_attribute(Attribute::Bold),
            Cell::new("Total Closed Trades").add_attribute(Attribute::Bold),
            Cell::new("% Profitable").add_attribute(Attribute::Bold),
            Cell::new("Profit Factor").add_attribute(Attribute::Bold),
            // Cell::new("Max Drawdown").add_attribute(Attribute::Bold),
            Cell::new("Avg Trade").add_attribute(Attribute::Bold),
        ]));

        table.add_row(vec![
            value_cell(
                &f(self.net_profit, self.net_profit_pct() * 100.0),
                match self.net_profit {
                    x if x > 0.0 => 1,
                    x if x < 0.0 => -1,
                    _ => 0,
                },
            ),
            value_cell(&self.closed_trades.len().to_string(), 0),
            value_cell(
                &f_percent(self.win_rate() * 100.0),
                match self.win_rate() {
                    x if x > 0.5 => 1,
                    x if x < 0.5 => -1,
                    _ => 0,
                },
            ),
            value_cell(
                &format!("{:0.3}", self.profit_factor()),
                match self.profit_factor() {
                    x if x > 1.0 => 1,
                    x if x < 1.0 => -1,
                    _ => 0,
                },
            ),
            // value_cell(
            //     &f(self.max_drawdown, self.max_drawdown_percent * 100.0),
            //     match self.max_drawdown {
            //         x if x < 0.2 => 1,
            //         x if x > 0.2 => -1,
            //         _ => 0,
            //     },
            // ),
            value_cell(&f_price(self.avg_trade()), 0),
        ]);

        println!("{}", table);
        let net_equity_line: Vec<(f32, f32)> = self
            .net_equity_list()
            .iter()
            .enumerate()
            .map(|(i, &value)| (i as f32 + 1.0, value as f32))
            .collect();
        // let (w, h) = plot;
        // Chart::new(w, h, 1.0, self.net_equity_list().len() as f32)
        //     .lineplot(&Shape::Lines(&net_equity_line))
        //     .nice();
        // auto width
        Chart::default()
            .lineplot(&Shape::Lines(&net_equity_line))
            .display();
    }
}

pub struct BacktestBarDump {
    pub bar_index: usize,
    pub equity: f64,
    pub net_equity: f64,
    pub position_size: f64,
    pub open_profit: f64,
    pub net_profit: f64,
    pub gross_profit: f64,
    pub gross_loss: f64,
    pub open_trades: usize,
    pub closed_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub trades: Vec<Trade>,
}

impl Default for BacktestBarDump {
    fn default() -> Self {
        Self {
            bar_index: 0,
            equity: f64::NAN,
            net_equity: f64::NAN,
            position_size: f64::NAN,
            open_profit: f64::NAN,
            net_profit: f64::NAN,
            gross_profit: f64::NAN,
            gross_loss: f64::NAN,
            open_trades: 0,
            closed_trades: 0,
            winning_trades: 0,
            losing_trades: 0,
            trades: vec![],
        }
    }
}

impl BacktestBarDump {
    pub fn dump(bt: &Backtest) -> Self {
        let mut dump = BacktestBarDump::default();
        dump.bar_index = bt.ctx.borrow().bar_index();
        dump.equity = bt.equity();
        dump.net_equity = bt.net_equity();
        dump.position_size = bt.position_size();
        dump.open_profit = bt.open_profit();
        dump.net_profit = bt.net_profit();
        dump.gross_profit = bt.gross_profit();
        dump.gross_loss = bt.gross_loss();
        dump.open_trades = bt.open_trades().len();
        dump.closed_trades = bt.closed_trades().len();
        dump.winning_trades = bt.winning_trades_count();
        dump.losing_trades = bt.losing_trades_count();
        dump.trades = bt.trades().into_iter().cloned().collect();
        return dump;
    }

    // pub fn assert_length(actual: &[BacktestBarDump], expected: &Backtest, len: usize) {
    //     assert_eq!(self.equity.len(), len, "equity");
    //     assert_eq!(self.net_equity.len(), len, "net_equity");
    //     assert_eq!(self.position_size.len(), len, "position_size");
    //     assert_eq!(self.open_profit.len(), len, "open_profit");
    //     assert_eq!(self.net_profit.len(), len, "net_profit");
    //     assert_eq!(self.gross_profit.len(), len, "gross_profit");
    //     assert_eq!(self.gross_loss.len(), len, "gross_loss");
    //     assert_eq!(self.winning_trades.len(), len, "winning_trades");
    //     assert_eq!(self.losing_trades.len(), len, "losing_trades");
    //     assert_eq!(self.trades.len(), len, "trades");
    // }
    // pub fn assert_compare(&mut self, other: &BacktestDebugDump) {
    //     self.assert_length(other.equity.len());
    //     for i in 0..self.equity.len() {
    //         self.assert_compare_at(other, i);
    //     }
    // }

    pub fn assert_compare(&self, expected: &BacktestBarDump) {
        let idx = self.bar_index;
        let price_eps = 1.0;
        let position_size_eps = 0.000001;
        let debug_msg_prefix = format!("[{}] ", idx);
        assert_eq!(
            self.bar_index, expected.bar_index,
            "{}bar_index: {:?} != {:?}",
            debug_msg_prefix, self.bar_index, expected.bar_index
        );
        assert!(
            self.equity
                .compare_with_precision(expected.equity, price_eps),
            "{:?}equity: {:?} != {:?}",
            debug_msg_prefix,
            self.equity,
            expected.equity
        );
        assert!(
            self.net_equity
                .compare_with_precision(expected.net_equity, price_eps),
            "{}net_equity: {:?} != {:?}",
            debug_msg_prefix,
            self.net_equity,
            expected.net_equity
        );
        assert!(
            self.position_size
                .compare_with_precision(expected.position_size, position_size_eps),
            "{}position_size: {:?} != {:?}",
            debug_msg_prefix,
            self.position_size,
            expected.position_size
        );
        assert!(
            self.open_profit
                .compare_with_precision(expected.open_profit, price_eps),
            "{}open_profit: {:?} != {:?}",
            debug_msg_prefix,
            self.open_profit,
            expected.open_profit
        );
        assert!(
            self.net_profit
                .compare_with_precision(expected.net_profit, price_eps),
            "{}net_profit: {:?} != {:?}",
            debug_msg_prefix,
            self.net_profit,
            expected.net_profit
        );
        assert!(
            self.gross_profit
                .compare_with_precision(expected.gross_profit, price_eps),
            "{}gross_profit: {:?} != {:?}",
            debug_msg_prefix,
            self.gross_profit,
            expected.gross_profit
        );
        assert!(
            self.gross_loss
                .compare_with_precision(expected.gross_loss, price_eps),
            "{}gross_loss: {:?} != {:?}",
            debug_msg_prefix,
            self.gross_loss,
            expected.gross_loss
        );
        assert_eq!(
            self.open_trades, expected.open_trades,
            "{}open_trades: {:?} != {:?}",
            debug_msg_prefix, self.open_trades, expected.open_trades
        );
        assert_eq!(
            self.closed_trades, expected.closed_trades,
            "{}closed_trades: {:?} != {:?}",
            debug_msg_prefix, self.closed_trades, expected.closed_trades
        );
        assert_eq!(
            self.winning_trades, expected.winning_trades,
            "{}winning_trades: {:?} != {:?}",
            debug_msg_prefix, self.winning_trades, expected.winning_trades
        );
        assert_eq!(
            self.losing_trades, expected.losing_trades,
            "{}losing_trades: {:?} != {:?}",
            debug_msg_prefix, self.losing_trades, expected.losing_trades
        );
        assert_eq!(
            self.trades.len(),
            expected.trades.len(),
            "{}trades: {:?} != {:?}",
            debug_msg_prefix,
            self.trades.len(),
            expected.trades.len()
        );
        assert_eq!(
            self.open_trades + self.closed_trades,
            expected.trades.len(),
            "{}open_trades + closed_trades: {:?} + {:?} != {:?}",
            debug_msg_prefix,
            self.open_trades,
            self.closed_trades,
            expected.trades.len()
        );
        let actual_trades = &self.trades;
        let expected_trades = &expected.trades;
        for expected_trade in expected_trades.iter() {
            let actual_trade = actual_trades.iter().find(|t| {
                if t.entry().is_none() || expected_trade.entry().is_none() {
                    return false;
                }
                if t.exit().is_none() || expected_trade.exit().is_none() {
                    return false;
                }
                return t.entry().unwrap().id() == expected_trade.entry().unwrap().id()
                    && t.exit().unwrap().id() == expected_trade.exit().unwrap().id();
            });
            assert!(
                actual_trade.is_some(),
                "{}{:?} trade not found",
                debug_msg_prefix,
                expected_trade,
            );
            let actual_trade = actual_trade.unwrap();
            Self::assert_compare_trade(actual_trade, expected_trade, &debug_msg_prefix);
        }
    }

    fn assert_compare_trade(actual: &Trade, expected: &Trade, msg_prefix: &str) {
        let msg_suffix = &format!(
            " | actual: {:?}, expected: {:?}",
            actual.entry(),
            expected.entry()
        );
        Self::assert_compare_trade_event(
            actual.entry(),
            expected.entry(),
            true,
            msg_prefix,
            msg_suffix,
        );
        Self::assert_compare_trade_event(
            actual.exit(),
            expected.exit(),
            false,
            msg_prefix,
            msg_suffix,
        );
        assert_eq!(
            actual.direction(),
            expected.direction(),
            "{}direction: {:?} != {:?}{}",
            msg_prefix,
            actual.direction(),
            expected.direction(),
            msg_suffix
        );
        assert_eq!(
            actual.is_closed(),
            expected.is_closed(),
            "{}is_closed: {:?} != {:?}{}",
            msg_prefix,
            actual.is_closed(),
            expected.is_closed(),
            msg_suffix
        );
        assert!(
            actual
                .size()
                .compare_with_precision(expected.size(), 0.000001),
            "{}size: {:?} != {:?}{}",
            msg_prefix,
            actual.size(),
            expected.size(),
            msg_suffix
        );
        assert!(
            actual.pnl().compare_with_precision(expected.pnl(), 1.0),
            "{}pnl: {:?} != {:?}{}",
            msg_prefix,
            actual.pnl(),
            expected.pnl(),
            msg_suffix
        );
    }

    fn assert_compare_trade_event(
        actual: Option<&TradeEvent>,
        expected: Option<&TradeEvent>,
        is_entry: bool,
        msg_prefix: &str,
        msg_suffix: &str,
    ) {
        let msg_prefix = format!(
            "{}{}: ",
            msg_prefix,
            if is_entry { "entry" } else { "exit" }
        );
        assert_eq!(
            actual.map(|x| x.fill_bar_index()),
            expected.map(|x| x.fill_bar_index()),
            "{}.fill_bar_index: {:?} != {:?}{}",
            msg_prefix,
            actual.map(|x| x.fill_bar_index()),
            expected.map(|x| x.fill_bar_index()),
            msg_suffix
        );
        assert_eq!(
            actual.map(|x| x.order_bar_index()),
            expected.map(|x| x.order_bar_index()),
            "{}.order_bar_index: {:?} != {:?}{}",
            msg_prefix,
            actual.map(|x| x.order_bar_index()),
            expected.map(|x| x.order_bar_index()),
            msg_suffix
        );
        assert!(
            actual
                .map(|x| x.price())
                .unwrap_or(f64::NAN)
                .compare_with_precision(expected.map(|x| x.price()).unwrap_or(f64::NAN), 1.0),
            "{}.price: {:?} != {:?}{}",
            msg_prefix,
            actual.map(|x| x.price()),
            expected.map(|x| x.price()),
            msg_suffix
        );
        assert_eq!(
            actual.map(|x| x.id()),
            expected.map(|x| x.id()),
            "{}.id: {:?} != {:?}{}",
            msg_prefix,
            actual.map(|x| x.id()),
            expected.map(|x| x.id()),
            msg_suffix
        );
    }
}

// #[derive(Debug, Clone)]
// pub struct BacktestSummary {
//     pub initial_capital: f64,
//     pub equity: f64,
//     pub net_equity: f64,
//     pub open_profit: f64,
//     pub net_profit: f64,
//     pub net_profit_pct: f64,
//     pub gross_profit: f64,
//     pub gross_profit_pct: f64,
//     pub gross_loss: f64,
//     pub gross_loss_pct: f64,
//     pub winning_trades: usize,
//     pub losing_trades: usize,
//     pub position_size: f64,
//     pub open_trades: usize,
//     pub closed_trades: usize,
//     pub open_longs: usize,
//     pub open_shorts: usize,
//     pub closed_longs: usize,
//     pub closed_shorts: usize,
//     pub total_longs: usize,
//     pub total_shorts: usize,
//     pub total_trades: usize,
//     pub win_rate: f64,
//     pub profit_factor: f64,
//     pub avg_trade: f64,
//     pub avg_winning_trade: f64,
//     pub avg_losing_trade: f64,
//     pub avg_win_loss_ratio: f64,
//     pub equity_list: Vec<f64>,
//     pub net_equity_list: Vec<f64>,
//     pub trades: Vec<Trade>,
//     pub sym: Sym,
//     pub sharpe_ratio: f64,
//     pub sortino_ratio: f64,
//     pub risk_free_rate: f64,
// }

// pub struct BacktestSummaryConfig {
//     pub risk_free_rate: f64,
// }

// impl Default for BacktestSummaryConfig {
//     fn default() -> Self {
//         Self {
//             risk_free_rate: 0.0,
//         }
//     }
// }

// impl BacktestSummary {
//     pub fn generate(bt: &Backtest, config: &BacktestSummaryConfig) -> Self {
//         let risk_free_rate = config.risk_free_rate;
//         let initial_capital = bt.initial_capital();
//         let equity = bt.equity();
//         let net_equity = bt.net_equity();
//         let open_profit = bt.open_profit();
//         let net_profit = bt.net_profit();
//         let net_profit_pct = net_profit_pct(net_profit, initial_capital);
//         let gross_profit = bt.gross_profit();
//         let gross_profit_pct = gross_profit_pct(gross_profit, initial_capital);
//         let gross_loss = bt.gross_loss();
//         let gross_loss_pct = gross_loss_pct(gross_loss, initial_capital);
//         let winning_trades = bt.winning_trades();
//         let losing_trades = bt.losing_trades();
//         let position_size = bt.position_size();
//         let open_longs = bt.open_longs();
//         let open_shorts = bt.open_shorts();
//         let closed_longs = bt.closed_longs();
//         let closed_shorts = bt.closed_shorts();
//         let open_trades = bt.open_trades().len();
//         let closed_trades = bt.closed_trades().len();
//         let total_longs = open_longs + closed_longs;
//         let total_shorts = open_shorts + closed_shorts;
//         let total_trades = open_trades + closed_trades;
//         let win_rate = win_rate(winning_trades, total_trades);
//         let profit_factor = profit_factor(gross_profit, gross_loss);
//         let avg_trade = avg_trade(net_profit, closed_trades);
//         let avg_winning_trade = avg_winning_trade(gross_profit, winning_trades);
//         let avg_losing_trade = avg_losing_trade(gross_loss, losing_trades);
//         let avg_win_loss_ratio = avg_win_loss_ratio(avg_winning_trade, avg_losing_trade);
//         let equity_list: Vec<f64> = bt.equity_list().to_vec();
//         let net_equity_list: Vec<f64> = bt.net_equity_list().to_vec();
//         let returns = returns(&equity_list, true);
//         let sharpe_ratio = sharpe_ratio_from_returns(&returns, risk_free_rate);
//         let sortino_ratio = sortino_ratio_from_returns(&returns, risk_free_rate);
//         Self {
//             initial_capital,
//             equity,
//             net_equity,
//             open_profit,
//             net_profit,
//             net_profit_pct,
//             gross_profit,
//             gross_profit_pct,
//             gross_loss,
//             gross_loss_pct,
//             winning_trades,
//             losing_trades,
//             position_size,
//             open_longs,
//             open_shorts,
//             closed_trades,
//             open_trades,
//             closed_longs,
//             closed_shorts,
//             total_longs,
//             total_shorts,
//             total_trades,
//             win_rate,
//             profit_factor,
//             avg_trade,
//             avg_winning_trade,
//             avg_losing_trade,
//             avg_win_loss_ratio,
//             equity_list,
//             net_equity_list,
//             trades: bt.trades().into_iter().cloned().collect(),
//             sym: bt.ctx.borrow().sym().clone(),
//             sharpe_ratio,
//             sortino_ratio,
//             risk_free_rate,
//         }
//     }

//     #[cfg(feature = "pretty_table")]
//     fn print_table(&self) {
//         let f_price = with_suffix(&format!(" {}", self.sym._currency()));
//         let f_percent = with_suffix("%");
//         let f = |price: f64, percent: f64| format!("{}\n{}", f_price(price), f_percent(percent));
//         let f_raw = |value: f64| format!("{:0.2}", value);

//         let mut table = ComfyTable::new();
//         table
//             .set_content_arrangement(ContentArrangement::DynamicFullWidth)
//             .set_header(vec!["Metric", "Value"]);

//         table.add_row(Row::from(vec![
//             Cell::new("Net Profit"),
//             Cell::new(f(self.net_profit, self.net_profit_pct * 100.0)),
//         ]));

//         table.add_row(Row::from(vec![
//             Cell::new("Gross Profit"),
//             Cell::new(f(self.gross_profit, self.gross_profit_pct * 100.0)),
//         ]));

//         table.add_row(Row::from(vec![
//             Cell::new("Gross Loss"),
//             Cell::new(f(self.gross_loss, self.gross_loss_pct * 100.0)),
//         ]));

//         table.add_row(Row::from(vec![
//             Cell::new("Sharpe Ratio"),
//             Cell::new(format!("{:0.3}", self.sharpe_ratio)),
//         ]));

//         table.add_row(Row::from(vec![
//             Cell::new("Sortino Ratio"),
//             Cell::new(format!("{:0.3}", self.sortino_ratio)),
//         ]));

//         table.add_row(Row::from(vec![
//             Cell::new("Profit Factor"),
//             Cell::new(format!("{:0.3}", self.profit_factor)),
//         ]));

//         table.add_row(Row::from(vec![
//             Cell::new("Open P/L"),
//             Cell::new(f_price(self.open_profit)),
//         ]));

//         table.add_row(Row::from(vec![
//             Cell::new("Total Closed Trades"),
//             Cell::new(self.closed_trades.to_string()),
//         ]));

//         table.add_row(Row::from(vec![
//             Cell::new("Number Winning Trades"),
//             Cell::new(self.winning_trades.to_string()),
//         ]));

//         table.add_row(Row::from(vec![
//             Cell::new("Number Losing Trades"),
//             Cell::new(self.losing_trades.to_string()),
//         ]));

//         table.add_row(Row::from(vec![
//             Cell::new("% Profitable"),
//             Cell::new(f_percent(self.win_rate * 100.0)),
//         ]));

//         table.add_row(Row::from(vec![
//             Cell::new("Avg Trade"),
//             Cell::new(f_price(self.avg_trade)),
//         ]));

//         table.add_row(Row::from(vec![
//             Cell::new("Avg Winning Trade"),
//             Cell::new(f_price(self.avg_winning_trade)),
//         ]));

//         table.add_row(Row::from(vec![
//             Cell::new("Avg Losing Trade"),
//             Cell::new(f_price(self.avg_losing_trade)),
//         ]));

//         table.add_row(Row::from(vec![
//             Cell::new("Ratio Avg Win / Avg Loss"),
//             Cell::new(f_raw(self.avg_win_loss_ratio)),
//         ]));

//         // Print the table
//         println!("{}", table);
//     }

//     #[cfg(feature = "pretty_table")]
//     pub fn display(&self, plot: Option<(u32, u32)>) {
//         self.print_table();
//         let plot = plot.unwrap_or((120_u32, 60_u32));
//         let f_price = with_suffix(&format!(" {}", self.sym._currency()));
//         let f_percent = with_suffix("%");
//         let f = |price: f64, percent: f64| format!("{} {}", f_price(price), f_percent(percent));
//         let f_raw = |value: f64| format!("{:0.2}", value);

//         let value_cell = |text: &str, theme: i32| {
//             let mut cell = Cell::new(text)
//                 .set_alignment(CellAlignment::Left)
//                 .add_attribute(Attribute::Bold);

//             cell = cell.fg(Color::White);

//             match theme {
//                 1 => cell = cell.fg(Color::Green),
//                 -1 => cell = cell.fg(Color::Red),
//                 _ => {}
//             }

//             cell
//         };

//         let mut table = ComfyTable::new();
//         table
//             .load_preset(UTF8_FULL_CONDENSED)
//             .apply_modifier(UTF8_FULL)
//             .set_content_arrangement(ContentArrangement::DynamicFullWidth);

//         // Add a header row
//         table.add_row(Row::from(vec![
//             Cell::new("Net Profit").add_attribute(Attribute::Bold),
//             Cell::new("Total Closed Trades").add_attribute(Attribute::Bold),
//             Cell::new("% Profitable").add_attribute(Attribute::Bold),
//             Cell::new("Profit Factor").add_attribute(Attribute::Bold),
//             // Cell::new("Max Drawdown").add_attribute(Attribute::Bold),
//             Cell::new("Avg Trade").add_attribute(Attribute::Bold),
//         ]));

//         table.add_row(vec![
//             value_cell(
//                 &f(self.net_profit, self.net_profit_pct * 100.0),
//                 match self.net_profit {
//                     x if x > 0.0 => 1,
//                     x if x < 0.0 => -1,
//                     _ => 0,
//                 },
//             ),
//             value_cell(&self.closed_trades.to_string(), 0),
//             value_cell(
//                 &f_percent(self.win_rate * 100.0),
//                 match self.win_rate {
//                     x if x > 0.5 => 1,
//                     x if x < 0.5 => -1,
//                     _ => 0,
//                 },
//             ),
//             value_cell(
//                 &format!("{:0.3}", self.profit_factor),
//                 match self.profit_factor {
//                     x if x > 1.0 => 1,
//                     x if x < 1.0 => -1,
//                     _ => 0,
//                 },
//             ),
//             // value_cell(
//             //     &f(self.max_drawdown, self.max_drawdown_percent * 100.0),
//             //     match self.max_drawdown {
//             //         x if x < 0.2 => 1,
//             //         x if x > 0.2 => -1,
//             //         _ => 0,
//             //     },
//             // ),
//             value_cell(&f_price(self.avg_trade), 0),
//         ]);

//         println!("{}", table);
//         let net_equity_line: Vec<(f32, f32)> = self
//             .net_equity_list
//             .iter()
//             .enumerate()
//             .map(|(i, &value)| (i as f32 + 1.0, value as f32))
//             .collect();
//         let (w, h) = plot;
//         Chart::new(w, h, 1.0, self.net_equity_list.len() as f32)
//             .lineplot(&Shape::Lines(&net_equity_line))
//             .nice();
//     }
// }
