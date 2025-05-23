use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::rc::Rc;

use qpace_core::backtest::{Backtest, BacktestConfig};
use qpace_core::ctx::Ctx;
use qpace_core::ohlcv::{Ohlcv, OhlcvReader};
use qpace_core::sym::Sym;

fn main() {
    // let workspace_root = std::env::var("BAZED_WORKSPACE_ROOT").unwrap();
    // let out_path = Path::new(&workspace_root).join("out");
    // let ohlcv_path = Path::new(&workspace_root).join("playground/btc.csv");
    // let ohlcv = Ohlcv::read_path(&ohlcv_path, "ms");
    // let ctx = Rc::new(RefCell::new({
    //     let mut ctx = Ctx::new();
    //     ctx.set_ohlcv(ohlcv.into_box());
    //     ctx.set_sym(Sym::default());
    //     ctx
    // }));
    // let bt = Rc::new(RefCell::new(Backtest::new(
    //     ctx.clone(),
    //     BacktestConfig::default(),
    // )));
    // loop {
    //     let bar_index = ctx.borrow_mut().next();
    //     if bar_index.is_none() {
    //         break;
    //     }
    //     bt.borrow_mut().on_bar_open();
    //     let ctx_borrow = ctx.borrow();
    //     let bar = ctx_borrow.bar();
    //     println!("{:?}", bar.open());
    //     bt.borrow_mut().on_bar_close();
    // }
    // while let Some(bar_index) = ctx.borrow_mut().next() {
    //     bt.borrow_mut().on_bar_open();
    //     let ctx_borrow = ctx.borrow();
    //     let bar = ctx_borrow.bar();
    //     println!("{:?}", bar.open());
    //     bt.borrow_mut().on_bar_close();
    // }
    // for bar_index in ctx.borrow_mut().into_iter() {
    //     bt.borrow_mut().on_bar_open();
    //     let ctx_borrow = ctx.borrow();
    //     let bar = ctx_borrow.bar();
    //     println!("{:?}", bar.open());
    //     bt.borrow_mut().on_bar_close();
    // }

    // let ohlcv_path =
    //     Path::new(&workspace_root).join("pace/lib/fixtures/backtest/tv_order_contracts.csv");
    // let ohlcv_loader = StaticOhlcvLoader::read_path(&ohlcv_path).to_arc();

    // let ctx = Context::new(
    //     ohlcv_loader,
    //     Some(SymInfo {
    //         min_qty: 0.000001,
    //         min_tick: 1.0,
    //         ..Default::default()
    //     }),
    // );
    // let mut bt = Backtest::new(
    //     ctx.clone(),
    //     BacktestConfig {
    //         debug: true,
    //         ..Default::default()
    //     },
    // );

    // for bar_index in ctx {
    //     bt.on_bar_open();

    //     if bar_index == 2000 {
    //         bt.order(OrderConfig {
    //             tag: Some("1_long_entry".to_string()),
    //             size: 0.5,
    //         })
    //         .unwrap();
    //     }
    //     if bar_index == 2100 {
    //         bt.order(OrderConfig {
    //             tag: Some("1_long_exit".to_string()),
    //             size: -0.5,
    //         })
    //         .unwrap();
    //     }
    //     if bar_index == 2105 {
    //         bt.order(OrderConfig {
    //             tag: Some("2_short_entry_1".to_string()),
    //             size: -0.25,
    //         })
    //         .unwrap();
    //     }
    //     if bar_index == 2110 {
    //         bt.order(OrderConfig {
    //             tag: Some("2_short_entry_2".to_string()),
    //             size: -0.75,
    //         })
    //         .unwrap();
    //     }
    //     if bar_index == 2115 {
    //         bt.order(OrderConfig {
    //             tag: Some("2_short_exit_1".to_string()),
    //             size: 0.8,
    //         })
    //         .unwrap();
    //     }
    //     if bar_index == 2125 {
    //         bt.order(OrderConfig {
    //             tag: Some("2_short_exit_2".to_string()),
    //             size: 0.2,
    //         })
    //         .unwrap();
    //     }
    //     if bar_index == 2140 {
    //         bt.order(OrderConfig {
    //             tag: Some("3_long_entry_1".to_string()),
    //             size: 0.25,
    //         })
    //         .unwrap();
    //     }
    //     if bar_index == 2180 {
    //         bt.order(OrderConfig {
    //             tag: Some("3_long_entry_2".to_string()),
    //             size: 0.25,
    //         })
    //         .unwrap();
    //     }
    //     if bar_index == 2250 {
    //         bt.order(OrderConfig {
    //             tag: Some("3_long_exit".to_string()),
    //             size: -0.5,
    //         })
    //         .unwrap();
    //     }

    //     // let x = 2250 - 5;
    //     // if bar_index > x && bar_index < x + 10 {
    //     //     println!(
    //     //         "\n\n[{:?}]: {:?}\nopen trades: {:#?} \nclosed trades: {:#?}",
    //     //         bar_index, bt.position_size, &bt.open_trades, &bt.closed_trades
    //     //     );
    //     // }
    //     // if bar_index == 0 {
    //     //     bt.order(OrderConfig {
    //     //         tag: Some("xd".to_string()),
    //     //         size: 6.9,
    //     //     })
    //     //     .unwrap();
    //     // }
    //     // if bar_index == 2 {
    //     //     bt.order(OrderConfig {
    //     //         // tag: Some("gowno".to_string()),
    //     //         size: -6.9,
    //     //         ..Default::default()
    //     //     })
    //     //     .unwrap();
    //     // }
    //     // if bar_index < 10 {
    //     //     println!(
    //     //         "[{:?}]: {:?}\nopen trades: {:?}\nclosed trades: {:?}",
    //     //         bar_index, bt.position_size, &bt.open_trades, &bt.closed_trades
    //     //     );
    //     // }
    //     bt.on_bar_close();
    // }

    // println!("---------");
    // println!("open: \n{:#?}", bt.open_trades);
    // println!("closed:\n{:#?}", bt.closed_trades);

    // // let data_path = Path::new(&workspace_root).join("out/stocks/blockchain/data/sol/tx");
    // // let entries = fs::read_dir(data_path).unwrap();

    // // for entry in entries {
    // //     let entry = entry.unwrap();
    // //     let path = entry.path();
    // //     println!("{:?}", path);
    // // }
}
