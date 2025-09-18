use crate::signal::Signal;
use napi::bindgen_prelude::*;
use napi::{Error, Result, Status};
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
    #[napi(js_name = toString)]
    pub fn node_to_string(&self) -> String {
        format!("{:?}", self.inner)
    }

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

    #[napi(getter = equityPct)]
    #[inline]
    pub fn node_get_equity_pct(&self) -> Option<f64> {
        self.inner.get_equity_pct()
    }

    #[napi(getter = size)]
    #[inline]
    pub fn node_get_size(&self) -> Option<f64> {
        self.inner.get_size()
    }

    #[napi(getter = isHold)]
    #[inline]
    pub fn node_is_hold(&self) -> bool {
        self.inner.is_hold()
    }

    #[napi(getter = isCloseAll)]
    #[inline]
    pub fn node_is_close_all(&self) -> bool {
        self.inner.is_close_all()
    }

    #[cfg(feature = "json")]
    #[napi(js_name = toJSON)]
    pub fn node_to_json(&self, env: Env) -> Result<Unknown> {
        env.to_js_value(&self.inner)
    }

    #[cfg(feature = "json")]
    #[napi(js_name = fromJSON)]
    pub fn node_from_json(env: Env, json: Unknown) -> Result<NodeSignal> {
        let sig: Signal = env.from_js_value(json)?;
        Ok(sig.into())
    }
}
