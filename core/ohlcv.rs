use chrono::{DateTime, Duration, Utc};
cfg_if::cfg_if! { if #[cfg(feature = "polars")] {
    use polars::frame::DataFrame;
    use polars::series::Series;
    use polars::error::PolarsError;
    use polars::prelude::NamedFrom;
    use crate::utils::SeriesCastUtils;
    use crate::utils::read_df_csv;
    use crate::utils::read_df_parquet;
    use crate::utils::write_df_csv;
    use crate::utils::write_df_parquet;

}}
cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass}};
  use pyo3::types::PyDict;
}}
use std::{
    any::Any,
    cell::RefCell,
    fmt,
    ops::Range,
    rc::Rc,
    sync::{Arc, RwLock},
};
cfg_if::cfg_if! {
    if #[cfg(feature = "bindings_wasm")] {
        use wasm_bindgen::prelude::*;
    }
}
use crate::timeframe::Timeframe;
use std::path::Path;

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "OhlcvBar"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = "OhlcvBar"))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OhlcvBar {
    open_time: Option<DateTime<Utc>>,
    close_time: Option<DateTime<Utc>>,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl PartialOrd for OhlcvBar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.open_time.partial_cmp(&other.open_time)
    }
}

impl fmt::Display for OhlcvBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!(
            "OhlcvBar(open_time={:?}, close_time={:?}, open={}, high={}, low={}, close={}, volume={})",
            self.open_time,
            self.close_time,
            self.open,
            self.high,
            self.low,
            self.close,
            self.volume,
        ))
    }
}

impl Default for OhlcvBar {
    fn default() -> Self {
        Self {
            open_time: None,
            close_time: None,
            open: f64::NAN,
            high: f64::NAN,
            low: f64::NAN,
            close: f64::NAN,
            volume: f64::NAN,
        }
    }
}

impl OhlcvBar {
    #[inline]
    pub fn new(
        open_time: Option<DateTime<Utc>>,
        close_time: Option<DateTime<Utc>>,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> Self {
        Self {
            open_time,
            close_time,
            open,
            high,
            low,
            close,
            volume,
        }
    }

    #[inline]
    pub fn open_time(&self) -> Option<&DateTime<Utc>> {
        self.open_time.as_ref()
    }

    #[inline]
    pub fn set_open_time(&mut self, open_time: Option<DateTime<Utc>>) {
        self.open_time = open_time;
    }

    #[inline]
    pub fn close_time(&self) -> Option<&DateTime<Utc>> {
        self.close_time.as_ref()
    }

    #[inline]
    pub fn set_close_time(&mut self, close_time: Option<DateTime<Utc>>) {
        self.close_time = close_time;
    }

    #[inline]
    pub fn open(&self) -> f64 {
        self.open
    }

    #[inline]
    pub fn set_open(&mut self, open: f64) {
        self.open = open;
    }

    #[inline]
    pub fn high(&self) -> f64 {
        self.high
    }

    #[inline]
    pub fn set_high(&mut self, high: f64) {
        self.high = high;
    }

    #[inline]
    pub fn low(&self) -> f64 {
        self.low
    }

    #[inline]
    pub fn set_low(&mut self, low: f64) {
        self.low = low;
    }

    #[inline]
    pub fn close(&self) -> f64 {
        self.close
    }

    #[inline]
    pub fn set_close(&mut self, close: f64) {
        self.close = close;
    }

    #[inline]
    pub fn volume(&self) -> f64 {
        self.volume
    }

    #[inline]
    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume;
    }

    #[inline]
    pub fn merge(&self, other: &OhlcvBar) -> OhlcvBar {
        let (open_time, open) = if self.open_time <= other.open_time {
            (self.open_time, self.open)
        } else {
            (other.open_time, other.open)
        };
        let high = self.high.max(other.high);
        let low = self.low.min(other.low);
        let (close_time, close) = if self.close_time >= other.close_time {
            (self.close_time, self.close)
        } else {
            (other.close_time, other.close)
        };
        let volume = self.volume + other.volume;
        let mut bar = OhlcvBar::default();
        bar.set_open_time(open_time);
        bar.set_close_time(close_time);
        bar.set_open(open);
        bar.set_high(high);
        bar.set_low(low);
        bar.set_close(close);
        bar.set_volume(volume);
        return bar;
    }
}

#[inline]
pub fn zip_ohlcv_bars(
    open_time: Option<Vec<Option<DateTime<Utc>>>>,
    close_time: Option<Vec<Option<DateTime<Utc>>>>,
    open: Option<Vec<f64>>,
    high: Option<Vec<f64>>,
    low: Option<Vec<f64>>,
    close: Option<Vec<f64>>,
    volume: Option<Vec<f64>>,
) -> Vec<OhlcvBar> {
    let len = open_time
        .as_ref()
        .map(|x| x.len())
        .or_else(|| close_time.as_ref().map(|x| x.len()))
        .or_else(|| open.as_ref().map(|x| x.len()))
        .or_else(|| high.as_ref().map(|x| x.len()))
        .or_else(|| low.as_ref().map(|x| x.len()))
        .or_else(|| close.as_ref().map(|x| x.len()))
        .or_else(|| volume.as_ref().map(|x| x.len()))
        .unwrap_or(0);
    assert!(open_time.is_none() || open_time.as_ref().unwrap().len() == len);
    assert!(close_time.is_none() || close_time.as_ref().unwrap().len() == len);
    assert!(open.is_none() || open.as_ref().unwrap().len() == len);
    assert!(high.is_none() || high.as_ref().unwrap().len() == len);
    assert!(low.is_none() || low.as_ref().unwrap().len() == len);
    assert!(close.is_none() || close.as_ref().unwrap().len() == len);
    assert!(volume.is_none() || volume.as_ref().unwrap().len() == len);
    let mut bars: Vec<OhlcvBar> = vec![OhlcvBar::default(); len];
    for i in 0..len {
        let open_time = open_time
            .as_ref()
            .and_then(|x| x.get(i))
            .copied()
            .unwrap_or(None);
        let close_time = close_time
            .as_ref()
            .and_then(|x| x.get(i))
            .copied()
            .unwrap_or(None);
        let open = open
            .as_ref()
            .and_then(|x| x.get(i))
            .copied()
            .unwrap_or(f64::NAN);
        let high = high
            .as_ref()
            .and_then(|x| x.get(i))
            .copied()
            .unwrap_or(f64::NAN);
        let low = low
            .as_ref()
            .and_then(|x| x.get(i))
            .copied()
            .unwrap_or(f64::NAN);
        let close = close
            .as_ref()
            .and_then(|x| x.get(i))
            .copied()
            .unwrap_or(f64::NAN);
        let volume = volume
            .as_ref()
            .and_then(|x| x.get(i))
            .copied()
            .unwrap_or(f64::NAN);
        bars[i] = OhlcvBar::new(open_time, close_time, open, high, low, close, volume);
    }
    return bars;
}

pub trait OhlcvReader: fmt::Debug {
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<OhlcvBar>;

    #[inline]
    fn at(&self, index: i32) -> Option<OhlcvBar> {
        let idx = if index < 0 {
            (self.len() as i32 + index) as usize
        } else {
            index as usize
        };
        self.get(idx)
    }

    fn slice(&self, range: Range<usize>) -> Vec<OhlcvBar>;
    fn bars(&self) -> Vec<OhlcvBar> {
        return self.slice(0..self.len());
    }
    fn into_box(self) -> Box<dyn OhlcvReader>;
    fn clone_box(&self) -> Box<dyn OhlcvReader>;
    fn as_any(&self) -> &dyn Any;

    #[inline]
    fn open_time(&self) -> Vec<Option<DateTime<Utc>>> {
        self.bars()
            .iter()
            .map(|bar| bar.open_time().copied())
            .collect()
    }

    #[inline]
    fn close_time(&self) -> Vec<Option<DateTime<Utc>>> {
        self.bars()
            .iter()
            .map(|bar| bar.close_time().copied())
            .collect()
    }

    #[inline]
    fn open(&self) -> Vec<f64> {
        self.bars().iter().map(|bar| bar.open()).collect()
    }

    #[inline]
    fn high(&self) -> Vec<f64> {
        self.bars().iter().map(|bar| bar.high()).collect()
    }

    #[inline]
    fn low(&self) -> Vec<f64> {
        self.bars().iter().map(|bar| bar.low()).collect()
    }

    #[inline]
    fn close(&self) -> Vec<f64> {
        self.bars().iter().map(|bar| bar.close()).collect()
    }

    #[inline]
    fn volume(&self) -> Vec<f64> {
        self.bars().iter().map(|bar| bar.volume()).collect()
    }

    #[cfg(feature = "polars")]
    fn to_polars(&self) -> Result<DataFrame, PolarsError> {
        let len = self.len();
        let mut open_time = Vec::with_capacity(len);
        let mut close_time = Vec::with_capacity(len);
        let mut open = Vec::with_capacity(len);
        let mut high = Vec::with_capacity(len);
        let mut low = Vec::with_capacity(len);
        let mut close = Vec::with_capacity(len);
        let mut volume = Vec::with_capacity(len);
        for i in 0..len {
            let bar = self.get(i).unwrap();
            open_time.push(bar.open_time.as_ref().map(|x| x.timestamp_millis()));
            close_time.push(bar.close_time.as_ref().map(|x| x.timestamp_millis()));
            open.push(bar.open);
            high.push(bar.high);
            low.push(bar.low);
            close.push(bar.close);
            volume.push(bar.volume);
        }
        let df = DataFrame::new(vec![
            Series::new("open_time", open_time),
            Series::new("close_time", close_time),
            Series::new("open", open),
            Series::new("high", high),
            Series::new("low", low),
            Series::new("close", close),
            Series::new("volume", volume),
        ]);
        return df;
    }

    #[cfg(feature = "bindings_py")]
    #[inline]
    fn into_py(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        dict.set_item("open_time", self.open_time())?;
        dict.set_item("close_time", self.close_time())?;
        dict.set_item("open", self.open())?;
        dict.set_item("high", self.high())?;
        dict.set_item("low", self.low())?;
        dict.set_item("close", self.close())?;
        dict.set_item("volume", self.volume())?;
        Ok(dict.into())
    }

    #[cfg(feature = "polars")]
    #[inline]
    fn write_csv(&self, path: &Path) {
        let df: DataFrame = self.to_polars().unwrap();
        write_df_csv(Path::new(&path), &mut df.clone()).unwrap();
    }

    #[cfg(feature = "polars")]
    #[inline]
    fn write_parquet(&self, path: &Path) {
        let df: DataFrame = self.to_polars().unwrap();
        write_df_parquet(Path::new(&path), &mut df.clone()).unwrap();
    }

    #[inline]
    fn sanity_check(&self) -> Result<(), Vec<String>> {
        let mut messages = vec![];
        if self.len() == 0 {
            messages.push("ohlcv is empty".to_string());
        }
        let all_volume_nan = self
            .slice(0..self.len())
            .iter()
            .all(|bar| bar.volume().is_nan());
        if all_volume_nan {
            messages.push("ohlcv volume is all NaN".to_string());
        }
        for bar_index in 0..self.len() {
            let bar: OhlcvBar = self.get(bar_index).unwrap();
            let prev_bar: Option<OhlcvBar> = if bar_index > 0 {
                Some(self.get(bar_index - 1).unwrap())
            } else {
                None
            };
            let mut bar_messages = vec![];
            if bar.close_time() <= bar.open_time() {
                bar_messages.push("close_time <= open_time".to_string());
            }
            if let Some(prev_bar) = prev_bar {
                if bar.open_time() < prev_bar.close_time() {
                    bar_messages.push("open_time < prev_bar.close_time".to_string());
                }
                if bar.close_time() <= prev_bar.close_time() {
                    bar_messages.push("close_time <= prev_bar.close_time".to_string());
                }
                if bar.open_time() <= prev_bar.open_time() {
                    bar_messages.push("open_time <= prev_bar.open_time".to_string());
                }
            }
            if bar.open().is_nan() {
                bar_messages.push("open is NaN".to_string());
            }
            if bar.high().is_nan() {
                bar_messages.push("high is NaN".to_string());
            }
            if bar.low().is_nan() {
                bar_messages.push("low is NaN".to_string());
            }
            if bar.close().is_nan() {
                bar_messages.push("close is NaN".to_string());
            }
            if !all_volume_nan && bar.volume().is_nan() {
                bar_messages.push("volume is NaN".to_string());
            }
            if bar_messages.len() > 0 {
                messages.push(format!(
                    "bar[{} | {:?} | {:?}]: {}",
                    bar_index,
                    bar.open_time(),
                    bar.close_time(),
                    bar_messages.join(", ")
                ));
            }
        }
        return if messages.len() > 0 {
            Err(messages)
        } else {
            Ok(())
        };
    }

    #[inline]
    fn find_bar_index_open_time_eq(&self, open_time: &DateTime<Utc>) -> Option<usize> {
        let bars = self.bars();
        for (index, bar) in bars.iter().enumerate() {
            if let Some(bar_open_time) = bar.open_time() {
                if bar_open_time == open_time {
                    return Some(index);
                }
            }
        }
        return None;
    }

    #[inline]
    fn find_bar_index_open_time_geq(&self, open_time: &DateTime<Utc>) -> Option<usize> {
        let bars = self.bars();
        for (index, bar) in bars.iter().enumerate() {
            if let Some(bar_open_time) = bar.open_time() {
                if bar_open_time >= open_time {
                    return Some(index);
                }
            }
        }
        return None;
    }
}

pub trait OhlcvWriter: fmt::Debug {
    fn push(&mut self, bar: OhlcvBar);

    #[inline]
    fn push_many(&mut self, bars: Vec<OhlcvBar>) {
        for bar in bars {
            self.push(bar);
        }
    }

    fn set(&mut self, index: usize, bar: OhlcvBar);

    #[cfg(feature = "polars")]
    #[inline]
    fn read_polars(&mut self, df: &DataFrame) {
        let bars = ohlcv_bars_from_polars(df, "s");
        self.push_many(bars);
    }

    #[cfg(feature = "polars")]
    #[inline]
    fn read_csv(&mut self, path: &Path) {
        let df = read_df_csv(path).unwrap();
        self.read_polars(&df);
    }

    #[cfg(feature = "polars")]
    #[inline]
    fn read_parquet(&mut self, path: &Path) {
        let df = read_df_parquet(path).unwrap();
        self.read_polars(&df);
    }
}

pub trait OhlcvReaderOps: OhlcvReader {
    #[inline]
    fn first(&mut self) -> Option<OhlcvBar> {
        self.at(0)
    }

    #[inline]
    fn last(&mut self) -> Option<OhlcvBar> {
        self.at(-1)
    }

    fn copy(&self) -> Self;
    fn head(&self, n: usize) -> Vec<OhlcvBar>;
    fn tail(&self, n: usize) -> Vec<OhlcvBar>;
    fn resample(&self, timeframe: Timeframe, align_utc: bool) -> Self;
}

pub trait OhlcvWriterOps: OhlcvWriter {
    fn extend(&mut self, other: &Self);
    fn sort(&mut self, ascending: bool);
    fn reverse(&mut self);
    fn clear(&mut self);
    fn pop(&mut self) -> Option<OhlcvBar>;
    fn shift(&mut self) -> Option<OhlcvBar>;
}

#[derive(Debug, Clone)]
pub struct Ohlcv {
    bars: Vec<OhlcvBar>,
    timeframe: Timeframe,
}

impl Default for Ohlcv {
    fn default() -> Self {
        Self {
            bars: vec![],
            timeframe: Timeframe::Unknown(),
        }
    }
}

#[cfg(feature = "polars")]
impl Into<DataFrame> for &Ohlcv {
    fn into(self) -> DataFrame {
        return self.to_polars().unwrap();
    }
}

impl Ohlcv {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn from_bars(bars: Vec<OhlcvBar>) -> Self {
        Self {
            bars,
            ..Self::default()
        }
    }

    #[inline]
    pub fn timeframe(&self) -> Timeframe {
        self.timeframe
    }

    #[inline]
    pub fn set_timeframe(&mut self, timeframe: Timeframe) {
        self.timeframe = timeframe;
    }
}

impl OhlcvReader for Ohlcv {
    #[inline]
    fn len(&self) -> usize {
        self.bars.len()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<OhlcvBar> {
        self.bars.get(index).copied()
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Vec<OhlcvBar> {
        self.bars[range].to_vec()
    }

    #[inline]
    fn into_box(self) -> Box<dyn OhlcvReader> {
        Box::new(self)
    }

    #[inline]
    fn clone_box(&self) -> Box<dyn OhlcvReader> {
        Box::new(self.clone())
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl OhlcvWriter for Ohlcv {
    #[inline]
    fn push(&mut self, bar: OhlcvBar) {
        self.bars.push(bar);
    }

    #[inline]
    fn push_many(&mut self, bars: Vec<OhlcvBar>) {
        self.bars.extend(bars);
    }

    #[inline]
    fn set(&mut self, index: usize, bar: OhlcvBar) {
        assert!(
            index < self.len(),
            "Index {} out of bounds {}",
            index,
            self.len()
        );
        self.bars[index] = bar;
    }
}

impl OhlcvReaderOps for Ohlcv {
    #[inline]
    fn copy(&self) -> Self {
        return Self {
            bars: self.bars(),
            timeframe: self.timeframe,
        };
    }

    #[inline]
    fn head(&self, n: usize) -> Vec<OhlcvBar> {
        self.slice(0..n)
    }

    #[inline]
    fn tail(&self, n: usize) -> Vec<OhlcvBar> {
        if n > self.len() {
            return self.bars();
        }
        self.slice(self.len() - n..self.len())
    }

    #[inline]
    fn resample(&self, timeframe: Timeframe, align_utc: bool) -> Self {
        let bars = resample(&self.bars, timeframe, align_utc);
        Self { bars, timeframe }
    }
}

impl OhlcvWriterOps for Ohlcv {
    #[inline]
    fn extend(&mut self, other: &Self) {
        self.bars.extend(other.bars());
    }

    #[inline]
    fn sort(&mut self, ascending: bool) {
        if ascending {
            self.bars.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        } else {
            self.bars.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
        }
    }

    #[inline]
    fn reverse(&mut self) {
        self.bars.reverse();
    }

    #[inline]
    fn clear(&mut self) {
        self.bars.clear();
    }

    #[inline]
    fn pop(&mut self) -> Option<OhlcvBar> {
        self.bars.pop()
    }

    #[inline]
    fn shift(&mut self) -> Option<OhlcvBar> {
        if self.len() == 0 {
            return None;
        }
        return Some(self.bars.remove(0));
    }
}

#[derive(Debug, Clone)]
pub struct RcOhlcv {
    inner: Rc<RefCell<Ohlcv>>,
}

impl Into<Ohlcv> for RcOhlcv {
    #[inline]
    fn into(self) -> Ohlcv {
        self.inner.borrow().clone()
    }
}

impl Into<RcOhlcv> for Ohlcv {
    #[inline]
    fn into(self) -> RcOhlcv {
        RcOhlcv {
            inner: Rc::new(RefCell::new(self)),
        }
    }
}

#[cfg(feature = "polars")]
impl Into<DataFrame> for &RcOhlcv {
    #[inline]
    fn into(self) -> DataFrame {
        self.inner.borrow().to_polars().unwrap()
    }
}

impl RcOhlcv {
    #[inline]
    pub fn new() -> Self {
        Ohlcv::new().into()
    }

    #[inline]
    pub fn from_bars(bars: Vec<OhlcvBar>) -> Self {
        Ohlcv::from_bars(bars).into()
    }

    #[inline]
    pub fn timeframe(&self) -> Timeframe {
        self.inner.borrow().timeframe()
    }

    #[inline]
    pub fn set_timeframe(&self, timeframe: Timeframe) {
        self.inner.borrow_mut().set_timeframe(timeframe);
    }
}

impl OhlcvReader for RcOhlcv {
    #[inline]
    fn len(&self) -> usize {
        self.inner.borrow().len()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<OhlcvBar> {
        self.inner.borrow().get(index)
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Vec<OhlcvBar> {
        self.inner.borrow().slice(range)
    }

    #[inline]
    fn into_box(self) -> Box<dyn OhlcvReader> {
        Box::new(self)
    }

    #[inline]
    fn clone_box(&self) -> Box<dyn OhlcvReader> {
        self.clone().into_box()
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl OhlcvWriter for RcOhlcv {
    #[inline]
    fn push(&mut self, bar: OhlcvBar) {
        self.inner.borrow_mut().push(bar);
    }

    #[inline]
    fn push_many(&mut self, bars: Vec<OhlcvBar>) {
        self.inner.borrow_mut().push_many(bars);
    }

    #[inline]
    fn set(&mut self, index: usize, bar: OhlcvBar) {
        self.inner.borrow_mut().set(index, bar);
    }
}

impl OhlcvReaderOps for RcOhlcv {
    #[inline]
    fn copy(&self) -> Self {
        self.inner.borrow().clone().into()
    }

    #[inline]
    fn head(&self, n: usize) -> Vec<OhlcvBar> {
        self.inner.borrow().head(n)
    }

    #[inline]
    fn tail(&self, n: usize) -> Vec<OhlcvBar> {
        self.inner.borrow().tail(n)
    }

    #[inline]
    fn resample(&self, timeframe: Timeframe, align_utc: bool) -> Self {
        self.inner.borrow().resample(timeframe, align_utc).into()
    }
}

impl OhlcvWriterOps for RcOhlcv {
    #[inline]
    fn extend(&mut self, other: &Self) {
        self.inner.borrow_mut().extend(&other.inner.borrow());
    }

    #[inline]
    fn sort(&mut self, ascending: bool) {
        self.inner.borrow_mut().sort(ascending);
    }

    #[inline]
    fn reverse(&mut self) {
        self.inner.borrow_mut().reverse();
    }

    #[inline]
    fn clear(&mut self) {
        self.inner.borrow_mut().clear();
    }

    #[inline]
    fn pop(&mut self) -> Option<OhlcvBar> {
        self.inner.borrow_mut().pop()
    }

    #[inline]
    fn shift(&mut self) -> Option<OhlcvBar> {
        self.inner.borrow_mut().shift()
    }
}

#[derive(Debug, Clone)]
pub struct ArcOhlcv {
    inner: Arc<RwLock<Ohlcv>>,
}

impl Into<Ohlcv> for ArcOhlcv {
    #[inline]
    fn into(self) -> Ohlcv {
        self.inner.write().unwrap().clone()
    }
}

impl Into<ArcOhlcv> for Ohlcv {
    #[inline]
    fn into(self) -> ArcOhlcv {
        ArcOhlcv {
            inner: Arc::new(RwLock::new(self)),
        }
    }
}

#[cfg(feature = "polars")]
impl Into<DataFrame> for &ArcOhlcv {
    #[inline]
    fn into(self) -> DataFrame {
        self.inner.read().unwrap().to_polars().unwrap()
    }
}

impl ArcOhlcv {
    #[inline]
    pub fn new() -> Self {
        Ohlcv::new().into()
    }

    #[inline]
    pub fn from_bars(bars: Vec<OhlcvBar>) -> Self {
        Ohlcv::from_bars(bars).into()
    }

    #[inline]
    pub fn timeframe(&self) -> Timeframe {
        self.inner.read().unwrap().timeframe()
    }

    #[inline]
    pub fn set_timeframe(&self, timeframe: Timeframe) {
        self.inner.write().unwrap().set_timeframe(timeframe);
    }
}

impl OhlcvReader for ArcOhlcv {
    #[inline]
    fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<OhlcvBar> {
        self.inner.read().unwrap().get(index)
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Vec<OhlcvBar> {
        self.inner.read().unwrap().slice(range)
    }

    #[inline]
    fn into_box(self) -> Box<dyn OhlcvReader> {
        Box::new(self)
    }

    #[inline]
    fn clone_box(&self) -> Box<dyn OhlcvReader> {
        self.clone().into_box()
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl OhlcvWriter for ArcOhlcv {
    #[inline]
    fn push(&mut self, bar: OhlcvBar) {
        self.inner.write().unwrap().push(bar);
    }

    #[inline]
    fn push_many(&mut self, bars: Vec<OhlcvBar>) {
        self.inner.write().unwrap().push_many(bars);
    }

    #[inline]
    fn set(&mut self, index: usize, bar: OhlcvBar) {
        self.inner.write().unwrap().set(index, bar);
    }
}

impl OhlcvReaderOps for ArcOhlcv {
    #[inline]
    fn copy(&self) -> Self {
        self.inner.read().unwrap().clone().into()
    }

    #[inline]
    fn head(&self, n: usize) -> Vec<OhlcvBar> {
        self.inner.read().unwrap().head(n)
    }

    #[inline]
    fn tail(&self, n: usize) -> Vec<OhlcvBar> {
        self.inner.read().unwrap().tail(n)
    }

    #[inline]
    fn resample(&self, timeframe: Timeframe, align_utc: bool) -> Self {
        self.inner
            .read()
            .unwrap()
            .resample(timeframe, align_utc)
            .into()
    }
}

impl OhlcvWriterOps for ArcOhlcv {
    #[inline]
    fn extend(&mut self, other: &Self) {
        self.inner
            .write()
            .unwrap()
            .extend(&other.inner.read().unwrap());
    }

    #[inline]
    fn sort(&mut self, ascending: bool) {
        self.inner.write().unwrap().sort(ascending);
    }

    #[inline]
    fn reverse(&mut self) {
        self.inner.write().unwrap().reverse();
    }

    #[inline]
    fn clear(&mut self) {
        self.inner.write().unwrap().clear();
    }

    #[inline]
    fn pop(&mut self) -> Option<OhlcvBar> {
        self.inner.write().unwrap().pop()
    }

    #[inline]
    fn shift(&mut self) -> Option<OhlcvBar> {
        self.inner.write().unwrap().shift()
    }
}

#[inline]
fn resample(bars: &[OhlcvBar], timeframe: Timeframe, align_utc: bool) -> Vec<OhlcvBar> {
    match timeframe {
        Timeframe::Ticks(n) => group_by_count(bars, n),
        Timeframe::Ranges(n) => group_by_count(bars, n),
        _ => {
            if align_utc {
                group_by_aligned_time(bars, timeframe)
            } else {
                // rolling time-based grouping (no pinned boundary)
                group_by_time(bars, timeframe.try_into().unwrap())
            }
        }
    }
}

fn group_by_count(bars: &[OhlcvBar], n: usize) -> Vec<OhlcvBar> {
    if n == 0 {
        return bars.to_vec();
    }
    let mut result = Vec::new();
    let mut aggregator = None;
    let mut count = 0usize;

    for bar in bars {
        if aggregator.is_none() {
            aggregator = Some(*bar);
            count = 1;
        } else if let Some(agg) = aggregator {
            // Merge this bar into aggregator
            let merged = agg.merge(bar);
            aggregator = Some(merged);
            count += 1;

            // If we reached n bars in this group, push aggregator & reset
            if count >= n {
                result.push(aggregator.unwrap());
                aggregator = None;
                count = 0;
            }
        }
    }

    // If there's a partial aggregator left, push it
    if let Some(agg) = aggregator {
        result.push(agg);
    }

    result
}

fn group_by_time(bars: &[OhlcvBar], duration: Duration) -> Vec<OhlcvBar> {
    let mut result = Vec::new();
    let mut aggregator: Option<OhlcvBar> = None;
    let mut bucket_start: Option<DateTime<Utc>> = None;

    for bar in bars {
        assert!(bar.open_time.is_some(), "Bar must have an open time");
        if aggregator.is_none() {
            aggregator = Some(*bar);
            bucket_start = Some(bar.open_time.unwrap());
        } else if let (Some(agg), Some(start)) = (aggregator, bucket_start) {
            // Determine if this bar crosses the boundary
            let boundary = start + duration;
            if bar.open_time.unwrap() >= boundary {
                // close out current aggregator
                result.push(agg);
                // start a new aggregator
                aggregator = Some(*bar);
                bucket_start = Some(bar.open_time.unwrap());
            } else {
                // merge into existing aggregator
                aggregator = Some(agg.merge(bar));
            }
        }
    }

    // Flush the last aggregator if present
    if let Some(agg) = aggregator {
        result.push(agg);
    }

    result
}

/// Floor a DateTime<Utc> to the start of the interval specified by Timeframe.
/// For example:
/// • Timeframe::Days(1) →  YYYY-MM-DD 00:00:00
/// • Timeframe::Months(1) → YYYY-MM-01 00:00:00
/// • Timeframe::Weeks(1) → Monday of that ISO week at 00:00:00
/// For multiples (e.g. Months(3)), we use an anchor reference (1970-01-01, or Monday for weeks)
/// and snap down to the largest multiple of n that does not exceed dt.
fn floor_to_timeframe(
    dt: chrono::DateTime<chrono::Utc>,
    tf: &Timeframe,
) -> chrono::DateTime<chrono::Utc> {
    use chrono::{Datelike, Duration, IsoWeek, NaiveDate, Timelike, Utc, Weekday};

    match tf {
        //----------------------------------------------------------------------
        // Years(n)
        //----------------------------------------------------------------------
        Timeframe::Years(n) => {
            // 1) Floor to Y-01-01 00:00:00
            let year_floor = dt
                .date_naive()
                .with_day0(0) // day=1 => day0 = day=1
                .unwrap() // ensure we're at the first day of the month
                .with_month0(0) // month=1 => month0 = month=0
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap();

            // 2) For n>1, do "year multiple" from an anchor (1970).
            //    We'll compute: year - anchorYear => mod n => subtract remainder.
            //    E.g. for n=5, we snap 2023 down to 2020, 2001 → 2000, etc.
            let anchor_year = 1970;
            let this_year = year_floor.year();
            let delta = this_year - anchor_year;
            let snapped = delta - (delta % (*n as i32));
            let new_year = anchor_year + snapped;
            let new_date = NaiveDate::from_ymd_opt(new_year, 1, 1).unwrap();
            new_date
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap()
        }

        //----------------------------------------------------------------------
        // Months(n)
        //----------------------------------------------------------------------
        Timeframe::Months(n) => {
            // 1) Floor to Y-M-01 00:00:00
            let (year, month, _day) = (dt.year(), dt.month(), dt.day());
            let month_floor = chrono::NaiveDate::from_ymd_opt(year as i32, month, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap();

            // 2) For n>1, do "month multiple" from anchor=1970-01
            //    We'll treat "year * 12 + (month-1)" as an integer offset from 1970-01.
            let anchor = chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
            let anchor_offset = (1970 * 12) + (1 - 1); // i.e. 1970-01 => 1970*12 + 0
            let this_offset = (year as i32) * 12 + (month as i32 - 1);
            let delta = this_offset - anchor_offset;
            let snapped = delta - (delta % (*n as i32));
            let new_offset = anchor_offset + snapped;

            // Reconstruct year+month from new_offset
            let new_year = new_offset / 12;
            let new_month = (new_offset % 12) + 1; // because month is 1-based
            let new_date = chrono::NaiveDate::from_ymd_opt(new_year, new_month as u32, 1).unwrap();
            new_date
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap()
        }

        //----------------------------------------------------------------------
        // Weeks(n)
        //----------------------------------------------------------------------
        Timeframe::Weeks(n) => {
            // Approach: 1) Floor dt to Monday 00:00.
            // For n>1, anchor from 1970-W01.
            // We'll do iso_week().week() => snap by mod n => reconstruct.
            //
            // Step 1: floor to naive "YYYY-MM-DD 00:00:00" ignoring daily time:
            let day_floor = dt.date_naive().and_hms_opt(0, 0, 0).unwrap();
            // 2) Move back day_floor.weekday().num_days_from_monday to find the Monday start:
            let weekday = day_floor.weekday().num_days_from_monday() as i64;
            let monday = day_floor - Duration::days(weekday);

            // Now "monday" is the Monday of that iso-week. For n=1, we’re done:
            if *n == 1 {
                return monday.and_local_timezone(Utc).unwrap();
            }

            // For n>1: anchor from 1970-01-05 (which was a Monday in ISO week 1 of 1970).
            let anchor = NaiveDate::from_ymd_opt(1970, 1, 5).unwrap(); // Monday of 1970-W01
            let anchor_ordinal = anchor.num_days_from_ce();
            let monday_ordinal = monday.num_days_from_ce();
            let delta = monday_ordinal - anchor_ordinal;
            let snapped = delta - (delta % (*n as i32));
            let new_ordinal = anchor_ordinal + snapped;
            let new_date = NaiveDate::from_num_days_from_ce(new_ordinal);
            new_date
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap()
        }

        //----------------------------------------------------------------------
        // Days(n)
        //----------------------------------------------------------------------
        Timeframe::Days(1) => {
            // simplest daily: floor to local midnight
            dt.date_naive()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap()
        }
        Timeframe::Days(n) => {
            // same approach as above, but anchor Y-m-d to 1970-01-01, mod n
            let day_start = dt.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let anchor = chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
            let anchor_ord = anchor.num_days_from_ce();
            let this_ord = day_start.date().num_days_from_ce();
            let delta = this_ord - anchor_ord;
            let snapped = delta - (delta % (*n as i32));
            let new_ord = anchor_ord + snapped;
            let new_date = chrono::NaiveDate::from_num_days_from_ce(new_ord);
            new_date
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap()
        }

        //----------------------------------------------------------------------
        // Hours(n)
        //----------------------------------------------------------------------
        Timeframe::Hours(n) => {
            let floored_hour = dt.hour() - (dt.hour() % (*n as u32));
            dt.date_naive()
                .and_hms_opt(floored_hour, 0, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap()
        }

        //----------------------------------------------------------------------
        // Minutes(n)
        //----------------------------------------------------------------------
        Timeframe::Minutes(n) => {
            let floored_min = dt.minute() - (dt.minute() % (*n as u32));
            dt.date_naive()
                .and_hms_opt(dt.hour(), floored_min, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap()
        }

        //----------------------------------------------------------------------
        // Seconds(n)
        //----------------------------------------------------------------------
        Timeframe::Seconds(n) => {
            let floored_sec = dt.second() - (dt.second() % (*n as u32));
            dt.date_naive()
                .and_hms_opt(dt.hour(), dt.minute(), floored_sec)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap()
        }

        //----------------------------------------------------------------------
        // Ticks(n), Ranges(n), Unknown => no alignment
        //----------------------------------------------------------------------
        _ => dt,
    }
}

// Step 2: A small helper that forces aggregator.close_time to the *end* of that bucket.
fn finalize_aggregator(agg: &mut OhlcvBar, tf: &Timeframe) {
    match tf {
        // If you want daily bars to end exactly at next midnight:
        Timeframe::Days(n) => {
            // close_time = open_time + (n * 24 hours)
            let offset = chrono::Duration::days(*n as i64);
            agg.close_time = Some(agg.open_time.unwrap() + offset);
        }

        // If you want hour bars pinned similarly, do the same for Hours(n).
        Timeframe::Hours(n) => {
            let offset = chrono::Duration::hours(*n as i64);
            agg.close_time = Some(agg.open_time.unwrap() + offset);
        }

        // Otherwise leave aggregator.close_time as the last intraday bar timestamp
        _ => {}
    }
}

// Step 3: group_by_aligned_time that sets open_time to the floored boundary
// and calls finalize_aggregator whenever we finish a bucket.
fn group_by_aligned_time(bars: &[OhlcvBar], timeframe: Timeframe) -> Vec<OhlcvBar> {
    let mut result = Vec::new();
    let mut aggregator: Option<OhlcvBar> = None;
    let mut current_bucket: Option<chrono::DateTime<chrono::Utc>> = None;

    for bar in bars {
        let bucket = floor_to_timeframe(bar.open_time.unwrap(), &timeframe);

        match aggregator {
            None => {
                // start a new aggregator, open_time pinned to the boundary
                let mut new_bar = *bar;
                new_bar.open_time = Some(bucket);
                aggregator = Some(new_bar);
                current_bucket = Some(bucket);
            }
            Some(mut agg) => {
                if Some(bucket) == current_bucket {
                    // same daily bucket => merge
                    agg = agg.merge(bar);
                    aggregator = Some(agg);
                } else {
                    // finalize old aggregator
                    finalize_aggregator(&mut agg, &timeframe);
                    result.push(agg);

                    // start next aggregator
                    let mut new_bar = *bar;
                    new_bar.open_time = Some(bucket);
                    aggregator = Some(new_bar);
                    current_bucket = Some(bucket);
                }
            }
        }
    }
    // flush last aggregator
    if let Some(mut agg) = aggregator {
        finalize_aggregator(&mut agg, &timeframe);
        result.push(agg);
    }

    result
}

/*
Below is the overall idea behind “align_utc” in the resampling logic:

• When align_utc = false:
The resampler performs a simple “rolling” grouping. It takes the earliest bar’s timestamp as a starting point and groups subsequent bars in fixed-length durations (e.g., every 24 hours for daily). This causes each group/window to shift relative to the actual calendar boundaries.

• When align_utc = true:
The resampler “pins” the bars to calendar-friendly boundaries in UTC. For example:
– Daily bars get folded into the 00:00–23:59 UTC bucket for each calendar day, rather than just any 24-hour interval.
– Weekly bars get anchored to Monday 00:00 UTC and end on Sunday 23:59 UTC (or the next Monday’s boundary).
– Monthly bars align to the first day of the month at 00:00 UTC, and so on for other timeframes.

This alignment makes the resampled bars match conventional calendar frames (like TradingView’s daily bars, which always cover midnight-to-midnight UTC). If you do not set align_utc (or set it to false), the resampling instead uses rolling windows that begin wherever your data starts, not necessarily on a neat boundary like midnight UTC.
*/

#[inline]
fn sort_bars_inplace(bars: &mut [OhlcvBar], desc: bool) {
    if desc {
        bars.sort_by(|a, b| b.open_time.cmp(&a.open_time));
    } else {
        bars.sort_by(|a, b| a.open_time.cmp(&b.open_time));
    }
}

#[cfg(feature = "polars")]
#[inline]
fn ohlcv_bars_from_polars(df: &DataFrame, time_unit: &str) -> Vec<OhlcvBar> {
    let cols = df.get_column_names();

    let open = Some(df.column("open").unwrap().to_f64());
    let high = Some(df.column("high").unwrap().to_f64());
    let low = Some(df.column("low").unwrap().to_f64());
    let close = Some(df.column("close").unwrap().to_f64());

    let volume = if cols.contains(&"volume") {
        Some(df.column("volume").unwrap().to_f64())
    } else {
        None
    };

    let open_time_col = if cols.contains(&"open_time") {
        Some("open_time")
    } else if cols.contains(&"time") {
        Some("time")
    } else {
        None
    };
    let close_time_col = if cols.contains(&"close_time") {
        Some("close_time")
    } else if cols.contains(&"time") {
        Some("time")
    } else {
        None
    };

    let open_time = if let Some(col) = open_time_col {
        if time_unit == "s" {
            Some(df.column(col).unwrap().to_datetime_from_s())
        } else if time_unit == "ms" {
            Some(df.column(col).unwrap().to_datetime_from_ms())
        } else {
            panic!("Invalid open time unit: {}", time_unit);
        }
    } else {
        None
    };

    let close_time = if let Some(col) = close_time_col {
        if time_unit == "s" {
            Some(df.column(col).unwrap().to_datetime_from_s())
        } else if time_unit == "ms" {
            Some(df.column(col).unwrap().to_datetime_from_ms())
        } else {
            panic!("Invalid close time unit: {}", time_unit);
        }
    } else {
        None
    };

    return zip_ohlcv_bars(open_time, close_time, open, high, low, close, volume);
}

#[inline]
pub fn hl2(high: f64, low: f64) -> f64 {
    return (high + low) / 2.0;
}

#[inline]
pub fn hlc3(high: f64, low: f64, close: f64) -> f64 {
    return (high + low + close) / 3.0;
}

#[inline]
pub fn hlcc4(high: f64, low: f64, close: f64) -> f64 {
    return (high + low + close + close) / 4.0;
}
