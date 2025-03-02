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

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "SymInfo"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "SymInfo"))]
#[derive(Debug, Clone, Copy)]
pub struct SymInfo {
    min_tick: f64,
    min_qty: f64,
    timeframe: Timeframe,
}

impl Default for SymInfo {
    #[inline]
    fn default() -> Self {
        Self {
            min_tick: f64::NAN,
            min_qty: f64::NAN,
            timeframe: Timeframe::Unknown(),
        }
    }
}

impl SymInfo {
    #[inline]
    pub fn timeframe(&self) -> &Timeframe {
        &self.timeframe
    }

    #[inline]
    pub fn min_tick(&self) -> f64 {
        self.min_tick
    }

    #[inline]
    pub fn min_qty(&self) -> f64 {
        self.min_qty
    }

    #[inline]
    pub fn set_timeframe(&mut self, timeframe: Timeframe) -> &mut Self {
        self.timeframe = timeframe;
        self
    }

    #[inline]
    pub fn set_min_tick(&mut self, min_tick: f64) -> &mut Self {
        self.min_tick = min_tick;
        self
    }

    #[inline]
    pub fn set_min_qty(&mut self, min_qty: f64) -> &mut Self {
        self.min_qty = min_qty;
        self
    }

    #[inline]
    pub fn round_to_min_tick(&self, value: f64) -> f64 {
        round_to_min_tick(value, self.min_tick)
    }

    #[inline]
    pub fn validate_contracts(&self, size: f64) -> bool {
        return validate_contracts(size, self.min_qty);
    }

    #[inline]
    pub fn round_contracts(&self, size: f64) -> f64 {
        return round_contracts(size, self.min_qty);
    }

    #[inline]
    pub fn btc_usd() -> Self {
        Self {
            min_tick: 1.0,
            min_qty: 0.000001,
            ..Default::default()
        }
    }

    #[inline]
    pub fn eth_usd() -> Self {
        Self {
            min_tick: 0.1,
            min_qty: 0.0001,
            ..Default::default()
        }
    }

    #[inline]
    pub fn sol_usd() -> Self {
        Self {
            min_tick: 0.01,
            min_qty: 0.0001,
            ..Default::default()
        }
    }
}
