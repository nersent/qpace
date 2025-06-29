use crate::signal::Signal;
use napi_derive::napi;

#[napi]
#[derive(Debug, Clone, PartialEq)]
pub struct NodeSignal {
    inner: Signal,
}

impl Into<Signal> for NodeSignal {
    #[inline]
    fn into(self) -> Signal {
        self.inner
    }
}

impl From<Signal> for NodeSignal {
    #[inline]
    fn from(inner: Signal) -> Self {
        NodeSignal { inner }
    }
}

#[napi]
impl NodeSignal {
    #[napi(js_name = Hold)]
    #[inline]
    pub fn node_hold() -> Self {
        Signal::hold().into()
    }

    #[napi(js_name = Size)]
    #[inline]
    pub fn node_size(size: f64) -> Self {
        Signal::size(size).into()
    }

    #[napi(js_name = EquityPct)]
    #[inline]
    pub fn node_equity_pct(equity_pct: f64) -> Self {
        Signal::equity_pct(equity_pct).into()
    }

    #[napi(js_name = CloseAll)]
    #[inline]
    pub fn node_close_all() -> Self {
        Signal::close_all().into()
    }

    #[napi(js_name = Long)]
    #[inline]
    pub fn node_long() -> Self {
        Signal::long().into()
    }

    #[napi(js_name = Short)]
    #[inline]
    pub fn node_short() -> Self {
        Signal::short().into()
    }

    #[napi(getter = id)]
    #[inline]
    pub fn node_id(&self) -> Option<String> {
        self.inner.id().cloned()
    }

    #[napi(setter = id)]
    #[inline]
    pub fn node_set_id(&mut self, id: Option<String>) {
        self.inner.set_id(id);
    }

    #[napi(getter = comment)]
    #[inline]
    pub fn node_comment(&self) -> Option<String> {
        self.inner.comment().cloned()
    }

    #[napi(setter = comment)]
    #[inline]
    pub fn node_set_comment(&mut self, comment: Option<String>) {
        self.inner.set_comment(comment);
    }
}
