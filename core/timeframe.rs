use crate::utils::{round_contracts, round_to_min_tick, validate_contracts};

cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
}}
cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass_enum)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "Timeframe"))]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Timeframe {
    Years(usize),
    Months(usize),
    Weeks(usize),
    Days(usize),
    Hours(usize),
    Minutes(usize),
    Seconds(usize),
    Ticks(usize),
    Ranges(usize),
    Unknown(),
}

impl Default for Timeframe {
    #[inline]
    fn default() -> Self {
        Timeframe::Unknown()
    }
}

impl Into<String> for Timeframe {
    #[inline]
    fn into(self) -> String {
        return match self {
            Timeframe::Years(value) => format!("{}Y", value),
            Timeframe::Months(value) => format!("{}M", value),
            Timeframe::Weeks(value) => format!("{}W", value),
            Timeframe::Days(value) => format!("{}D", value),
            Timeframe::Hours(value) => format!("{}h", value),
            Timeframe::Minutes(value) => format!("{}m", value),
            Timeframe::Seconds(value) => format!("{}s", value),
            Timeframe::Ticks(value) => format!("{}T", value),
            Timeframe::Ranges(value) => format!("{}R", value),
            Timeframe::Unknown() => String::from("UNKNOWN"),
        };
    }
}

impl From<String> for Timeframe {
    #[inline]
    fn from(value: String) -> Self {
        let mut chars = value.chars();
        let mut num = String::new();
        let mut unit = String::new();
        while let Some(c) = chars.next() {
            if c.is_numeric() {
                num.push(c);
            } else {
                unit.push(c);
            }
        }
        let num = num.parse::<usize>().unwrap();
        return match unit.as_str() {
            "Y" => Timeframe::Years(num),
            "M" => Timeframe::Months(num),
            "W" => Timeframe::Weeks(num),
            "D" => Timeframe::Days(num),
            "h" => Timeframe::Hours(num),
            "m" => Timeframe::Minutes(num),
            "s" => Timeframe::Seconds(num),
            "T" => Timeframe::Ticks(num),
            "R" => Timeframe::Ranges(num),
            _ => Timeframe::Unknown(),
        };
    }
}
