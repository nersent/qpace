cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
}}
cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass_enum)]
#[cfg_attr(feature = "bindings_py", pyclass)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SignalKind {
    Size(f64),
    EquityPct(f64),
    Hold(),
    CloseAll(),
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass)]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen)]
#[derive(Debug, Clone, PartialEq)]
pub struct Signal {
    kind: SignalKind,
    id: Option<String>,
}

impl Default for Signal {
    fn default() -> Self {
        Self {
            kind: SignalKind::Hold(),
            id: None,
        }
    }
}

impl Signal {
    #[inline]
    pub fn kind(&self) -> &SignalKind {
        &self.kind
    }

    #[inline]
    pub fn id(&self) -> &Option<String> {
        &self.id
    }

    #[inline]
    pub fn set_id(&mut self, id: Option<String>) -> &mut Self {
        self.id = id;
        self
    }

    #[inline]
    pub fn hold() -> Self {
        Self {
            kind: SignalKind::Hold(),
            ..Default::default()
        }
    }

    #[inline]
    pub fn size(size: f64) -> Self {
        Self {
            kind: SignalKind::Size(size),
            ..Default::default()
        }
    }

    #[inline]
    pub fn equity_pct(equity_pct: f64) -> Self {
        Self {
            kind: SignalKind::EquityPct(equity_pct),
            ..Default::default()
        }
    }

    #[inline]
    pub fn close_all() -> Self {
        Self {
            kind: SignalKind::CloseAll(),
            ..Default::default()
        }
    }

    #[inline]
    pub fn long() -> Self {
        Self::equity_pct(1.0)
    }

    #[inline]
    pub fn short() -> Self {
        Self::equity_pct(-1.0)
    }
}
