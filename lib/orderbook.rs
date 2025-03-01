cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
}}
cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}
use crate::{
    trade::TradeDirection,
    utils::{round_contracts, validate_contracts},
};
use core::f64;
use std::collections::{HashMap, VecDeque};
use thiserror::Error;

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "OrderConfig"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "OrderConfig"))]
#[derive(Debug, Clone)]
pub struct OrderConfig {
    size: f64,
    tag: Option<String>,
}

impl Default for OrderConfig {
    fn default() -> Self {
        Self {
            size: f64::NAN,
            tag: None,
        }
    }
}

impl OrderConfig {
    pub fn new(size: f64, tag: Option<String>) -> Self {
        Self { size, tag }
    }
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "Order"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "Order"))]
#[derive(Debug, Clone)]
pub struct Order {
    id: usize,
    size: f64,
    tag: Option<String>,
}

impl Order {
    #[inline]
    pub fn id(&self) -> usize {
        return self.id;
    }

    #[inline]
    pub fn size(&self) -> f64 {
        return self.size;
    }

    #[inline]
    pub fn tag(&self) -> &Option<String> {
        return &self.tag;
    }

    #[inline]
    pub fn direction(&self) -> TradeDirection {
        return TradeDirection::from(self.size);
    }
}

#[derive(Debug, Clone)]
pub struct OrderBookConfig {
    pub min_size: f64,
    pub debug: bool,
}

impl Default for OrderBookConfig {
    fn default() -> Self {
        return Self {
            min_size: 0.000001,
            debug: false,
        };
    }
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "OrderBook"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "OrderBook"))]
pub struct OrderBook {
    config: OrderBookConfig,
    orders: HashMap<usize, Order>,
    queue: VecDeque<usize>,
    id_counter: usize,
    price: f64,
}

#[derive(Debug, Error)]
pub enum OrderBookError {
    #[error("Invalid qty: {0}")]
    InvalidQty(f64),
    #[error("IgnoredSize: {0}")]
    IgnoredSize(f64),
}

impl OrderBook {
    pub fn new(config: OrderBookConfig) -> Self {
        Self {
            config,
            queue: VecDeque::new(),
            orders: HashMap::new(),
            id_counter: 0,
            price: f64::NAN,
        }
    }

    #[inline]
    pub fn price(&self) -> f64 {
        self.price
    }

    #[inline]
    pub fn set_price(&mut self, price: f64) {
        self.price = price;
    }

    // #[inline]
    // pub fn validate_contracts(&self, size: f64) -> bool {
    //     return validate_contracts(size, self.config.min_size);
    // }

    // #[inline]
    // pub fn round_contracts(&self, size: f64) -> f64 {
    //     return round_contracts(size, self.config.min_size);
    // }

    #[inline]
    fn create_id(&mut self) -> usize {
        let id = self.id_counter;
        self.id_counter += 1;
        return id;
    }

    #[inline]
    pub fn enqueue(&mut self, order_opts: OrderConfig) -> Result<usize, OrderBookError> {
        // if !self.validate_contracts(order_opts.size) {
        //     return Err(OrderBookError::IgnoredSize(order_opts.size));
        // }
        let id = self.create_id();
        let order = Order {
            id,
            size: round_contracts(order_opts.size, self.config.min_size),
            tag: order_opts.tag,
        };
        self.orders.insert(id, order);
        self.queue.push_back(id);
        return Ok(id);
    }

    #[inline]
    pub fn pop_front(&mut self) -> Option<usize> {
        return self.queue.pop_front();
    }

    #[inline]
    pub fn get_order(&self, id: usize) -> Option<&Order> {
        return self.orders.get(&id);
    }
}
