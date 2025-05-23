cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
  use js_sys::{Object, Reflect};
  use crate::timeframe_js::{JsTimeframe};
  use js_sys::Float64Array;
  use js_sys::Array;
}}
use crate::ohlcv::{zip_ohlcv_bars, Ohlcv};
use crate::timeframe::Timeframe;
use crate::{
    ohlcv::{OhlcvBar, OhlcvReader, OhlcvWriter},
    rs_utils::get_oldest_possible_datetime,
};
use chrono::{DateTime, Utc};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::{ffi::OsStr, ops::Range, path::Path};

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(raw_module = "../../lib/internal.js")]
extern "C" {
    #[wasm_bindgen(js_name = readOhlcvBarsFromPath)]
    fn js_read_ohlcv_bars_from_path(format: &str, path: &str, time_unit: &str) -> Vec<OhlcvBar>;

    #[wasm_bindgen(js_name = writeOhlcvBarsToPath)]
    fn js_write_ohlcv_bars_to_path(format: &str, path: &str, bars: Vec<OhlcvBar>);
}

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(js_class=OhlcvBar)]
impl OhlcvBar {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn js_new(
        open_time: Option<js_sys::Date>,
        close_time: Option<js_sys::Date>,
        open: Option<f64>,
        high: Option<f64>,
        low: Option<f64>,
        close: Option<f64>,
        volume: Option<f64>,
    ) -> Self {
        let open_time = open_time
            .map(|x| x.into())
            .unwrap_or_else(|| get_oldest_possible_datetime());
        let close_time = close_time
            .map(|x| x.into())
            .unwrap_or_else(|| get_oldest_possible_datetime());
        let open = open.unwrap_or(f64::NAN);
        let high = high.unwrap_or(f64::NAN);
        let low = low.unwrap_or(f64::NAN);
        let close = close.unwrap_or(f64::NAN);
        let volume = volume.unwrap_or(f64::NAN);
        Self::new(open_time, close_time, open, high, low, close, volume)
    }

    #[wasm_bindgen(getter = openTime)]
    #[inline]
    pub fn js_open_time(&self) -> js_sys::Date {
        (*self.open_time()).into()
    }

    #[wasm_bindgen(getter = closeTime)]
    #[inline]
    pub fn js_close_time(&self) -> js_sys::Date {
        (*self.close_time()).into()
    }

    // #[wasm_bindgen(getter = openTimeMs)]
    // #[inline]
    // pub fn js_open_time_ms(&self) -> f64 {
    //     self.open_time().timestamp_millis() as f64
    // }

    // #[wasm_bindgen(getter = closeTimeMs)]
    // #[inline]
    // pub fn js_close_time_ms(&self) -> f64 {
    //     self.close_time().timestamp_millis() as f64
    // }

    #[wasm_bindgen(getter = open)]
    #[inline]
    pub fn js_open(&self) -> f64 {
        self.open()
    }

    #[wasm_bindgen(getter = high)]
    #[inline]
    pub fn js_high(&self) -> f64 {
        self.high()
    }

    #[wasm_bindgen(getter = low)]
    #[inline]
    pub fn js_low(&self) -> f64 {
        self.low()
    }

    #[wasm_bindgen(getter = close)]
    #[inline]
    pub fn js_close(&self) -> f64 {
        self.close()
    }

    #[wasm_bindgen(getter = volume)]
    #[inline]
    pub fn js_volume(&self) -> f64 {
        self.volume()
    }

    #[wasm_bindgen(getter = hl2)]
    #[inline]
    pub fn js_hl2(&self) -> f64 {
        self.hl2()
    }

    #[wasm_bindgen(getter = hlc3)]
    #[inline]
    pub fn js_hlc3(&self) -> f64 {
        self.hlc3()
    }

    #[wasm_bindgen(getter = hlcc4)]
    #[inline]
    pub fn js_hlcc4(&self) -> f64 {
        self.hlcc4()
    }

    #[wasm_bindgen(js_name = "toString")]
    #[inline]
    pub fn js_to_string(&self) -> String {
        self.fmt()
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn js_to_json(&self) -> JsValue {
        let obj = Object::new();
        let _ = Reflect::set(
            &obj,
            &"open_time".into(),
            &JsValue::from(self.js_open_time()),
        );
        let _ = Reflect::set(
            &obj,
            &"close_time".into(),
            &JsValue::from(self.js_close_time()),
        );
        let _ = Reflect::set(&obj, &"open".into(), &JsValue::from(self.js_open()));
        let _ = Reflect::set(&obj, &"high".into(), &JsValue::from(self.js_high()));
        let _ = Reflect::set(&obj, &"low".into(), &JsValue::from(self.js_low()));
        let _ = Reflect::set(&obj, &"close".into(), &JsValue::from(self.js_close()));
        let _ = Reflect::set(&obj, &"volume".into(), &JsValue::from(self.js_volume()));
        obj.into()
    }

    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn js_from_json(json: JsValue) -> Self {
        let obj = json.unchecked_into::<Object>();
        let open_time = Reflect::get(&obj, &"open_time".into())
            .unwrap()
            .unchecked_into::<js_sys::Date>();
        let close_time = Reflect::get(&obj, &"close_time".into())
            .unwrap()
            .unchecked_into::<js_sys::Date>();
        let open = Reflect::get(&obj, &"open".into())
            .unwrap()
            .as_f64()
            .unwrap();
        let high = Reflect::get(&obj, &"high".into())
            .unwrap()
            .as_f64()
            .unwrap();
        let low = Reflect::get(&obj, &"low".into()).unwrap().as_f64().unwrap();
        let close = Reflect::get(&obj, &"close".into())
            .unwrap()
            .as_f64()
            .unwrap();
        let volume = Reflect::get(&obj, &"volume".into())
            .unwrap()
            .as_f64()
            .unwrap();
        Self::new(
            open_time.into(),
            close_time.into(),
            open,
            high,
            low,
            close,
            volume,
        )
    }

    #[wasm_bindgen(js_name = "merge")]
    #[inline]
    pub fn js_merge(&self, other: &OhlcvBar) -> OhlcvBar {
        self.merge(other)
    }

    // #[wasm_bindgen(js_name = "hash")]
    // #[inline]
    // pub fn js_hash(&self) -> String {
    //     self.hash()
    // }
}

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(js_name = "Ohlcv")]
#[derive(Clone, Debug)]
pub struct JsOhlcv {
    inner: Rc<RefCell<Ohlcv>>,
    timeframe: JsTimeframe,
}

#[cfg(feature = "bindings_wasm")]
impl Into<JsOhlcv> for Ohlcv {
    #[inline]
    fn into(self) -> JsOhlcv {
        JsOhlcv {
            inner: Rc::new(RefCell::new(self)),
            timeframe: Timeframe::Unknown().into(),
        }
    }
}

#[cfg(feature = "bindings_wasm")]
impl Into<Ohlcv> for JsOhlcv {
    #[inline]
    fn into(self) -> Ohlcv {
        self.inner.borrow().clone()
    }
}

#[cfg(feature = "bindings_wasm")]
impl OhlcvReader for JsOhlcv {
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

#[cfg(feature = "bindings_wasm")]
impl OhlcvWriter for JsOhlcv {
    #[inline]
    fn push(&mut self, bar: OhlcvBar) {
        self.inner.borrow_mut().push(bar);
    }

    #[inline]
    fn push_many(&mut self, bars: &[OhlcvBar]) {
        self.inner.borrow_mut().push_many(bars);
    }
}

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(js_class=Ohlcv)]
impl JsOhlcv {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn js_new() -> Self {
        Ohlcv::empty().into()
    }

    #[wasm_bindgen(getter = timeframe)]
    #[inline]
    pub fn js_timeframe(&self) -> JsTimeframe {
        self.timeframe.clone()
    }

    #[wasm_bindgen(setter = timeframe)]
    #[inline]
    pub fn js_set_timeframe(&mut self, timeframe: JsTimeframe) {
        self.timeframe = timeframe;
    }

    #[wasm_bindgen(js_name = "fromBars")]
    #[inline]
    pub fn js_from_bars(bars: Vec<OhlcvBar>) -> Self {
        Ohlcv::from_bars(bars).into()
    }

    #[wasm_bindgen(getter = bars)]
    #[inline]
    pub fn js_bars(&self) -> Vec<OhlcvBar> {
        self.all_bars().to_vec()
    }

    #[wasm_bindgen(js_name = "at")]
    #[inline]
    pub fn js_bar_at(&self, index: i32) -> Option<OhlcvBar> {
        let index = if index < 0 {
            (self.len() as i32 + index) as usize
        } else {
            index as usize
        };
        if index >= self.len() {
            return None;
        }
        Some(*self.bar(index))
    }

    #[wasm_bindgen(getter = length)]
    #[inline]
    pub fn js_len(&self) -> usize {
        self.len()
    }

    #[wasm_bindgen(js_name = "toString")]
    #[inline]
    pub fn js_to_string(&self) -> String {
        format!("Ohlcv(len={})", self.len())
    }

    #[wasm_bindgen(getter = "open")]
    #[inline]
    pub fn js_open(&self) -> Vec<f64> {
        self.open()
    }

    #[wasm_bindgen(getter = "high")]
    #[inline]
    pub fn js_high(&self) -> Vec<f64> {
        self.high()
    }

    #[wasm_bindgen(getter = "low")]
    #[inline]
    pub fn js_low(&self) -> Vec<f64> {
        self.low()
    }

    #[wasm_bindgen(getter = "close")]
    #[inline]
    pub fn js_close(&self) -> Vec<f64> {
        self.close()
    }

    #[wasm_bindgen(getter = "volume")]
    #[inline]
    pub fn js_volume(&self) -> Vec<f64> {
        self.volume()
    }

    #[wasm_bindgen(getter = "openTime")]
    #[inline]
    pub fn js_open_time(&self) -> Vec<js_sys::Date> {
        self.open_time()
            .iter()
            .map(|x| (*x).into())
            .collect::<Vec<_>>()
    }

    #[wasm_bindgen(getter = "closeTime")]
    #[inline]
    pub fn js_close_time(&self) -> Vec<js_sys::Date> {
        self.close_time()
            .iter()
            .map(|x| (*x).into())
            .collect::<Vec<_>>()
    }

    // #[wasm_bindgen(getter = "openTimeMs")]
    // #[inline]
    // pub fn js_open_time_ms(&self) -> Vec<f64> {
    //     self.open_time_ms().iter().map(|x| *x as f64).collect()
    // }

    // #[wasm_bindgen(getter = "closeTimeMs")]
    // #[inline]
    // pub fn js_close_time_ms(&self) -> Vec<f64> {
    //     self.close_time_ms().iter().map(|x| *x as f64).collect()
    // }

    #[wasm_bindgen(js_name = "add")]
    #[inline]
    pub fn js_add(&mut self, bar: OhlcvBar) {
        self.push(bar);
    }

    #[wasm_bindgen(js_name = "addList")]
    #[inline]
    pub fn js_add_list(&mut self, bars: Vec<OhlcvBar>) {
        self.push_many(&bars);
    }

    #[wasm_bindgen(js_name = "readCSV")]
    #[inline]
    #[doc = "`time_unit: 'ms' | 's`. Default: 's'"]
    pub fn js_read_csv(path: &str, time_unit: Option<String>) -> JsOhlcv {
        let time_unit = time_unit.unwrap_or("s".to_string());
        let bars = js_read_ohlcv_bars_from_path("csv", path, &time_unit);
        Ohlcv::from_bars(bars).into()
    }

    #[wasm_bindgen(js_name = "readParquet")]
    #[inline]
    #[doc = "`time_unit: 'ms' | 's`. Default: 's'"]
    pub fn js_read_parquet(path: &str, time_unit: Option<String>) -> JsOhlcv {
        let time_unit = time_unit.unwrap_or("s".to_string());
        let bars = js_read_ohlcv_bars_from_path("parquet", path, &time_unit);
        Ohlcv::from_bars(bars).into()
    }

    #[wasm_bindgen(js_name = "writeCSV")]
    #[inline]
    pub fn js_write_csv(&self, path: &str) {
        let bars = self.all_bars();
        js_write_ohlcv_bars_to_path("csv", path, bars.to_vec());
    }

    #[wasm_bindgen(js_name = "writeParquet")]
    #[inline]
    pub fn js_write_parquet(&self, path: &str) {
        let bars = self.all_bars();
        js_write_ohlcv_bars_to_path("parquet", path, bars.to_vec());
    }

    #[wasm_bindgen(js_name = "resample")]
    #[inline]
    #[doc = "Resamples OHLCV bars into the specified timeframe.
If align_utc is true, bars are pinned to calendar-based UTC boundaries;
otherwise, a rolling time window is used.\n`align_utc`: boolean. Default: true"]
    pub fn js_resample(&self, timeframe: JsTimeframe, align_utc: Option<bool>) -> JsOhlcv {
        let align_utc = align_utc.unwrap_or(true);
        let timeframe: Timeframe = timeframe.into();
        self.inner.borrow().resample(timeframe, align_utc).into()
    }

    #[wasm_bindgen(js_name = "sort")]
    #[inline]
    #[doc = "Sorts the OHLCV in-place based on time. \n`ord`: 'asc' | 'desc'. Default: 'asc'"]
    pub fn js_sort(&self, ord: Option<String>) {
        let ord = ord.unwrap_or("asc".to_string());
        self.inner.borrow_mut().sort(ord.into());
    }

    #[wasm_bindgen(js_name = "extend")]
    #[inline]
    #[doc = "Merges data from the other OHLCV into this one."]
    pub fn js_extend(&self, other: &JsOhlcv) {
        self.inner
            .borrow_mut()
            .push_many(&other.inner.borrow().bars);
    }

    #[wasm_bindgen(js_name = "clone")]
    #[inline]
    pub fn js_clone(&self) -> JsOhlcv {
        let ohlcv = self.inner.borrow_mut().clone();
        let mut ohlcv: JsOhlcv = ohlcv.into();
        ohlcv.timeframe = self.timeframe.clone();
        ohlcv
    }

    // #[wasm_bindgen(js_name = "removeDuplicates")]
    // #[inline]
    // pub fn js_remove_duplicates(&self, other: &JsOhlcv) {
    //     self.inner
    //         .borrow_mut()
    //         .
    // }
}
