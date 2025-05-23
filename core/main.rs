use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::rc::Rc;

use qpace_core::timeframe::Timeframe;

fn main() {
    let raw = r#""15m""#; // <-- 15 minutes
    let tf: Timeframe = serde_json::from_str(raw).unwrap();
    println!("{tf:?}");
}
