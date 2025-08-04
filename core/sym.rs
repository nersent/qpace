#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(try_from = "String", into = "String"))]
pub enum SymKind {
    Stock,
    Future,
    Option,
    Forex,
    Crypto,
    Unknown,
    Other(String),
}

impl SymKind {
    pub fn trading_days(&self) -> f64 {
        match self {
            SymKind::Crypto => 365.0,
            _ => 252.0,
        }
    }
}

impl Default for SymKind {
    #[inline]
    fn default() -> Self {
        SymKind::Unknown
    }
}

impl Into<String> for &SymKind {
    #[inline]
    fn into(self) -> String {
        match self {
            SymKind::Stock => "stock".to_string(),
            SymKind::Future => "future".to_string(),
            SymKind::Option => "option".to_string(),
            SymKind::Forex => "forex".to_string(),
            SymKind::Crypto => "crypto".to_string(),
            SymKind::Unknown => "unknown".to_string(),
            SymKind::Other(s) => s.clone(),
        }
    }
}

impl Into<String> for SymKind {
    #[inline]
    fn into(self) -> String {
        (&self).into()
    }
}

impl From<String> for SymKind {
    #[inline]
    fn from(value: String) -> Self {
        match value.as_str() {
            "stock" => SymKind::Stock,
            "future" => SymKind::Future,
            "option" => SymKind::Option,
            "forex" => SymKind::Forex,
            "crypto" => SymKind::Crypto,
            "unknown" => SymKind::Unknown,
            _ => SymKind::Other(value),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct Sym {
    id: Option<String>,
    ticker_id: Option<String>,
    kind: SymKind,
    min_tick: f64,
    min_qty: f64,
    prefix: Option<String>,
    currency: Option<String>,
    base_currency: Option<String>,
    ticker: Option<String>,
    country: Option<String>,
    price_scale: f64,
    point_value: f64,
    metadata: Option<String>,
}

impl PartialEq for Sym {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.id.is_none() || other.id.is_none() {
            return false;
        }
        self.id == other.id
    }
}

impl Default for Sym {
    #[inline]
    fn default() -> Self {
        Self {
            id: None,
            ticker_id: None,
            kind: SymKind::default(),
            min_tick: f64::NAN,
            min_qty: f64::NAN,
            prefix: None,
            currency: None,
            base_currency: None,
            ticker: None,
            country: None,
            price_scale: f64::NAN,
            point_value: f64::NAN,
            metadata: None,
        }
    }
}

impl Into<String> for &Sym {
    #[inline]
    fn into(self) -> String {
        format!(
            "Sym(ticker_id={:?}, id={:?}, kind={:?})",
            self.ticker_id, self.id, self.kind
        )
    }
}

impl Sym {
    #[inline]
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    #[inline]
    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }

    #[inline]
    pub fn ticker_id(&self) -> Option<&str> {
        self.ticker_id.as_deref()
    }

    #[inline]
    pub fn set_ticker_id(&mut self, ticker_id: Option<String>) {
        self.ticker_id = ticker_id;
    }

    #[inline]
    pub fn kind(&self) -> &SymKind {
        &self.kind
    }

    #[inline]
    pub fn set_kind(&mut self, kind: SymKind) {
        self.kind = kind;
    }

    #[inline]
    pub fn min_tick(&self) -> f64 {
        self.min_tick
    }

    #[inline]
    pub fn set_min_tick(&mut self, min_tick: f64) {
        self.min_tick = min_tick;
    }

    #[inline]
    pub fn min_qty(&self) -> f64 {
        self.min_qty
    }

    #[inline]
    pub fn set_min_qty(&mut self, min_qty: f64) {
        self.min_qty = min_qty;
    }

    #[inline]
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    #[inline]
    pub fn set_prefix(&mut self, prefix: Option<String>) {
        self.prefix = prefix;
    }

    #[inline]
    pub fn currency(&self) -> Option<&str> {
        self.currency.as_deref()
    }

    #[inline]
    pub fn _currency(&self) -> String {
        self.currency().unwrap_or("?").to_string()
    }

    #[inline]
    pub fn set_currency(&mut self, currency: Option<String>) {
        self.currency = currency;
    }

    #[inline]
    pub fn base_currency(&self) -> Option<&str> {
        self.base_currency.as_deref()
    }

    #[inline]
    pub fn set_base_currency(&mut self, base_currency: Option<String>) {
        self.base_currency = base_currency;
    }

    #[inline]
    pub fn ticker(&self) -> Option<&str> {
        self.ticker.as_deref()
    }

    #[inline]
    pub fn set_ticker(&mut self, ticker: Option<String>) {
        self.ticker = ticker;
    }

    #[inline]
    pub fn country(&self) -> Option<&str> {
        self.country.as_deref()
    }

    #[inline]
    pub fn set_country(&mut self, country: Option<String>) {
        self.country = country;
    }

    #[inline]
    pub fn price_scale(&self) -> f64 {
        self.price_scale
    }

    #[inline]
    pub fn set_price_scale(&mut self, price_scale: f64) -> &mut Self {
        self.price_scale = price_scale;
        self
    }

    #[inline]
    pub fn point_value(&self) -> f64 {
        self.point_value
    }

    #[inline]
    pub fn set_point_value(&mut self, point_value: f64) -> &mut Self {
        self.point_value = point_value;
        self
    }

    #[inline]
    pub fn metadata(&self) -> Option<&str> {
        self.metadata.as_deref()
    }

    #[inline]
    pub fn set_metadata(&mut self, metadata: Option<String>) {
        self.metadata = metadata;
    }

    #[inline]
    pub fn qty_scale(&self) -> f64 {
        assert!(self.min_qty > 0.0, "min_qty must be greater than 0.0");
        return (1.0 / self.min_qty).round();
    }

    #[inline]
    pub fn btc_usd() -> Self {
        Self {
            id: Some("QPACE__BITSTAMP:BTCUSD".to_string()),
            ticker_id: Some("BITSTAMP:BTCUSD".to_string()),
            min_tick: 1.0,
            min_qty: 0.000001,
            prefix: Some("BITSTAMP".to_string()),
            currency: Some("USD".to_string()),
            base_currency: Some("BTC".to_string()),
            ticker: Some("BTCUSD".to_string()),
            kind: SymKind::Crypto,
            price_scale: 1.0,
            point_value: 1.0,
            ..Default::default()
        }
    }

    #[inline]
    pub fn eth_usd() -> Self {
        Self {
            id: Some("QPACE__BITSTAMP:ETHUSD".to_string()),
            ticker_id: Some("BITSTAMP:ETHUSD".to_string()),
            min_tick: 0.1,
            min_qty: 0.0001,
            prefix: Some("BITSTAMP".to_string()),
            currency: Some("USD".to_string()),
            base_currency: Some("ETH".to_string()),
            ticker: Some("ETHUSD".to_string()),
            kind: SymKind::Crypto,
            price_scale: 10.0,
            point_value: 1.0,
            ..Default::default()
        }
    }

    #[inline]
    pub fn sol_usd() -> Self {
        Self {
            id: Some("QPACE__BINANCE:SOLUSD".to_string()),
            ticker_id: Some("BINANCE:SOLUSD".to_string()),
            min_tick: 0.01,
            min_qty: 0.001,
            prefix: Some("BINANCE".to_string()),
            currency: Some("USD".to_string()),
            base_currency: Some("SOL".to_string()),
            ticker: Some("SOLUSD".to_string()),
            kind: SymKind::Crypto,
            price_scale: 100.0,
            point_value: 1.0,
            ..Default::default()
        }
    }

    #[inline]
    pub fn doge_usd() -> Self {
        Self {
            id: Some("QPACE__BINANCE:DOGEUSD".to_string()),
            ticker_id: Some("BINANCE:DOGEUSD".to_string()),
            min_tick: 0.0001,
            min_qty: 0.001,
            prefix: Some("BINANCE".to_string()),
            currency: Some("USD".to_string()),
            base_currency: Some("DOGE".to_string()),
            ticker: Some("DOGEUSD".to_string()),
            kind: SymKind::Crypto,
            price_scale: 10000.0,
            point_value: 1.0,
            ..Default::default()
        }
    }
}
