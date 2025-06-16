use crate::trade::{Trade, TradeDirection, TradeEvent};
use napi_derive::napi;

#[napi]
#[derive(Debug, Clone)]
pub struct NodeTradeDirection {
    inner: TradeDirection,
}

impl Into<NodeTradeDirection> for TradeDirection {
    #[inline]
    fn into(self) -> NodeTradeDirection {
        NodeTradeDirection { inner: self }
    }
}

impl Into<TradeDirection> for NodeTradeDirection {
    #[inline]
    fn into(self) -> TradeDirection {
        self.inner
    }
}

#[napi]
impl NodeTradeDirection {
    #[napi(js_name = "Long")]
    #[inline]
    pub fn node_long(&self) -> Self {
        TradeDirection::Long.into()
    }

    #[napi(js_name = "Short")]
    #[inline]
    pub fn node_short(&self) -> Self {
        TradeDirection::Short.into()
    }

    #[napi(js_name = "toNumber")]
    #[inline]
    pub fn node_to_number(&self) -> f64 {
        self.inner.into()
    }

    #[napi(js_name = "fromNumber")]
    #[inline]
    pub fn node_from_number(value: f64) -> Self {
        TradeDirection::from(value).into()
    }
}

#[napi]
#[derive(Debug, Clone)]
pub struct NodeTradeEvent {
    inner: TradeEvent,
}

impl Into<NodeTradeEvent> for TradeEvent {
    #[inline]
    fn into(self) -> NodeTradeEvent {
        NodeTradeEvent { inner: self }
    }
}

impl Into<TradeEvent> for NodeTradeEvent {
    #[inline]
    fn into(self) -> TradeEvent {
        self.inner
    }
}

#[napi]
#[derive(Debug, Clone)]
pub struct NodeTrade {
    inner: Trade,
}

impl Into<NodeTrade> for Trade {
    #[inline]
    fn into(self) -> NodeTrade {
        NodeTrade { inner: self }
    }
}

impl Into<Trade> for NodeTrade {
    #[inline]
    fn into(self) -> Trade {
        self.inner
    }
}

#[napi]
impl NodeTradeEvent {
    #[napi(getter = id)]
    #[inline]
    pub fn node_id(&self) -> Option<String> {
        self.inner.id().cloned()
    }

    #[napi(getter = orderBarIndex)]
    #[inline]
    pub fn node_order_bar_index(&self) -> i32 {
        self.inner.order_bar_index() as i32
    }

    #[napi(getter = fillBarIndex)]
    #[inline]
    pub fn node_fill_bar_index(&self) -> i32 {
        self.inner.fill_bar_index() as i32
    }

    #[napi(getter = price)]
    #[inline]
    pub fn node_price(&self) -> f64 {
        self.inner.price()
    }

    #[napi(getter = comment)]
    #[inline]
    pub fn node_comment(&self) -> Option<String> {
        self.inner.comment().cloned()
    }
}

#[napi]
impl NodeTrade {
    #[napi(getter = size)]
    #[inline]
    pub fn node_size(&self) -> f64 {
        self.inner.size()
    }

    #[napi(getter = entry)]
    #[inline]
    pub fn node_entry(&self) -> Option<NodeTradeEvent> {
        self.inner.entry().cloned().map(|x| x.into())
    }

    #[napi(getter = exit)]
    #[inline]
    pub fn node_exit(&self) -> Option<NodeTradeEvent> {
        self.inner.exit().cloned().map(|x| x.into())
    }

    #[napi(getter = pnl)]
    #[inline]
    pub fn node_pnl(&self) -> f64 {
        self.inner.pnl()
    }

    #[napi(getter = direction)]
    #[inline]
    pub fn node_direction(&self) -> NodeTradeDirection {
        self.inner.direction().into()
    }

    #[napi(getter = isActive)]
    #[inline]
    pub fn node_is_active(&self) -> bool {
        self.inner.is_active()
    }

    #[napi(getter = isClosed)]
    #[inline]
    pub fn node_is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}
