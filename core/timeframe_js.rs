cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}
use crate::{rs_utils::get_oldest_possible_datetime, timeframe::Timeframe};

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(js_name = "Timeframe")]
#[derive(Debug, Clone)]
pub struct JsTimeframe {
    inner: Timeframe,
}

#[cfg(feature = "bindings_wasm")]
impl From<Timeframe> for JsTimeframe {
    fn from(inner: Timeframe) -> Self {
        JsTimeframe { inner }
    }
}

#[cfg(feature = "bindings_wasm")]
impl Into<Timeframe> for JsTimeframe {
    fn into(self) -> Timeframe {
        self.inner
    }
}

#[cfg(feature = "bindings_wasm")]
#[wasm_bindgen(js_class = Timeframe)]
impl JsTimeframe {
    #[wasm_bindgen(js_name = "fromString")]
    #[inline]
    pub fn js_from_string(timeframe: String) -> Self {
        Timeframe::from(timeframe).into()
    }

    #[wasm_bindgen(js_name = toString)]
    #[inline]
    pub fn js_to_string(&self) -> String {
        self.inner.into()
    }

    #[wasm_bindgen(js_name = years)]
    #[inline]
    pub fn js_from_years(value: usize) -> JsTimeframe {
        Timeframe::Years(value).into()
    }

    #[wasm_bindgen(js_name = months)]
    #[inline]
    pub fn js_from_months(value: usize) -> JsTimeframe {
        Timeframe::Months(value).into()
    }

    #[wasm_bindgen(js_name = weeks)]
    #[inline]
    pub fn js_from_weeks(value: usize) -> JsTimeframe {
        Timeframe::Weeks(value).into()
    }

    #[wasm_bindgen(js_name = days)]
    #[inline]
    pub fn js_from_days(value: usize) -> JsTimeframe {
        Timeframe::Days(value).into()
    }

    #[wasm_bindgen(js_name = hours)]
    #[inline]
    pub fn js_from_hours(value: usize) -> JsTimeframe {
        Timeframe::Hours(value).into()
    }

    #[wasm_bindgen(js_name = minutes)]
    #[inline]
    pub fn js_from_minutes(value: usize) -> JsTimeframe {
        Timeframe::Minutes(value).into()
    }

    #[wasm_bindgen(js_name = seconds)]
    #[inline]
    pub fn js_from_seconds(value: usize) -> JsTimeframe {
        Timeframe::Seconds(value).into()
    }

    #[wasm_bindgen(js_name = ticks)]
    #[inline]
    pub fn js_from_ticks(value: usize) -> JsTimeframe {
        Timeframe::Ticks(value).into()
    }

    #[wasm_bindgen(js_name = ranges)]
    #[inline]
    pub fn js_from_ranges(value: usize) -> JsTimeframe {
        Timeframe::Ranges(value).into()
    }

    #[wasm_bindgen(js_name = unknown)]
    #[inline]
    pub fn js_from_unknown() -> JsTimeframe {
        Timeframe::Unknown().into()
    }

    #[wasm_bindgen(getter = years)]
    #[inline]
    pub fn js_years(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Years(value) => Some(value),
            _ => None,
        }
    }

    #[wasm_bindgen(getter = months)]
    #[inline]
    pub fn js_months(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Months(value) => Some(value),
            _ => None,
        }
    }

    #[wasm_bindgen(getter = weeks)]
    #[inline]
    pub fn js_weeks(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Weeks(value) => Some(value),
            _ => None,
        }
    }

    #[wasm_bindgen(getter = days)]
    #[inline]
    pub fn js_days(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Days(value) => Some(value),
            _ => None,
        }
    }

    #[wasm_bindgen(getter = hours)]
    #[inline]
    pub fn js_hours(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Hours(value) => Some(value),
            _ => None,
        }
    }

    #[wasm_bindgen(getter = minutes)]
    #[inline]
    pub fn js_minutes(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Minutes(value) => Some(value),
            _ => None,
        }
    }

    #[wasm_bindgen(getter = seconds)]
    #[inline]
    pub fn js_seconds(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Seconds(value) => Some(value),
            _ => None,
        }
    }

    #[wasm_bindgen(getter = ticks)]
    #[inline]
    pub fn js_ticks(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Ticks(value) => Some(value),
            _ => None,
        }
    }

    #[wasm_bindgen(getter = ranges)]
    #[inline]
    pub fn js_ranges(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Ranges(value) => Some(value),
            _ => None,
        }
    }

    #[wasm_bindgen(getter = unknown)]
    #[inline]
    pub fn js_unknown(&self) -> bool {
        matches!(self.inner, Timeframe::Unknown())
    }

    #[wasm_bindgen(js_name = eq)]
    #[inline]
    pub fn js_eq(&self, other: JsTimeframe) -> bool {
        self.inner == other.into()
    }

    #[wasm_bindgen(js_name = ne)]
    #[inline]
    pub fn js_ne(&self, other: JsTimeframe) -> bool {
        self.inner != other.into()
    }
}
