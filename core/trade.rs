cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
}}
cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}
use thiserror::Error;

use crate::utils::pnl;

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass_enum)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "TradeDirection", eq, eq_int))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "TradeDirection"))]
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum TradeDirection {
    Long = 1,
    Short = -1,
}

impl From<f64> for TradeDirection {
    #[inline]
    fn from(value: f64) -> Self {
        if value >= 0.0 {
            return TradeDirection::Long;
        }
        return TradeDirection::Short;
    }
}

impl Into<f64> for TradeDirection {
    #[inline]
    fn into(self) -> f64 {
        match &self {
            TradeDirection::Long => 1.0,
            TradeDirection::Short => -1.0,
        }
    }
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "TradeEvent"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "TradeEvent"))]
#[derive(Debug, Clone, PartialEq)]
pub struct TradeEvent {
    id: Option<String>,
    order_bar_index: usize,
    fill_bar_index: usize,
    price: f64,
    comment: Option<String>,
}

impl Default for TradeEvent {
    fn default() -> Self {
        Self {
            id: None,
            order_bar_index: 0,
            fill_bar_index: 0,
            price: f64::NAN,
            comment: None,
        }
    }
}

impl TradeEvent {
    #[inline]
    pub fn new(
        id: Option<String>,
        order_bar_index: usize,
        fill_bar_index: usize,
        price: f64,
        comment: Option<String>,
    ) -> Self {
        Self {
            id,
            order_bar_index,
            fill_bar_index,
            price,
            comment,
        }
    }

    #[inline]
    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    #[inline]
    pub fn order_bar_index(&self) -> usize {
        self.order_bar_index
    }

    #[inline]
    pub fn fill_bar_index(&self) -> usize {
        self.fill_bar_index
    }

    #[inline]
    pub fn price(&self) -> f64 {
        self.price
    }

    #[inline]
    pub fn comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    #[inline]
    pub fn set_id(&mut self, id: Option<String>) -> &mut Self {
        self.id = id;
        return self;
    }

    #[inline]
    pub fn set_order_bar_index(&mut self, order_bar_index: usize) -> &mut Self {
        self.order_bar_index = order_bar_index;
        return self;
    }

    #[inline]
    pub fn set_fill_bar_index(&mut self, fill_bar_index: usize) -> &mut Self {
        self.fill_bar_index = fill_bar_index;
        return self;
    }

    #[inline]
    pub fn set_price(&mut self, price: f64) -> &mut Self {
        self.price = price;
        return self;
    }
}

#[derive(Debug, Error, Clone)]
pub enum TradeError {
    #[error("Trade has not been entered")]
    NotEntered,
    #[error("Trade is not active")]
    NotActive,
    #[error("Trade has been closed")]
    Closed,
    #[error("Trade has already been entered")]
    AlreadyEntered,
    #[error("Trade has already been closed")]
    AlreadyClosed,
    #[error("Trade size is invalid. Got {0}")]
    InvalidTradeSize(f64),
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "Trade"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "Trade"))]
#[derive(Debug, Clone, PartialEq)]
pub struct Trade {
    size: f64,
    entry: Option<TradeEvent>,
    exit: Option<TradeEvent>,
    pnl: f64,
}

impl Default for Trade {
    fn default() -> Self {
        Self {
            size: f64::NAN,
            entry: None,
            exit: None,
            pnl: 0.0,
        }
    }
}

impl Trade {
    pub fn new() -> Self {
        Self {
            size: f64::NAN,
            entry: None,
            exit: None,
            pnl: f64::NAN,
        }
    }

    #[inline]
    pub fn size(&self) -> f64 {
        self.size
    }

    #[inline]
    pub fn entry(&self) -> Option<&TradeEvent> {
        self.entry.as_ref()
    }

    #[inline]
    pub fn exit(&self) -> Option<&TradeEvent> {
        self.exit.as_ref()
    }

    #[inline]
    pub fn pnl(&self) -> f64 {
        self.pnl
    }

    #[inline]
    pub fn direction(&self) -> TradeDirection {
        return TradeDirection::from(self.size);
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        return self.entry.is_some() && self.exit.is_none();
    }

    #[inline]
    pub fn is_closed(&self) -> bool {
        return self.exit.is_some();
    }

    #[inline]
    pub fn set_size(&mut self, size: f64) -> Result<(), TradeError> {
        if size.is_nan() {
            return Err(TradeError::InvalidTradeSize(size));
        }
        self.size = size;
        return Ok(());
    }

    #[inline]
    pub fn set_entry(&mut self, entry: TradeEvent) -> Result<(), TradeError> {
        if self.entry.is_some() {
            return Err(TradeError::AlreadyEntered);
        }
        if self.exit.is_some() {
            return Err(TradeError::Closed);
        }
        self.entry = Some(entry);
        return Ok(());
    }

    #[inline]
    pub fn set_exit(&mut self, exit: TradeEvent) -> Result<(), TradeError> {
        if self.entry.is_none() {
            return Err(TradeError::NotEntered);
        }
        if self.exit.is_some() {
            return Err(TradeError::AlreadyClosed);
        }
        self.exit = Some(exit);
        return Ok(());
    }

    #[inline]
    pub fn set_pnl(&mut self, pnl: f64) -> Result<(), TradeError> {
        if !self.is_active() {
            return Err(TradeError::NotActive);
        }
        self.pnl = pnl;
        return Ok(());
    }

    #[inline]
    pub fn set_pnl_from_price(&mut self, price: f64) -> Result<(), TradeError> {
        let pnl = pnl(
            self.size,
            self.entry.as_ref().map(|x| x.price).unwrap_or(f64::NAN),
            price,
        );
        return self.set_pnl(pnl);
    }
}
