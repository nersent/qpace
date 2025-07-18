#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SignalKind {
    Size(f64),
    EquityPct(f64),
    Hold(),
    CloseAll(),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Signal {
    kind: SignalKind,
    id: Option<String>,
    comment: Option<String>,
}

impl Default for Signal {
    fn default() -> Self {
        Self {
            kind: SignalKind::Hold(),
            id: None,
            comment: None,
        }
    }
}

impl Signal {
    #[inline]
    pub fn kind(&self) -> &SignalKind {
        &self.kind
    }

    #[inline]
    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    #[inline]
    pub fn set_id(&mut self, id: Option<String>) -> &mut Self {
        self.id = id;
        self
    }

    #[inline]
    pub fn comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    #[inline]
    pub fn set_comment(&mut self, comment: Option<String>) -> &mut Self {
        self.comment = comment;
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

#[derive(Debug, Clone, PartialEq)]
pub struct SymSignal {
    sym_id: String,
    signal: Signal,
}

impl SymSignal {
    #[inline]
    pub fn new(sym_id: String, signal: Signal) -> Self {
        Self { sym_id, signal }
    }
}
