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
  use polars::frame::DataFrame;
  use crate::rs_utils::{SeriesCastUtils};
  use crate::rs_utils::{read_df};
  use crate::rs_utils::{read_df_csv, read_df_parquet};
}}
use crate::rs_utils::get_oldest_possible_datetime;
use crate::timeframe::Timeframe;
use crate::utils::{hl2, hlc3, hlcc4, Ord};
use chrono::{prelude::*, Duration};
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
        bar.open_time = open_time;
        bar.close_time = close_time;
        bar.open = open;
        bar.high = high;
        bar.low = low;
        bar.close = close;
        bar.volume = volume;
        return bar;
    }

    // #[inline]
    // pub fn hash(&self) -> String {
    //     return format!(
    //         "{}-{}-{}-{}-{}-{}-{}",
    //         self.open_time.timestamp_millis(),
    //         self.close_time.timestamp_millis(),
    //         self.open,
    //         self.high,
    //         self.low,
    //         self.close,
    //         self.volume
    //     );
    // }
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
    pub bars: Vec<OhlcvBar>,
}

impl Default for Ohlcv {
    fn default() -> Self {
        Self::empty()
    }
}

impl Ohlcv {
    #[inline]
    pub fn empty() -> Self {
        Self { bars: vec![] }
    }

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

    #[inline]
    pub fn resample(&self, timeframe: Timeframe, align_utc: bool) -> Self {
        /*
        TradingView’s “daily” bars are conventionally anchored to a calendar boundary (e.g. midnight UTC).
        That means all intraday bars between 00:00 and 23:59 form one “daily” candlestick.
        Rolling bars, on the other hand, merely group a fixed duration (like 24 hours) starting at the first bar’s timestamp.
        So the boundary “drifts” relative to the calendar.
        If you want TradingView-like daily bars, you must “floor” bar.open_time to the midnight boundary (or some other session start).
        For example, if bar.open_time is 2011-09-01 09:00 UTC, you round it down to 2011-09-01 00:00 UTC for the “day bucket,” ensuring each day’s bar is pinned to 00:00–23:59.
        Therefore, the core reason they differ is that TradingView daily bars align to the calendar day, whereas a rolling resampler aligns to an arbitrary 24-hour (or N-hour) window from the data’s initial timestamp. */
        Self::from_bars(resample(&self.bars, timeframe, align_utc))
    }

    #[inline]
    pub fn sort(&mut self, order: Ord) {
        match order {
            Ord::Asc => sort_bars_inplace(&mut self.bars, false),
            Ord::Desc => sort_bars_inplace(&mut self.bars, true),
        }
    }

    #[inline]
    pub fn clone(&self) -> Self {
        Self::from_bars(self.bars.clone())
    }

    // #[inline]
    // pub fn remove_time_duplicates(&mut self) {
    //     let mut unique_bars = Vec::new();
    //     let mut seen_times = std::collections::HashSet::new();

    //     for bar in &self.bars {
    //         if !seen_times.contains(&bar.open_time) {
    //             unique_bars.push(*bar);
    //             seen_times.insert(bar.open_time);
    //         }
    //     }

    //     self.bars = unique_bars;
    // }
}

#[cfg(feature = "polars")]
impl Ohlcv {
    #[inline]
    pub fn from_polars(df: &DataFrame, time_unit: &str) -> Ohlcv {
        Self::from_bars(ohlcv_bars_from_polars(&df, time_unit))
    }

    #[inline]
    pub fn read_csv(path: &Path, time_unit: &str) -> Ohlcv {
        let df = read_df_csv(path);
        Self::from_polars(&df, time_unit)
    }

    #[inline]
    pub fn read_parquet(path: &Path, time_unit: &str) -> Ohlcv {
        let df = read_df_parquet(path);
        Self::from_polars(&df, time_unit)
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

#[cfg(feature = "bindings_wasm")]
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
    let open_time = open_time.map(|list| {
        list.iter()
            .map(|x| Some(DateTime::from(x)))
            .collect::<Vec<_>>()
    });
    let close_time = close_time.map(|list| {
        list.iter()
            .map(|x| Some(DateTime::from(x)))
            .collect::<Vec<_>>()
    });
    zip_ohlcv_bars(open_time, close_time, open, high, low, close, volume)
}

// implement unzip for js
// #[cfg(feature = "bindings_wasm")]
// #[wasm_bindgen(js_name=unzipOhlcvBars)]
// #[inline]
// pub fn js_unzip_ohlcv_bars(bars: Vec<OhlcvBar>) -> Vec<Vec<Option<js_sys::Date>>> {
//     let open_time = bars.iter().map(|bar| Some(js_sys::Date::new(&bar.open_time.to_string()))).collect::<Vec<_>>();
//     let close_time = bars.iter().map(|bar| Some(js_sys::Date::new(&bar.close_time.to_string()))).collect::<Vec<_>>();
//     vec![open_time, close_time]
// }

#[cfg(feature = "polars")]
#[inline]
pub fn ohlcv_bars_from_polars(df: &DataFrame, time_unit: &str) -> Vec<OhlcvBar> {
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
pub fn resample(bars: &[OhlcvBar], timeframe: Timeframe, align_utc: bool) -> Vec<OhlcvBar> {
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
        if aggregator.is_none() {
            aggregator = Some(*bar);
            bucket_start = Some(bar.open_time);
        } else if let (Some(agg), Some(start)) = (aggregator, bucket_start) {
            // Determine if this bar crosses the boundary
            let boundary = start + duration;
            if bar.open_time >= boundary {
                // close out current aggregator
                result.push(agg);
                // start a new aggregator
                aggregator = Some(*bar);
                bucket_start = Some(bar.open_time);
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
            agg.close_time = agg.open_time + offset;
        }

        // If you want hour bars pinned similarly, do the same for Hours(n).
        Timeframe::Hours(n) => {
            let offset = chrono::Duration::hours(*n as i64);
            agg.close_time = agg.open_time + offset;
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
        let bucket = floor_to_timeframe(bar.open_time, &timeframe);

        match aggregator {
            None => {
                // start a new aggregator, open_time pinned to the boundary
                let mut new_bar = *bar;
                new_bar.open_time = bucket;
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
                    new_bar.open_time = bucket;
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
