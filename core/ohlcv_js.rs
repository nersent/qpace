cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}
use crate::{
    ohlcv::{ArcOhlcv, OhlcvBar, OhlcvLoader, OhlcvReader, OhlcvWriter},
    rs_utils::get_oldest_possible_datetime,
};
use chrono::{DateTime, Utc};

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
}

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(js_class=ArcOhlcv)]
impl ArcOhlcv {
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
        self.fmt()
    }

    #[wasm_bindgen(js_name = "fromBars")]
    #[inline]
    pub fn js_from_bars(bars: Vec<OhlcvBar>) -> Self {
        Self::from_bars(bars)
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
}

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(js_class=Ohlcv)]
impl OhlcvLoader {
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
        self.fmt()
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

    #[wasm_bindgen(js_name = "fromBars")]
    #[inline]
    pub fn js_from_bars(bars: Vec<OhlcvBar>) -> Self {
        Self::from_bars(bars)
    }
}
