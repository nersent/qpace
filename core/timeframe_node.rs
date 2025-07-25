use crate::timeframe::Timeframe;
use chrono::Duration;
use napi::{Error, Result, Status};
use napi_derive::napi;

#[napi]
pub struct NodeTimeframe {
    inner: Timeframe,
}

impl From<Timeframe> for NodeTimeframe {
    fn from(inner: Timeframe) -> Self {
        NodeTimeframe { inner }
    }
}

impl Into<Timeframe> for &NodeTimeframe {
    fn into(self) -> Timeframe {
        self.inner
    }
}

#[napi]
impl NodeTimeframe {
    #[napi(js_name = toString)]
    #[inline]
    pub fn node_to_string(&self) -> String {
        self.inner.to_string()
    }

    #[napi(factory, js_name = fromString)]
    #[inline]
    pub fn node_from_string(timeframe: String) -> Self {
        Timeframe::from(timeframe).into()
    }

    #[napi(factory, js_name = Years)]
    #[inline]
    pub fn node_from_years(value: i32) -> Self {
        Timeframe::Years(value as usize).into()
    }

    #[napi(factory, js_name = Months)]
    #[inline]
    pub fn node_from_months(value: i32) -> Self {
        Timeframe::Months(value as usize).into()
    }

    #[napi(factory, js_name = Weeks)]
    #[inline]
    pub fn node_from_weeks(value: i32) -> Self {
        Timeframe::Weeks(value as usize).into()
    }

    #[napi(factory, js_name = Days)]
    #[inline]
    pub fn node_from_days(value: i32) -> Self {
        Timeframe::Days(value as usize).into()
    }

    #[napi(factory, js_name = Hours)]
    #[inline]
    pub fn node_from_hours(value: i32) -> Self {
        Timeframe::Hours(value as usize).into()
    }

    #[napi(factory, js_name = Minutes)]
    #[inline]
    pub fn node_from_minutes(value: i32) -> Self {
        Timeframe::Minutes(value as usize).into()
    }

    #[napi(factory, js_name = Seconds)]
    #[inline]
    pub fn node_from_seconds(value: i32) -> Self {
        Timeframe::Seconds(value as usize).into()
    }

    #[napi(factory, js_name = Ticks)]
    #[inline]
    pub fn node_from_ticks(value: i32) -> Self {
        Timeframe::Ticks(value as usize).into()
    }

    #[napi(factory, js_name = Ranges)]
    #[inline]
    pub fn node_from_ranges(value: i32) -> Self {
        Timeframe::Ranges(value as usize).into()
    }

    #[napi(factory, js_name = Unknown)]
    #[inline]
    pub fn node_from_unknown() -> Self {
        Timeframe::Unknown().into()
    }

    #[napi(getter = years)]
    #[inline]
    pub fn node_years(&self) -> Option<i32> {
        self.inner.years().map(|v| v as i32)
    }

    #[napi(getter = months)]
    #[inline]
    pub fn node_months(&self) -> Option<i32> {
        self.inner.months().map(|v| v as i32)
    }

    #[napi(getter = weeks)]
    #[inline]
    pub fn node_weeks(&self) -> Option<i32> {
        self.inner.weeks().map(|v| v as i32)
    }

    #[napi(getter = days)]
    #[inline]
    pub fn node_days(&self) -> Option<i32> {
        self.inner.days().map(|v| v as i32)
    }

    #[napi(getter = hours)]
    #[inline]
    pub fn node_hours(&self) -> Option<i32> {
        self.inner.hours().map(|v| v as i32)
    }

    #[napi(getter = minutes)]
    #[inline]
    pub fn node_minutes(&self) -> Option<i32> {
        self.inner.minutes().map(|v| v as i32)
    }

    #[napi(getter = seconds)]
    #[inline]
    pub fn node_seconds(&self) -> Option<i32> {
        self.inner.seconds().map(|v| v as i32)
    }

    #[napi(getter = ticks)]
    #[inline]
    pub fn node_ticks(&self) -> Option<i32> {
        self.inner.ticks().map(|v| v as i32)
    }

    #[napi(getter = ranges)]
    #[inline]
    pub fn node_ranges(&self) -> Option<i32> {
        self.inner.ranges().map(|v| v as i32)
    }

    #[napi(getter = unknown)]
    #[inline]
    pub fn node_unknown(&self) -> bool {
        self.inner.unknown()
    }

    #[napi(js_name = eq)]
    #[inline]
    pub fn node_eq(&self, other: &NodeTimeframe) -> bool {
        self.inner == other.inner
    }

    #[napi(js_name = toDurationS)]
    #[inline]
    pub fn node_to_duration_s(&self) -> Result<i32> {
        let duration = TryInto::<Duration>::try_into(self.inner);
        match duration {
            Ok(dur) => Ok(dur.num_seconds() as i32),
            Err(_) => Err(Error::new(Status::InvalidArg, "Invalid timeframe")),
        }
    }

    #[napi(factory, js_name = fromDurationS)]
    #[inline]
    pub fn node_from_duration_s(duration: i32) -> Self {
        let duration = Duration::seconds(duration as i64);
        let timeframe = TryInto::<Timeframe>::try_into(duration);
        match timeframe {
            Ok(timeframe) => timeframe.into(),
            Err(_) => Timeframe::Unknown().into(),
        }
    }
}
