use crate::ohlcv::{
    zip_ohlcv_bars, ArcOhlcv, Ohlcv, OhlcvBar, OhlcvReader, OhlcvReaderOps, OhlcvWriter,
    OhlcvWriterOps,
};
use crate::timeframe::Timeframe;
use crate::timeframe_node::NodeTimeframe;
use crate::utils::get_oldest_possible_datetime;
use chrono::{DateTime, Duration, Utc};
use napi::bindgen_prelude::*;
use napi::{Error, Result, Status};
use napi_derive::napi;
use std::path::Path;

#[napi]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeOhlcvBar {
    inner: OhlcvBar,
}

impl Into<OhlcvBar> for NodeOhlcvBar {
    fn into(self) -> OhlcvBar {
        self.inner
    }
}

impl From<OhlcvBar> for NodeOhlcvBar {
    fn from(bar: OhlcvBar) -> Self {
        Self { inner: bar }
    }
}

#[napi]
impl NodeOhlcvBar {
    #[napi(constructor)]
    pub fn new(
        open_time: Option<DateTime<Utc>>,
        close_time: Option<DateTime<Utc>>,
        open: Option<f64>,
        high: Option<f64>,
        low: Option<f64>,
        close: Option<f64>,
        volume: Option<f64>,
    ) -> Self {
        OhlcvBar::new(
            open_time,
            close_time,
            open.unwrap_or(f64::NAN),
            high.unwrap_or(f64::NAN),
            low.unwrap_or(f64::NAN),
            close.unwrap_or(f64::NAN),
            volume.unwrap_or(f64::NAN),
        )
        .into()
    }

    #[napi(getter = openTime)]
    #[inline]
    pub fn node_open_time(&self) -> Option<DateTime<Utc>> {
        self.inner.open_time().copied()
    }

    #[napi(setter = openTime)]
    pub fn node_set_open_time(&mut self, open_time: Option<DateTime<Utc>>) {
        self.inner.set_open_time(open_time);
    }

    #[napi(getter = closeTime)]
    #[inline]
    pub fn node_close_time(&self) -> Option<DateTime<Utc>> {
        self.inner.close_time().copied()
    }

    #[napi(setter = closeTime)]
    pub fn node_set_close_time(&mut self, close_time: Option<DateTime<Utc>>) {
        self.inner.set_close_time(close_time);
    }

    #[napi(getter = open)]
    #[inline]
    pub fn node_open(&self) -> f64 {
        self.inner.open()
    }

    #[napi(setter = open)]
    pub fn node_set_open(&mut self, open: f64) {
        self.inner.set_open(open);
    }

    #[napi(getter = high)]
    #[inline]
    pub fn node_high(&self) -> f64 {
        self.inner.high()
    }

    #[napi(setter = high)]
    pub fn node_set_high(&mut self, high: f64) {
        self.inner.set_high(high);
    }

    #[napi(getter = low)]
    #[inline]
    pub fn node_low(&self) -> f64 {
        self.inner.low()
    }

    #[napi(setter = low)]
    pub fn node_set_low(&mut self, low: f64) {
        self.inner.set_low(low);
    }

    #[napi(getter = close)]
    #[inline]
    pub fn node_close(&self) -> f64 {
        self.inner.close()
    }

    #[napi(setter = close)]
    pub fn node_set_close(&mut self, close: f64) {
        self.inner.set_close(close);
    }

    #[napi(getter = volume)]
    #[inline]
    pub fn node_volume(&self) -> f64 {
        self.inner.volume()
    }

    #[napi(setter = volume)]
    pub fn node_set_volume(&mut self, volume: f64) {
        self.inner.set_volume(volume);
    }

    #[napi(js_name = merge)]
    #[inline]
    pub fn node_merge(&self, other: &NodeOhlcvBar) -> Self {
        self.inner.merge(&other.inner).into()
    }

    #[napi(js_name = toString)]
    #[inline]
    pub fn node_to_string(&self) -> String {
        format!("{:?}", self.inner)
    }

    #[napi(js_name = toJSON)]
    #[inline]
    pub fn node_to_json(&self, env: Env) -> Result<Object> {
        let mut js_obj = Object::new(&env)?;
        if let Some(dt) = self.node_open_time() {
            js_obj.set("open_time", dt.to_rfc3339())?;
        } else {
            js_obj.set("open_time", Null)?;
        }
        if let Some(dt) = self.node_close_time() {
            js_obj.set("close_time", dt.to_rfc3339())?;
        } else {
            js_obj.set("close_time", Null)?;
        }
        js_obj.set("open", self.node_open())?;
        js_obj.set("high", self.node_high())?;
        js_obj.set("low", self.node_low())?;
        js_obj.set("close", self.node_close())?;
        js_obj.set("volume", self.node_volume())?;
        Ok(js_obj)
    }

    #[napi(js_name = fromJSON)]
    pub fn node_from_json(json: Object) -> Result<NodeOhlcvBar> {
        fn parse_dt(opt: Option<String>) -> Result<Option<DateTime<Utc>>> {
            if let Some(s) = opt {
                let dt = DateTime::parse_from_rfc3339(&s)
                    .map_err(|e| Error::new(Status::InvalidArg, format!("bad date: {}", e)))?
                    .with_timezone(&Utc);
                Ok(Some(dt))
            } else {
                Ok(None)
            }
        }

        let open_time_s: Option<String> = json.get("open_time")?;
        let close_time_s: Option<String> = json.get("close_time")?;
        let open: Option<f64> = json.get("open")?;
        let high: Option<f64> = json.get("high")?;
        let low: Option<f64> = json.get("low")?;
        let close: Option<f64> = json.get("close")?;
        let volume: Option<f64> = json.get("volume")?;

        let open_time = parse_dt(open_time_s)?;
        let close_time = parse_dt(close_time_s)?;
        let open = open.unwrap_or(f64::NAN);
        let high = high.unwrap_or(f64::NAN);
        let low = low.unwrap_or(f64::NAN);
        let close = close.unwrap_or(f64::NAN);
        let volume = volume.unwrap_or(f64::NAN);

        let bar = OhlcvBar::new(open_time, close_time, open, high, low, close, volume);

        Ok(NodeOhlcvBar { inner: bar })
    }
}

#[napi]
#[derive(Clone, Debug)]
pub struct NodeOhlcv {
    inner: ArcOhlcv,
}

impl Default for NodeOhlcv {
    fn default() -> Self {
        ArcOhlcv::new().into()
    }
}

impl Into<NodeOhlcv> for Ohlcv {
    fn into(self) -> NodeOhlcv {
        NodeOhlcv { inner: self.into() }
    }
}

impl Into<ArcOhlcv> for &NodeOhlcv {
    fn into(self) -> ArcOhlcv {
        self.inner.clone()
    }
}

impl From<ArcOhlcv> for NodeOhlcv {
    fn from(ohlcv: ArcOhlcv) -> Self {
        Self { inner: ohlcv }
    }
}

#[napi]
impl NodeOhlcv {
    #[napi(constructor)]
    pub fn new() -> Self {
        ArcOhlcv::new().into()
    }

    #[napi(js_name = fromBars)]
    pub fn node_from_bars(bars: Vec<&NodeOhlcvBar>) -> Self {
        let bars: Vec<OhlcvBar> = bars.into_iter().map(|b| (*b).into()).collect();
        ArcOhlcv::from_bars(bars).into()
    }

    #[napi(getter = timeframe)]
    #[inline]
    pub fn node_timeframe(&self) -> NodeTimeframe {
        self.inner.timeframe().into()
    }

    #[napi(setter = timeframe)]
    pub fn node_set_timeframe(&mut self, timeframe: &NodeTimeframe) {
        self.inner.set_timeframe(timeframe.into());
    }

    #[napi(getter = openTime)]
    #[inline]
    pub fn node_open_time(&self) -> Vec<Option<DateTime<Utc>>> {
        self.inner.open_time()
    }

    #[napi(getter = closeTime)]
    #[inline]
    pub fn node_close_time(&self) -> Vec<Option<DateTime<Utc>>> {
        self.inner.close_time()
    }

    #[napi(getter = open)]
    #[inline]
    pub fn node_open(&self) -> Vec<f64> {
        self.inner.open()
    }

    #[napi(getter = high)]
    #[inline]
    pub fn node_high(&self) -> Vec<f64> {
        self.inner.high()
    }

    #[napi(getter = low)]
    #[inline]
    pub fn node_low(&self) -> Vec<f64> {
        self.inner.low()
    }

    #[napi(getter = close)]
    #[inline]
    pub fn node_close(&self) -> Vec<f64> {
        self.inner.close()
    }

    #[napi(getter = volume)]
    #[inline]
    pub fn node_volume(&self) -> Vec<f64> {
        self.inner.volume()
    }

    #[napi(getter = bars)]
    #[inline]
    pub fn node_bars(&self) -> Vec<NodeOhlcvBar> {
        self.inner.bars().iter().map(|bar| (*bar).into()).collect()
    }

    #[napi(js_name = at)]
    #[inline]
    pub fn node_at(&self, index: i32) -> Option<NodeOhlcvBar> {
        self.inner.at(index).map(|bar| bar.into())
    }

    #[napi(getter = length)]
    #[inline]
    pub fn node_length(&self) -> i32 {
        self.inner.len() as i32
    }

    #[napi(js_name = "slice")]
    #[inline]
    pub fn node_slice(&self, start: i32, end: i32) -> Self {
        let start = start as usize;
        let end = end as usize;
        let mut ohlcv = Ohlcv::from_bars(self.inner.slice(start..end));
        ohlcv.set_timeframe(self.inner.timeframe().into());
        return ohlcv.into();
    }

    #[napi(js_name = "head")]
    #[inline]
    pub fn node_head(&self, count: i32) -> Self {
        let count = count as usize;
        let mut ohlcv = Ohlcv::from_bars(self.inner.head(count));
        ohlcv.set_timeframe(self.inner.timeframe().into());
        return ohlcv.into();
    }

    #[napi(js_name = "tail")]
    #[inline]
    pub fn node_tail(&self, count: i32) -> Self {
        let count = count as usize;
        let mut ohlcv = Ohlcv::from_bars(self.inner.tail(count));
        ohlcv.set_timeframe(self.inner.timeframe().into());
        return ohlcv.into();
    }

    #[napi(js_name = "copy")]
    pub fn node_copy(&self) -> Self {
        self.inner.copy().into()
    }

    #[napi(js_name = "extend")]
    pub fn node_extend(&mut self, other: &NodeOhlcv) {
        self.inner.extend((&other.inner).into())
    }

    #[napi(js_name = "resample")]
    pub fn node_resample(&self, timeframe: &NodeTimeframe, align_utc: bool) -> Self {
        self.inner.resample(timeframe.into(), align_utc).into()
    }

    #[napi(js_name = "sort")]
    pub fn node_ascending(&mut self, ascending: bool) {
        self.inner.sort(ascending);
    }

    #[napi(js_name = "reverse")]
    pub fn node_reverse(&mut self) {
        self.inner.reverse();
    }

    #[napi(js_name = "clear")]
    pub fn node_clear(&mut self) {
        self.inner.clear();
    }

    #[napi(js_name = "pop")]
    pub fn node_pop(&mut self) -> Option<NodeOhlcvBar> {
        self.inner.pop().map(|bar| bar.into())
    }

    #[napi(js_name = "shift")]
    pub fn node_shift(&mut self) -> Option<NodeOhlcvBar> {
        self.inner.shift().map(|bar| bar.into())
    }

    #[napi(js_name = "push")]
    pub fn node_push(&mut self, bar: &NodeOhlcvBar) {
        self.inner.push(bar.inner.into());
    }

    #[napi(js_name = "pushMany")]
    pub fn node_push_many(&mut self, bars: Vec<&NodeOhlcvBar>) {
        self.inner
            .push_many(bars.into_iter().map(|b| b.inner.clone()).collect());
    }

    #[napi(js_name = "toString")]
    pub fn node_to_string(&self) -> String {
        format!("{:?}", self.inner)
    }

    #[cfg(feature = "polars")]
    #[napi(js_name = "read_csv")]
    #[inline]
    pub fn node_read_csv(path: String) -> Self {
        let mut ohlcv = Ohlcv::new();
        ohlcv.read_csv(Path::new(&path));
        let ohlcv: ArcOhlcv = ohlcv.into();
        return ohlcv.into();
    }

    #[cfg(feature = "polars")]
    #[napi(js_name = "read_parquet")]
    #[inline]
    pub fn node_read_parquet(path: String) -> Self {
        let mut ohlcv = Ohlcv::new();
        ohlcv.read_parquet(Path::new(&path));
        let ohlcv: ArcOhlcv = ohlcv.into();
        return ohlcv.into();
    }

    #[cfg(feature = "polars")]
    #[napi(js_name = "write_csv")]
    #[inline]
    pub fn node_write_csv(&self, path: String) {
        self.inner.write_csv(Path::new(&path));
    }

    #[cfg(feature = "polars")]
    #[napi(js_name = "write_parquet")]
    #[inline]
    pub fn node_write_parquet(&self, path: String) {
        self.inner.write_parquet(Path::new(&path));
    }

    #[napi(js_name = "sanityCheck")]
    pub fn node_sanity_check(&self) -> Vec<String> {
        match self.inner.sanity_check() {
            Ok(_) => vec![],
            Err(e) => e,
        }
    }
}

#[napi(js_name = "zipOhlcvBars")]
#[inline]
pub fn node_zip_ohlcv_bars(
    open_time: Option<Vec<Option<DateTime<Utc>>>>,
    close_time: Option<Vec<Option<DateTime<Utc>>>>,
    open: Option<Vec<f64>>,
    high: Option<Vec<f64>>,
    low: Option<Vec<f64>>,
    close: Option<Vec<f64>>,
    volume: Option<Vec<f64>>,
) -> Vec<NodeOhlcvBar> {
    return zip_ohlcv_bars(open_time, close_time, open, high, low, close, volume)
        .into_iter()
        .map(|bar| bar.into())
        .collect();
}
