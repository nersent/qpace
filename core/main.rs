use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::rc::Rc;

use qpace_core::timeframe::Timeframe;

fn main() {
    let mut _2_src: f64 = 11.70000;
    let mut _3_ma: f64 = 11.70000;
    let mut _4_dev = f64::NAN;
    _4_dev = 0.0;
    let mut _5_cci = (_2_src - _3_ma) / (0.015 * _4_dev);

    println!("CCI: {}", _5_cci);
}
