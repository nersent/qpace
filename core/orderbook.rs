use crate::trade::TradeDirection;
use core::f64;
use std::collections::{HashMap, VecDeque};
use thiserror::Error;

#[inline]
pub fn round_to_min_tick(value: f64, min_tick: f64) -> f64 {
    if value.is_nan() {
        return 0.0;
    }
    // @TODO: assert
    if min_tick.is_nan() {
        return value;
    }
    return (value / min_tick).round() * min_tick;
}

#[inline]
pub fn round_contracts(size: f64, price_scale: f64) -> f64 {
    if size.is_nan() {
        return 0.0;
    }
    // @TODO: assert
    if price_scale.is_nan() || price_scale == 0.0 {
        return size;
    }
    return ((size * price_scale) + f64::EPSILON).round() / price_scale;
}

#[inline]
pub fn validate_contracts(size: f64, min_qty: f64) -> bool {
    return min_qty.is_nan() || !size.is_nan() && size.abs() >= min_qty;
}

#[inline]
pub fn order_size(
    equity_pct: f64,
    equity: f64,
    exchange_rate: f64,
    instrument_price: f64,
    point_value: f64,
) -> f64 {
    return (equity_pct * equity * exchange_rate) / (instrument_price * point_value);
}

#[inline]
pub fn order_size_for_equity_pct(
    equity_pct: f64,
    equity: f64,
    current_position: f64,
    instrument_price: f64,
    point_value: f64,
    exchange_rate: f64,
) -> f64 {
    let equity_order_size = order_size(
        equity_pct.abs(),
        equity,
        exchange_rate,
        instrument_price,
        point_value,
    );
    let sign = equity_pct.signum();
    let order_size = equity_order_size * sign - current_position;
    return order_size;
}

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
    pub price_scale: f64,
    pub debug: bool,
}

impl Default for OrderBookConfig {
    fn default() -> Self {
        return Self {
            min_size: 0.000001,
            price_scale: 1000000.0,
            debug: false,
        };
    }
}

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
            size: round_contracts(order_opts.size, self.config.price_scale),
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
