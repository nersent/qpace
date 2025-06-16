use crate::ohlcv::Ohlcv;
use crate::ohlcv::OhlcvReaderOps;
use crate::ohlcv::OhlcvWriterOps;
use crate::ohlcv::RcOhlcv;
use crate::ohlcv::{OhlcvBar, OhlcvReader, OhlcvWriter};
use crate::timeframe_wasm::WasmTimeframe;
use wasm_bindgen::prelude::*;
cfg_if::cfg_if! { if #[cfg(target_arch = "wasm32")] {
use crate::ohlcv::zip_ohlcv_bars;
use chrono::DateTime;
use chrono::Utc;
use js_sys::Object;
use js_sys::Reflect;
use js_sys::Array;
}}

#[wasm_bindgen(js_class=OhlcvBar)]
impl OhlcvBar {
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn wasm_new(
        open_time: Option<js_sys::Date>,
        close_time: Option<js_sys::Date>,
        open: Option<f64>,
        high: Option<f64>,
        low: Option<f64>,
        close: Option<f64>,
        volume: Option<f64>,
    ) -> Self {
        Self::new(
            open_time.map(|d| d.into()),
            close_time.map(|d| d.into()),
            open.unwrap_or(f64::NAN),
            high.unwrap_or(f64::NAN),
            low.unwrap_or(f64::NAN),
            close.unwrap_or(f64::NAN),
            volume.unwrap_or(f64::NAN),
        )
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(getter = openTime)]
    #[inline]
    pub fn wasm_open_time(&self) -> Option<js_sys::Date> {
        self.open_time().copied().map(|d| d.into())
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(getter = closeTime)]
    #[inline]
    pub fn wasm_close_time(&self) -> Option<js_sys::Date> {
        self.close_time().copied().map(|d| d.into())
    }

    #[wasm_bindgen(getter = open)]
    #[inline]
    pub fn wasm_open(&self) -> f64 {
        self.open()
    }

    #[wasm_bindgen(getter = high)]
    #[inline]
    pub fn wasm_high(&self) -> f64 {
        self.high()
    }

    #[wasm_bindgen(getter = low)]
    #[inline]
    pub fn wasm_low(&self) -> f64 {
        self.low()
    }

    #[wasm_bindgen(getter = close)]
    #[inline]
    pub fn wasm_close(&self) -> f64 {
        self.close()
    }

    #[wasm_bindgen(getter = volume)]
    #[inline]
    pub fn wasm_volume(&self) -> f64 {
        self.volume()
    }

    #[wasm_bindgen(js_name = "merge")]
    #[inline]
    pub fn wasm_merge(&self, other: &Self) -> Self {
        self.merge(other)
    }

    #[wasm_bindgen(js_name = "toString")]
    #[inline]
    pub fn wasm_to_string(&self) -> String {
        format!("{:?}", self)
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn wasm_to_json(&self) -> JsValue {
        let obj = Object::new();
        let open_time_val = match self.wasm_open_time() {
            Some(date) => JsValue::from(date),
            None => JsValue::NULL,
        };
        Reflect::set(&obj, &JsValue::from_str("open_time"), &open_time_val).unwrap();
        let close_time_val = match self.wasm_close_time() {
            Some(date) => JsValue::from(date),
            None => JsValue::NULL,
        };
        Reflect::set(&obj, &JsValue::from_str("close_time"), &close_time_val).unwrap();
        Reflect::set(
            &obj,
            &JsValue::from_str("open"),
            &JsValue::from_f64(self.wasm_open()),
        )
        .unwrap();
        Reflect::set(
            &obj,
            &JsValue::from_str("high"),
            &JsValue::from_f64(self.wasm_high()),
        )
        .unwrap();
        Reflect::set(
            &obj,
            &JsValue::from_str("low"),
            &JsValue::from_f64(self.wasm_low()),
        )
        .unwrap();
        Reflect::set(
            &obj,
            &JsValue::from_str("close"),
            &JsValue::from_f64(self.wasm_close()),
        )
        .unwrap();
        Reflect::set(
            &obj,
            &JsValue::from_str("volume"),
            &JsValue::from_f64(self.wasm_volume()),
        )
        .unwrap();
        obj.into()
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn wasm_from_json(json: JsValue) -> Self {
        let obj = json.unchecked_into::<Object>();
        let open_time_val = Reflect::get(&obj, &JsValue::from_str("openTime")).unwrap();
        let open_time = if open_time_val.is_null() || open_time_val.is_undefined() {
            None
        } else {
            Some(open_time_val.unchecked_into::<js_sys::Date>().into())
        };
        let close_time_val = Reflect::get(&obj, &JsValue::from_str("closeTime")).unwrap();
        let close_time = if close_time_val.is_null() || close_time_val.is_undefined() {
            None
        } else {
            Some(close_time_val.unchecked_into::<js_sys::Date>().into())
        };
        let open = Reflect::get(&obj, &JsValue::from_str("open"))
            .unwrap()
            .as_f64()
            .unwrap_or(f64::NAN);
        let high = Reflect::get(&obj, &JsValue::from_str("high"))
            .unwrap()
            .as_f64()
            .unwrap_or(f64::NAN);
        let low = Reflect::get(&obj, &JsValue::from_str("low"))
            .unwrap()
            .as_f64()
            .unwrap_or(f64::NAN);
        let close = Reflect::get(&obj, &JsValue::from_str("close"))
            .unwrap()
            .as_f64()
            .unwrap_or(f64::NAN);
        let volume = Reflect::get(&obj, &JsValue::from_str("volume"))
            .unwrap()
            .as_f64()
            .unwrap_or(f64::NAN);
        Self::new(open_time, close_time, open, high, low, close, volume)
    }
}

#[wasm_bindgen(js_name = "Ohlcv")]
#[derive(Clone, Debug)]
pub struct WasmOhlcv {
    inner: RcOhlcv,
}

impl Default for WasmOhlcv {
    #[inline]
    fn default() -> Self {
        Ohlcv::new().into()
    }
}

impl Into<WasmOhlcv> for Ohlcv {
    #[inline]
    fn into(self) -> WasmOhlcv {
        WasmOhlcv { inner: self.into() }
    }
}

impl Into<Ohlcv> for WasmOhlcv {
    #[inline]
    fn into(self) -> Ohlcv {
        self.inner.into()
    }
}

impl Into<RcOhlcv> for WasmOhlcv {
    #[inline]
    fn into(self) -> RcOhlcv {
        self.inner
    }
}

impl Into<WasmOhlcv> for RcOhlcv {
    #[inline]
    fn into(self) -> WasmOhlcv {
        WasmOhlcv { inner: self }
    }
}

#[wasm_bindgen(js_class=Ohlcv)]
impl WasmOhlcv {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn wasm_new() -> Self {
        Ohlcv::new().into()
    }

    #[wasm_bindgen(js_name = "fromBars")]
    #[inline]
    pub fn wasm_from_bars(bars: Vec<OhlcvBar>) -> Self {
        Ohlcv::from_bars(bars).into()
    }

    #[wasm_bindgen(getter = timeframe)]
    #[inline]
    pub fn wasm_timeframe(&self) -> WasmTimeframe {
        self.inner.timeframe().into()
    }

    #[wasm_bindgen(setter = timeframe)]
    #[inline]
    pub fn wasm_set_timeframe(&mut self, timeframe: WasmTimeframe) {
        self.inner.set_timeframe(timeframe.into());
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(getter = openTime)]
    pub fn wasm_open_time(&self) -> Array {
        let mut arr = Array::new();
        for opt in self.inner.open_time() {
            let js_val = match opt {
                Some(dt) => js_sys::Date::from(dt).into(),
                None => JsValue::NULL,
            };
            arr.push(&js_val);
        }
        arr
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(getter = closeTime)]
    pub fn wasm_close_time(&self) -> Array {
        let mut arr = Array::new();
        for opt in self.inner.close_time() {
            let js_val = match opt {
                Some(dt) => js_sys::Date::from(dt).into(),
                None => JsValue::NULL,
            };
            arr.push(&js_val);
        }
        arr
    }

    #[wasm_bindgen(getter = "open")]
    #[inline]
    pub fn wasm_open(&self) -> Vec<f64> {
        self.inner.open()
    }

    #[wasm_bindgen(getter = "high")]
    #[inline]
    pub fn wasm_high(&self) -> Vec<f64> {
        self.inner.high()
    }

    #[wasm_bindgen(getter = "low")]
    #[inline]
    pub fn wasm_low(&self) -> Vec<f64> {
        self.inner.low()
    }

    #[wasm_bindgen(getter = "close")]
    #[inline]
    pub fn wasm_close(&self) -> Vec<f64> {
        self.inner.close()
    }

    #[wasm_bindgen(getter = "volume")]
    #[inline]
    pub fn wasm_volume(&self) -> Vec<f64> {
        self.inner.volume()
    }

    #[wasm_bindgen(js_name = "bars")]
    #[inline]
    pub fn wasm_bars(&self) -> Vec<OhlcvBar> {
        self.inner.bars()
    }

    #[wasm_bindgen(js_name = "at")]
    #[inline]
    pub fn wasm_at(&self, index: i32) -> Option<OhlcvBar> {
        self.inner.at(index)
    }

    #[wasm_bindgen(getter = length)]
    #[inline]
    pub fn wasm_length(&self) -> usize {
        self.inner.len()
    }

    #[wasm_bindgen(js_name = "slice")]
    #[inline]
    pub fn wasm_slice(&self, start: usize, end: usize) -> Self {
        let mut ohlcv = Ohlcv::from_bars(self.inner.slice(start..end));
        ohlcv.set_timeframe(self.inner.timeframe().into());
        return ohlcv.into();
    }

    #[wasm_bindgen(js_name = "head")]
    #[inline]
    pub fn wasm_head(&self, count: usize) -> Self {
        let mut ohlcv = Ohlcv::from_bars(self.inner.head(count));
        ohlcv.set_timeframe(self.inner.timeframe().into());
        return ohlcv.into();
    }

    #[wasm_bindgen(js_name = "tail")]
    #[inline]
    pub fn wasm_tail(&self, count: usize) -> Self {
        let mut ohlcv = Ohlcv::from_bars(self.inner.tail(count));
        ohlcv.set_timeframe(self.inner.timeframe().into());
        return ohlcv.into();
    }

    #[wasm_bindgen(js_name = "copy")]
    #[inline]
    pub fn wasm_copy(&self) -> Self {
        self.inner.copy().into()
    }

    #[wasm_bindgen(js_name = "extend")]
    #[inline]
    pub fn wasm_extend(&mut self, other: &Self) {
        self.inner.extend(&other.inner);
    }

    #[wasm_bindgen(js_name = "resample")]
    #[inline]
    pub fn wasm_resample(&self, timeframe: WasmTimeframe, align_utc: bool) -> Self {
        self.inner.resample(timeframe.into(), align_utc).into()
    }

    #[wasm_bindgen(js_name = "sort")]
    #[inline]
    pub fn wasm_sort(&mut self, ascending: bool) {
        self.inner.sort(ascending);
    }

    #[wasm_bindgen(js_name = "reverse")]
    #[inline]
    pub fn wasm_reverse(&mut self) {
        self.inner.reverse();
    }

    #[wasm_bindgen(js_name = "clear")]
    #[inline]
    pub fn wasm_clear(&mut self) {
        self.inner.clear();
    }

    #[wasm_bindgen(js_name = "pop")]
    #[inline]
    pub fn wasm_pop(&mut self) -> Option<OhlcvBar> {
        self.inner.pop()
    }

    #[wasm_bindgen(js_name = "shift")]
    #[inline]
    pub fn wasm_shift(&mut self) -> Option<OhlcvBar> {
        self.inner.shift()
    }

    #[wasm_bindgen(js_name = "push")]
    #[inline]
    pub fn wasm_push(&mut self, bar: OhlcvBar) {
        self.inner.push(bar);
    }

    #[wasm_bindgen(js_name = "pushMany")]
    #[inline]
    pub fn wasm_push_many(&mut self, bars: Vec<OhlcvBar>) {
        self.inner.push_many(bars);
    }

    #[wasm_bindgen(js_name = "toString")]
    #[inline]
    pub fn wasm_to_string(&self) -> String {
        format!("{:?}", self.inner)
    }

    #[wasm_bindgen(js_name = "sanityCheck")]
    #[inline]
    pub fn wasm_sanity_check(&self) -> Vec<String> {
        match self.inner.sanity_check() {
            Ok(_) => vec![],
            Err(e) => e,
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn js_value_to_option_datetime(js_value: &JsValue) -> Option<DateTime<Utc>> {
    if js_value.is_null() || js_value.is_undefined() {
        None
    } else {
        js_value
            .as_f64()
            .map(|t| DateTime::<Utc>::from_timestamp(t as i64, 0).unwrap())
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "zipOhlcvBars")]
#[inline]
pub fn wasm_zip_ohlcv_bars(
    open_time: Option<Vec<JsValue>>,
    close_time: Option<Vec<JsValue>>,
    open: Option<Vec<f64>>,
    high: Option<Vec<f64>>,
    low: Option<Vec<f64>>,
    close: Option<Vec<f64>>,
    volume: Option<Vec<f64>>,
) -> Vec<OhlcvBar> {
    let open_time: Option<Vec<Option<DateTime<Utc>>>> = open_time.map(|v| {
        v.into_iter()
            .map(|v| js_value_to_option_datetime(&v))
            .collect()
    });
    let close_time: Option<Vec<Option<DateTime<Utc>>>> = close_time.map(|v| {
        v.into_iter()
            .map(|v| js_value_to_option_datetime(&v))
            .collect()
    });
    return zip_ohlcv_bars(open_time, close_time, open, high, low, close, volume);
}
