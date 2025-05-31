use crate::timeframe::Timeframe;
use chrono::Duration;
use js_sys::Error;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "Timeframe")]
#[derive(Debug, Clone)]
pub struct WasmTimeframe {
    inner: Timeframe,
}

impl From<Timeframe> for WasmTimeframe {
    fn from(inner: Timeframe) -> Self {
        WasmTimeframe { inner }
    }
}

impl Into<Timeframe> for WasmTimeframe {
    fn into(self) -> Timeframe {
        self.inner
    }
}

#[wasm_bindgen(js_class = Timeframe)]
impl WasmTimeframe {
    #[wasm_bindgen(js_name = toString)]
    #[inline]
    pub fn wasm_to_string(&self) -> String {
        self.inner.into()
    }

    #[wasm_bindgen(js_name = fromString)]
    #[inline]
    pub fn wasm_from_string(timeframe: String) -> Self {
        Timeframe::from(timeframe).into()
    }

    #[wasm_bindgen(js_name = years)]
    #[inline]
    pub fn wasm_from_years(value: usize) -> Self {
        Timeframe::Years(value).into()
    }

    #[wasm_bindgen(js_name = months)]
    #[inline]
    pub fn wasm_from_months(value: usize) -> Self {
        Timeframe::Months(value).into()
    }

    #[wasm_bindgen(js_name = weeks)]
    #[inline]
    pub fn wasm_from_weeks(value: usize) -> Self {
        Timeframe::Weeks(value).into()
    }

    #[wasm_bindgen(js_name = days)]
    #[inline]
    pub fn wasm_from_days(value: usize) -> Self {
        Timeframe::Days(value).into()
    }

    #[wasm_bindgen(js_name = hours)]
    #[inline]
    pub fn wasm_from_hours(value: usize) -> Self {
        Timeframe::Hours(value).into()
    }

    #[wasm_bindgen(js_name = minutes)]
    #[inline]
    pub fn wasm_from_minutes(value: usize) -> Self {
        Timeframe::Minutes(value).into()
    }

    #[wasm_bindgen(js_name = seconds)]
    #[inline]
    pub fn wasm_from_seconds(value: usize) -> Self {
        Timeframe::Seconds(value).into()
    }

    #[wasm_bindgen(js_name = ticks)]
    #[inline]
    pub fn wasm_from_ticks(value: usize) -> Self {
        Timeframe::Ticks(value).into()
    }

    #[wasm_bindgen(js_name = ranges)]
    #[inline]
    pub fn wasm_from_ranges(value: usize) -> Self {
        Timeframe::Ranges(value).into()
    }

    #[wasm_bindgen(js_name = unknown)]
    #[inline]
    pub fn wasm_from_unknown() -> Self {
        Timeframe::Unknown().into()
    }

    #[wasm_bindgen(getter = years)]
    #[inline]
    pub fn wasm_years(&self) -> Option<usize> {
        self.inner.years()
    }

    #[wasm_bindgen(getter = months)]
    #[inline]
    pub fn wasm_months(&self) -> Option<usize> {
        self.inner.months()
    }

    #[wasm_bindgen(getter = weeks)]
    #[inline]
    pub fn wasm_weeks(&self) -> Option<usize> {
        self.inner.weeks()
    }

    #[wasm_bindgen(getter = days)]
    #[inline]
    pub fn wasm_days(&self) -> Option<usize> {
        self.inner.days()
    }

    #[wasm_bindgen(getter = hours)]
    #[inline]
    pub fn wasm_hours(&self) -> Option<usize> {
        self.inner.hours()
    }

    #[wasm_bindgen(getter = minutes)]
    #[inline]
    pub fn wasm_minutes(&self) -> Option<usize> {
        self.inner.minutes()
    }

    #[wasm_bindgen(getter = seconds)]
    #[inline]
    pub fn wasm_seconds(&self) -> Option<usize> {
        self.inner.seconds()
    }

    #[wasm_bindgen(getter = ticks)]
    #[inline]
    pub fn wasm_ticks(&self) -> Option<usize> {
        self.inner.ticks()
    }

    #[wasm_bindgen(getter = ranges)]
    #[inline]
    pub fn wasm_ranges(&self) -> Option<usize> {
        self.inner.ranges()
    }

    #[wasm_bindgen(getter = unknown)]
    #[inline]
    pub fn wasm_unknown(&self) -> bool {
        self.inner.unknown()
    }

    #[wasm_bindgen(js_name = eq)]
    #[inline]
    pub fn wasm_eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    #[wasm_bindgen(js_name = toDurationS)]
    #[inline]
    pub fn wasm_to_duration_s(&self) -> Result<i32, Error> {
        let duration = TryInto::<Duration>::try_into(self.inner);
        match duration {
            Ok(duration) => Ok(duration.num_seconds() as i32),
            Err(_) => Err(Error::new("Invalid timeframe").into()),
        }
    }

    #[wasm_bindgen(js_name = fromDurationS)]
    #[inline]
    pub fn wasm_from_duration_s(duration: i32) -> Self {
        let duration = Duration::seconds(duration as i64);
        let timeframe = TryInto::<Timeframe>::try_into(duration);
        match timeframe {
            Ok(timeframe) => timeframe.into(),
            Err(_) => Timeframe::Unknown().into(),
        }
    }
}
