cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum, gen_stub_pyfunction}};
  use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    types::{PySequence, PySlice, PySliceIndices},
  };
  use crate::rs_utils::{pyslice_to_range};
}}
cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}
cfg_if::cfg_if! { if #[cfg(feature = "polars")] {
  use polars::prelude::*;
  use polars::frame::DataFrame;
  use crate::rs_utils::{SeriesCastUtils};
}}
use crate::rs_utils::get_oldest_possible_datetime;
use crate::utils::{hl2, hlc3, hlcc4};
use chrono::prelude::*;
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use itertools::izip;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::cell::{Ref, RefCell};
use std::fmt::Debug;
use std::{cell::Cell, rc::Rc};
use std::{ops::Range, path::Path, sync::Arc};

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "OhlcvBar"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OhlcvBar {
    open_time: DateTime<Utc>,
    close_time: DateTime<Utc>,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl Default for OhlcvBar {
    fn default() -> Self {
        Self {
            open_time: get_oldest_possible_datetime(),
            close_time: get_oldest_possible_datetime(),
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
        open_time: DateTime<Utc>,
        close_time: DateTime<Utc>,
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
    pub fn open_time(&self) -> &DateTime<Utc> {
        &self.open_time
    }

    #[inline]
    pub fn close_time(&self) -> &DateTime<Utc> {
        &self.close_time
    }

    #[inline]
    pub fn open_time_ms(&self) -> i64 {
        self.open_time.timestamp_millis()
    }

    #[inline]
    pub fn close_time_ms(&self) -> i64 {
        self.close_time.timestamp_millis()
    }

    #[inline]
    pub fn open(&self) -> f64 {
        self.open
    }

    #[inline]
    pub fn high(&self) -> f64 {
        self.high
    }

    #[inline]
    pub fn low(&self) -> f64 {
        self.low
    }

    #[inline]
    pub fn close(&self) -> f64 {
        self.close
    }

    #[inline]
    pub fn volume(&self) -> f64 {
        self.volume
    }

    #[inline]
    pub fn hl2(&self) -> f64 {
        hl2(self.high, self.low)
    }

    #[inline]
    pub fn hlc3(&self) -> f64 {
        hlc3(self.high, self.low, self.close)
    }

    #[inline]
    pub fn hlcc4(&self) -> f64 {
        hlcc4(self.high, self.low, self.close)
    }

    #[inline]
    pub fn fmt(&self) -> String {
        format!(
            "OhlcvBar(open_time={}, close_time={}, open={}, high={}, low={}, close={}, volume={})",
            self.open_time,
            self.close_time,
            self.open,
            self.high,
            self.low,
            self.close,
            self.volume
        )
    }
}

pub trait OhlcvReader: Debug {
    fn len(&self) -> usize;
    fn bar(&self, index: usize) -> &OhlcvBar;
    fn bars(&self, range: Range<usize>) -> &[OhlcvBar];
    fn all_bars(&self) -> &[OhlcvBar] {
        return self.bars(0..self.len());
    }
    fn into_box(self) -> Box<dyn OhlcvReader>;
    fn clone_box(&self) -> Box<dyn OhlcvReader>;
    fn as_any(&self) -> &dyn Any;

    #[inline]
    fn open(&self) -> Vec<f64> {
        self.all_bars().iter().map(|bar| bar.open()).collect()
    }

    #[inline]
    fn high(&self) -> Vec<f64> {
        self.all_bars().iter().map(|bar| bar.high()).collect()
    }

    #[inline]
    fn low(&self) -> Vec<f64> {
        self.all_bars().iter().map(|bar| bar.low()).collect()
    }

    #[inline]
    fn close(&self) -> Vec<f64> {
        self.all_bars().iter().map(|bar| bar.close()).collect()
    }

    #[inline]
    fn volume(&self) -> Vec<f64> {
        self.all_bars().iter().map(|bar| bar.volume()).collect()
    }

    #[inline]
    fn hl2(&self) -> Vec<f64> {
        self.all_bars().iter().map(|bar| bar.hl2()).collect()
    }

    #[inline]
    fn hlc3(&self) -> Vec<f64> {
        self.all_bars().iter().map(|bar| bar.hlc3()).collect()
    }

    #[inline]
    fn hlcc4(&self) -> Vec<f64> {
        self.all_bars().iter().map(|bar| bar.hlcc4()).collect()
    }

    #[inline]
    fn open_time(&self) -> Vec<DateTime<Utc>> {
        self.all_bars().iter().map(|bar| *bar.open_time()).collect()
    }

    #[inline]
    fn close_time(&self) -> Vec<DateTime<Utc>> {
        self.all_bars()
            .iter()
            .map(|bar| *bar.close_time())
            .collect()
    }

    #[inline]
    fn open_time_ms(&self) -> Vec<i64> {
        self.all_bars()
            .iter()
            .map(|bar| bar.open_time_ms())
            .collect()
    }

    #[inline]
    fn close_time_ms(&self) -> Vec<i64> {
        self.all_bars()
            .iter()
            .map(|bar| bar.close_time_ms())
            .collect()
    }
}

pub trait OhlcvWriter: Debug {
    fn push(&mut self, bar: OhlcvBar);

    #[inline]
    fn push_many(&mut self, bars: &[OhlcvBar]) {
        for bar in bars {
            self.push(*bar);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ohlcv {
    bars: Vec<OhlcvBar>,
}

impl Ohlcv {
    #[inline]
    pub fn from_bars(bars: Vec<OhlcvBar>) -> Self {
        Self { bars }
    }

    #[inline]
    pub fn empty_bars(count: usize) -> Self {
        let bars = vec![OhlcvBar::default(); count];
        Self::from_bars(bars)
    }

    #[inline]
    pub fn from_uniform_price(prices: Vec<f64>) -> Self {
        let bars: Vec<OhlcvBar> = prices
            .into_iter()
            .enumerate()
            .map(|(_, price)| OhlcvBar {
                open: price,
                high: price,
                low: price,
                close: price,
                volume: f64::NAN,
                open_time: get_oldest_possible_datetime(),
                close_time: get_oldest_possible_datetime(),
            })
            .collect();
        return Self::from_bars(bars);
    }
}

impl OhlcvReader for Ohlcv {
    #[inline]
    fn len(&self) -> usize {
        self.bars.len()
    }

    #[inline]
    fn bar(&self, index: usize) -> &OhlcvBar {
        &self.bars[index]
    }

    #[inline]
    fn bars(&self, range: Range<usize>) -> &[OhlcvBar] {
        &self.bars[range]
    }

    #[inline]
    fn into_box(self) -> Box<dyn OhlcvReader> {
        Box::new(self)
    }

    #[inline]
    fn clone_box(&self) -> Box<dyn OhlcvReader> {
        Box::new(std::clone::Clone::clone(self))
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
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "ArcOhlcv"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = ArcOhlcv))]
#[derive(Clone, Debug)]
#[doc = "Multi-threaded immutable OHLCV data."]
pub struct ArcOhlcv {
    inner: Arc<Ohlcv>,
}

impl ArcOhlcv {
    pub fn from_bars(bars: Vec<OhlcvBar>) -> Self {
        Self {
            inner: Arc::new(Ohlcv::from_bars(bars)),
        }
    }
}

impl OhlcvReader for ArcOhlcv {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    fn bar(&self, index: usize) -> &OhlcvBar {
        self.inner.bar(index)
    }

    #[inline]
    fn bars(&self, range: Range<usize>) -> &[OhlcvBar] {
        self.inner.bars(range)
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

impl ArcOhlcv {
    pub fn fmt(&self) -> String {
        format!("Ohlcv(len={})", self.len())
    }
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "Ohlcv", unsendable))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = Ohlcv))]
#[derive(Clone, Debug)]
#[doc = "Single-threaded mutable OHLCV data."]
pub struct OhlcvLoader {
    inner: Rc<RefCell<Ohlcv>>,
}

impl OhlcvReader for OhlcvLoader {
    #[inline]
    fn len(&self) -> usize {
        self.inner.borrow().len()
    }

    #[inline]
    fn bar(&self, index: usize) -> &OhlcvBar {
        let borrowed = self.inner.borrow();
        let ptr: *const OhlcvBar = &borrowed.bars[index];
        unsafe { &*ptr }
    }

    #[inline]
    fn bars(&self, range: Range<usize>) -> &[OhlcvBar] {
        let borrowed = self.inner.borrow();
        let bars = &borrowed.bars[range];
        unsafe { std::slice::from_raw_parts(bars.as_ptr(), bars.len()) }
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

impl OhlcvWriter for OhlcvLoader {
    #[inline]
    fn push(&mut self, bar: OhlcvBar) {
        self.inner.borrow_mut().push(bar);
    }
}

impl OhlcvLoader {
    pub fn from_bars(bars: Vec<OhlcvBar>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(Ohlcv::from_bars(bars))),
        }
    }

    pub fn fmt(&self) -> String {
        format!("OhlcvLoader(len={})", self.len())
    }
}

impl Into<Option<ArcOhlcv>> for &dyn OhlcvReader {
    fn into(self) -> Option<ArcOhlcv> {
        self.as_any().downcast_ref::<ArcOhlcv>().cloned()
    }
}

impl Into<Option<OhlcvLoader>> for &dyn OhlcvReader {
    fn into(self) -> Option<OhlcvLoader> {
        self.as_any().downcast_ref::<OhlcvLoader>().cloned()
    }
}

impl Into<ArcOhlcv> for Ohlcv {
    fn into(self) -> ArcOhlcv {
        ArcOhlcv {
            inner: Arc::new(self),
        }
    }
}

impl Into<OhlcvLoader> for Ohlcv {
    fn into(self) -> OhlcvLoader {
        OhlcvLoader {
            inner: Rc::new(RefCell::new(self)),
        }
    }
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyfunction)]
#[cfg_attr(feature = "bindings_py", pyfunction)]
#[cfg_attr(feature = "bindings_py", pyo3(signature=(open_time=None, close_time=None, open=None, high=None, low=None, close=None, volume=None)))]
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

    let open_time = open_time.unwrap_or(vec![None; len]);
    let close_time = close_time.unwrap_or(vec![None; len]);
    let open = open.unwrap_or(vec![f64::NAN; len]);
    let high = high.unwrap_or(vec![f64::NAN; len]);
    let low = low.unwrap_or(vec![f64::NAN; len]);
    let close = close.unwrap_or(vec![f64::NAN; len]);
    let volume = volume.unwrap_or(vec![f64::NAN; len]);

    assert!(len == open_time.len());
    assert!(len == close_time.len());
    assert!(len == open.len());
    assert!(len == high.len());
    assert!(len == low.len());
    assert!(len == close.len());
    assert!(len == volume.len());

    let bars: Vec<OhlcvBar> = izip!(
        open.iter(),
        high.iter(),
        low.iter(),
        close.iter(),
        volume.iter(),
        open_time.iter(),
        close_time.iter(),
    )
    .enumerate()
    .map(
        |(_, (&open, &high, &low, &close, &volume, open_time, close_time))| {
            OhlcvBar::new(
                open_time.unwrap_or_else(|| get_oldest_possible_datetime()),
                close_time.unwrap_or_else(|| get_oldest_possible_datetime()),
                open,
                high,
                low,
                close,
                volume,
            )
        },
    )
    .collect();

    return bars;
}

cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
    #[wasm_bindgen(js_name=zipOhlcvBars)]
    #[inline]
    pub fn js_zip_ohlcv_bars(
        open_time: Option<Vec<js_sys::Date>>,
        close_time: Option<Vec<js_sys::Date>>,
        open: Option<Vec<f64>>,
        high: Option<Vec<f64>>,
        low: Option<Vec<f64>>,
        close: Option<Vec<f64>>,
        volume: Option<Vec<f64>>,
    ) -> Vec<OhlcvBar> {
        let open_time = open_time.map(|list| list.iter().map(|x| Some(DateTime::from(x))).collect::<Vec<_>>());
        let close_time = close_time.map(|list| list.iter().map(|x| Some(DateTime::from(x))).collect::<Vec<_>>());
        zip_ohlcv_bars(open_time, close_time, open, high, low, close, volume)
    }
}}

cfg_if::cfg_if! { if #[cfg(feature = "polars")] {
    #[inline]
    pub fn ohlcv_bars_from_polars(df: &DataFrame) -> Vec<OhlcvBar> {
        let cols = df.get_column_names();

        let open = Some(df.column("open").unwrap().to_f64());
        let high = Some( df.column("high").unwrap().to_f64());
        let low = Some(df.column("low").unwrap().to_f64());
        let close = Some(df.column("close").unwrap().to_f64());

        let volume = if cols.contains(&"volume") {
            Some(df.column("volume").unwrap().to_f64())
        } else {
            None
        };

        let open_time = if cols.contains(&"open_time") {
            Some(df.column("open_time").unwrap().to_datetime())
        } else if cols.contains(&"time") {
            Some(df.column("time").unwrap().to_datetime())
        } else {
            None
        };

        let close_time = if cols.contains(&"close_time") {
            Some(df.column("close_time").unwrap().to_datetime())
        } else if cols.contains(&"time") {
            Some(df.column("time").unwrap().to_datetime())
        } else {
            None
        };

        return zip_ohlcv_bars(open_time, close_time, open, high, low, close, volume);
    }
}}
