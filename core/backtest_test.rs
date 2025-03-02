#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
#[cfg(test)]
#[cfg(feature = "polars")]
mod test {
    use std::{
        any::Any,
        cell::RefCell,
        fs,
        path::{Path, PathBuf},
        rc::Rc,
    };

    use polars::prelude::DataFrame;
    use serde::{Deserialize, Serialize};

    use crate::{
        backtest::{Backtest, BacktestBarDump, BacktestConfig},
        ctx::Ctx,
        ohlcv::{ArcOhlcv, Ohlcv, OhlcvReader},
        orderbook::OrderConfig,
        rs_utils::{read_df, Float64Utils, OptionFloatUtils, SeriesCastUtils},
        sym::SymInfo,
        test_utils::format_fixture_path,
        trade::{Trade, TradeDirection, TradeEvent},
    };

    fn format_path(path: &str) -> PathBuf {
        format_fixture_path(&format!("backtest/{}", path))
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct BacktestFixtureConfigSymInfo {
        pub min_tick: f64,
        pub min_qty: f64,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct BacktestFixtureSignal {
        pub __disabled: Option<bool>,
        pub bar_index: usize,
        pub kind: String,
        pub id: String,
        pub direction: String,
        pub qty: Option<f64>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct BacktestFixtureTrade {
        pub __disabled: Option<bool>,
        pub entry_bar_index: Option<usize>,
        pub entry_price: Option<f64>,
        pub entry_id: Option<String>,
        pub exit_bar_index: Option<usize>,
        pub exit_price: Option<f64>,
        pub exit_id: Option<String>,
        pub profit: Option<f64>,
        pub size: Option<f64>,
        pub direction: Option<String>,
        pub closed: bool,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct BacktestFixtureConfig {
        pub initial_capital: f64,
        pub process_orders_on_close: bool,
        pub price: Option<Vec<f64>>,
        pub fixture: Option<String>,
        pub price_precision: Option<usize>,
        pub sym_info: Option<BacktestFixtureConfigSymInfo>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct BacktestFixtureTimeline {
        pub equity: Option<Vec<f64>>,
        pub position_size: Option<Vec<f64>>,
        pub open_profit: Option<Vec<f64>>,
        pub net_profit: Option<Vec<f64>>,
        pub open_trades: Option<Vec<f64>>,
        pub closed_trades: Option<Vec<f64>>,
        pub gross_loss: Option<Vec<f64>>,
        pub gross_profit: Option<Vec<f64>>,
        pub winning_trades: Option<Vec<f64>>,
        pub losing_trades: Option<Vec<f64>>,
    }

    impl BacktestFixtureTimeline {
        pub fn default() -> Self {
            Self {
                equity: None,
                position_size: None,
                open_profit: None,
                net_profit: None,
                open_trades: None,
                closed_trades: None,
                gross_loss: None,
                gross_profit: None,
                winning_trades: None,
                losing_trades: None,
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct BacktestFixture {
        pub config: BacktestFixtureConfig,
        pub signals: Vec<BacktestFixtureSignal>,
        pub trades: Vec<BacktestFixtureTrade>,
        pub timeline: Option<BacktestFixtureTimeline>,
    }

    pub struct BacktestFixtureTarget {
        pub fixture: BacktestFixture,
        pub ctx: Rc<RefCell<Ctx>>,
        pub strategy: Rc<RefCell<Backtest>>,
    }

    pub fn load_fixture(path: &Path) -> BacktestFixtureTarget {
        let data = fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&data).unwrap();
        let mut backtest_fixture: BacktestFixture = serde_json::from_value(json).unwrap();

        let mut ctx: Option<Ctx> = None;
        let mut df: Option<DataFrame> = None;
        if let Some(fixture_path) = &backtest_fixture.config.fixture {
            let _df = read_df(&format_path(fixture_path));

            let dp: ArcOhlcv = Ohlcv::from_polars(&_df).into();

            println!("{:?}", backtest_fixture.config.sym_info);

            let mut sym_info = SymInfo::default();

            if let Some(sym_info_config) = &backtest_fixture.config.sym_info {
                sym_info
                    .set_min_tick(sym_info_config.min_tick)
                    .set_min_qty(sym_info_config.min_qty);
            }

            let _ctx = Ctx::new(dp.into_box(), sym_info);

            backtest_fixture.timeline = Some(BacktestFixtureTimeline {
                equity: Some(_df.column("equity").unwrap().to_f64()),
                position_size: Some(_df.column("position_size").unwrap().to_f64()),
                open_profit: Some(_df.column("open_profit").unwrap().to_f64()),
                net_profit: Some(_df.column("net_profit").unwrap().to_f64()),
                open_trades: Some(_df.column("open_trades").unwrap().to_f64()),
                closed_trades: Some(_df.column("closed_trades").unwrap().to_f64()),
                gross_loss: Some(_df.column("gross_loss").unwrap().to_f64()),
                gross_profit: Some(_df.column("gross_profit").unwrap().to_f64()),
                winning_trades: Some(_df.column("winning_trades").unwrap().to_f64()),
                losing_trades: Some(_df.column("losing_trades").unwrap().to_f64()),
                ..BacktestFixtureTimeline::default()
            });

            df = Some(_df);
            ctx = Some(_ctx);
        } else {
            let price = backtest_fixture.config.price.clone().unwrap();

            ctx = Some(Ctx::new(
                Ohlcv::from_uniform_price(price).into_box(),
                SymInfo::default(),
            ));
        }
        let ctx = Rc::new(RefCell::new(ctx.unwrap()));

        let strategy_config = BacktestConfig::new(
            backtest_fixture.config.initial_capital,
            backtest_fixture.config.process_orders_on_close,
        );

        let strategy = Rc::new(RefCell::new(Backtest::new(ctx.clone(), strategy_config)));

        return BacktestFixtureTarget {
            fixture: backtest_fixture,
            ctx,
            strategy,
        };
    }

    impl Into<Trade> for BacktestFixtureTrade {
        fn into(self) -> Trade {
            let mut t = Trade::default();
            if self.entry_bar_index.is_some() {
                let mut e = TradeEvent::default();
                e.set_id(self.entry_id.clone());
                e.set_order_bar_index(self.entry_bar_index.unwrap() - 1);
                e.set_fill_bar_index(self.entry_bar_index.unwrap());
                e.set_price(self.entry_price.unwrap());
                t.set_entry(e).unwrap();
            }
            t.set_size(self.size.unwrap()).unwrap();
            t.set_pnl(self.profit.unwrap()).unwrap();
            if self.exit_bar_index.is_some() {
                let mut e = TradeEvent::default();
                e.set_id(self.exit_id.clone());
                e.set_order_bar_index(self.exit_bar_index.unwrap() - 1);
                e.set_fill_bar_index(self.exit_bar_index.unwrap());
                e.set_price(self.exit_price.unwrap());
                t.set_exit(e).unwrap();
            }
            return t;
        }
    }

    impl BacktestFixtureTarget {
        pub fn expected_bar_dumps(&self) -> Vec<BacktestBarDump> {
            let mut dumps: Vec<BacktestBarDump> = vec![];
            for bar_index in 0..self.ctx.borrow().len() {
                let mut dump = BacktestBarDump::default();
                dump.bar_index = bar_index;
                if self.fixture.timeline.is_some() {
                    let t = self.fixture.timeline.as_ref().unwrap();
                    dump.equity = t.equity.as_ref().unwrap()[bar_index];
                    dump.position_size = t.position_size.as_ref().unwrap()[bar_index];
                    dump.open_profit = t.open_profit.as_ref().unwrap()[bar_index];
                    dump.net_profit = t.net_profit.as_ref().unwrap()[bar_index];
                    dump.open_trades = t.open_trades.as_ref().unwrap()[bar_index] as usize;
                    dump.closed_trades = t.closed_trades.as_ref().unwrap()[bar_index] as usize;
                    dump.gross_loss = t.gross_loss.as_ref().unwrap()[bar_index];
                    dump.gross_profit = t.gross_profit.as_ref().unwrap()[bar_index];
                    dump.winning_trades = t.winning_trades.as_ref().unwrap()[bar_index] as usize;
                    dump.losing_trades = t.losing_trades.as_ref().unwrap()[bar_index] as usize;
                }

                for fixture_trade in &self.fixture.trades {
                    dump.trades.push(fixture_trade.clone().into());
                }
                dumps.push(dump);
            }
            return dumps;
        }
    }

    pub fn run(t: &BacktestFixtureTarget) {
        let mut actual: Vec<BacktestBarDump> = vec![];
        loop {
            let bar_index = t.ctx.borrow_mut().next();
            if bar_index.is_none() {
                break;
            }
            let bar_index = bar_index.unwrap();
            t.strategy.borrow_mut().on_bar_open();

            let signal_fixture = t
                .fixture
                .signals
                .iter()
                .find(|s| s.bar_index == bar_index && !s.__disabled.unwrap_or(false));

            if let Some(signal_fixture) = signal_fixture {
                let direction = match signal_fixture.direction.as_str() {
                    "long" => TradeDirection::Long,
                    "short" => TradeDirection::Short,
                    _ => panic!("Unknown direction {}", signal_fixture.direction),
                };

                let direction_f64: f64 = direction.into();
                let order: OrderConfig = match signal_fixture.kind.as_str() {
                    // "entry" => Signal::Entry(config),
                    // "close" => Signal::Close(config),
                    "order" => OrderConfig::new(
                        signal_fixture.qty.unwrap_or(1.0).abs() * direction_f64,
                        Some(signal_fixture.id.clone()),
                    ),
                    _ => panic!("Unknown signal kind {}", signal_fixture.kind),
                };

                t.strategy.borrow_mut().order(order).unwrap();
                t.strategy.borrow_mut().on_bar_close();
            }

            actual.push(t.strategy.borrow().dump_bar());
        }
        let expected = t.expected_bar_dumps();
        assert_eq!(
            actual.len(),
            expected.len(),
            "{:?} != {:?}",
            actual.len(),
            expected.len()
        );
        for (a, e) in actual.iter().zip(expected.iter()) {
            a.assert_compare(e);
        }
    }

    // #[test]
    // pub fn simple() {
    //     Backtest::new(BacktestFixture::load(&format_path("simple.json"))).run();
    // }

    // #[test]
    // pub fn tv_entry_only_duplicates() {
    //     Backtest::new(BacktestFixture::load(&format_path(
    //         "tv_entry_only_duplicates.json",
    //     )))
    //     .run();
    // }

    // #[test]
    // pub fn tv_entry_close_duplicates() {
    //     Backtest::new(BacktestFixture::load(&format_path(
    //         "tv_entry_close_duplicates.json",
    //     )))
    //     .run();
    // }

    #[test]
    pub fn tv_order_contracts() {
        run(&load_fixture(&format_path("tv_order_contracts.json")));
    }
}
