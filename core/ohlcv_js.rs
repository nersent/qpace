cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
  use js_sys::{Object, Reflect};
  use crate::timeframe_js::{JsTimeframe};
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

    #[wasm_bindgen(getter = openTimeMs)]
    #[inline]
    pub fn js_open_time_ms(&self) -> f64 {
        self.open_time().timestamp_millis() as f64
    }

    #[wasm_bindgen(getter = closeTimeMs)]
    #[inline]
    pub fn js_close_time_ms(&self) -> f64 {
        self.close_time().timestamp_millis() as f64
    }

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
            &"openTime".into(),
            &JsValue::from(self.js_open_time()),
        );
        let _ = Reflect::set(
            &obj,
            &"closeTime".into(),
            &JsValue::from(self.js_close_time()),
        );
        let _ = Reflect::set(&obj, &"open".into(), &JsValue::from(self.js_open()));
        let _ = Reflect::set(&obj, &"high".into(), &JsValue::from(self.js_high()));
        let _ = Reflect::set(&obj, &"low".into(), &JsValue::from(self.js_low()));
        let _ = Reflect::set(&obj, &"close".into(), &JsValue::from(self.js_close()));
        let _ = Reflect::set(&obj, &"volume".into(), &JsValue::from(self.js_volume()));
        obj.into()
    }
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

    #[wasm_bindgen(js_name = "bar")]
    #[inline]
    pub fn js_bar(&self, index: usize) -> OhlcvBar {
        *self.bar(index)
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

    #[wasm_bindgen(js_name = "open")]
    #[inline]
    pub fn js_open(&self) -> Vec<f64> {
        self.open()
    }

    #[wasm_bindgen(js_name = "high")]
    #[inline]
    pub fn js_high(&self) -> Vec<f64> {
        self.high()
    }

    #[wasm_bindgen(js_name = "low")]
    #[inline]
    pub fn js_low(&self) -> Vec<f64> {
        self.low()
    }

    #[wasm_bindgen(js_name = "close")]
    #[inline]
    pub fn js_close(&self) -> Vec<f64> {
        self.close()
    }

    #[wasm_bindgen(js_name = "volume")]
    #[inline]
    pub fn js_volume(&self) -> Vec<f64> {
        self.volume()
    }

    #[wasm_bindgen(js_name = "openTime")]
    #[inline]
    pub fn js_open_time(&self) -> Vec<js_sys::Date> {
        self.open_time()
            .iter()
            .map(|x| (*x).into())
            .collect::<Vec<_>>()
    }

    #[wasm_bindgen(js_name = "closeTime")]
    #[inline]
    pub fn js_close_time(&self) -> Vec<js_sys::Date> {
        self.close_time()
            .iter()
            .map(|x| (*x).into())
            .collect::<Vec<_>>()
    }

    #[wasm_bindgen(js_name = "openTimeMs")]
    #[inline]
    pub fn js_open_time_ms(&self) -> Vec<f64> {
        self.open_time_ms().iter().map(|x| *x as f64).collect()
    }

    #[wasm_bindgen(js_name = "closeTimeMs")]
    #[inline]
    pub fn js_close_time_ms(&self) -> Vec<f64> {
        self.close_time_ms().iter().map(|x| *x as f64).collect()
    }

    #[wasm_bindgen(js_name = "push")]
    #[inline]
    pub fn js_push(&mut self, bar: OhlcvBar) {
        self.push(bar);
    }

    #[wasm_bindgen(js_name = "pushMany")]
    #[inline]
    pub fn js_push_many(&mut self, bars: Vec<OhlcvBar>) {
        self.push_many(&bars);
    }
}
