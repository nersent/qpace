use crate::timeframe::Timeframe;
use chrono::Duration;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[gen_stub_pyclass]
#[pyclass(name = "Timeframe")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyTimeframe {
    inner: Timeframe,
}

impl Default for PyTimeframe {
    #[inline]
    fn default() -> Self {
        Timeframe::default().into()
    }
}

impl Into<Timeframe> for PyTimeframe {
    #[inline]
    fn into(self) -> Timeframe {
        self.inner
    }
}

impl From<Timeframe> for PyTimeframe {
    #[inline]
    fn from(inner: Timeframe) -> Self {
        PyTimeframe { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyTimeframe {
    #[pyo3(name = "__str__")]
    #[inline]
    pub fn py_str(&self) -> String {
        self.inner.into()
    }

    #[staticmethod]
    #[pyo3(name = "from_str")]
    #[inline]
    pub fn py_from_str(timeframe: String) -> Self {
        Timeframe::from(timeframe).into()
    }

    #[pyo3(name = "__repr__")]
    #[inline]
    pub fn py_repr(&self) -> String {
        format!("{:?}", self.inner)
    }

    #[staticmethod]
    #[pyo3(name = "Years")]
    #[inline]
    pub fn py_from_years(value: usize) -> Self {
        Timeframe::Years(value).into()
    }

    #[staticmethod]
    #[pyo3(name = "Months")]
    #[inline]
    pub fn py_from_months(value: usize) -> Self {
        Timeframe::Months(value).into()
    }

    #[staticmethod]
    #[pyo3(name = "Weeks")]
    #[inline]
    pub fn py_from_weeks(value: usize) -> Self {
        Timeframe::Weeks(value).into()
    }

    #[staticmethod]
    #[pyo3(name = "Days")]
    #[inline]
    pub fn py_from_days(value: usize) -> Self {
        Timeframe::Days(value).into()
    }

    #[staticmethod]
    #[pyo3(name = "Hours")]
    #[inline]
    pub fn py_from_hours(value: usize) -> Self {
        Timeframe::Hours(value).into()
    }

    #[staticmethod]
    #[pyo3(name = "Minutes")]
    #[inline]
    pub fn py_from_minutes(value: usize) -> Self {
        Timeframe::Minutes(value).into()
    }

    #[staticmethod]
    #[pyo3(name = "Seconds")]
    #[inline]
    pub fn py_from_seconds(value: usize) -> Self {
        Timeframe::Seconds(value).into()
    }

    #[staticmethod]
    #[pyo3(name = "Ticks")]
    #[inline]
    pub fn py_from_ticks(value: usize) -> Self {
        Timeframe::Ticks(value).into()
    }

    #[staticmethod]
    #[pyo3(name = "Ranges")]
    #[inline]
    pub fn py_from_ranges(value: usize) -> Self {
        Timeframe::Ranges(value).into()
    }

    #[staticmethod]
    #[pyo3(name = "Unknown")]
    #[inline]
    pub fn py_from_unknown() -> Self {
        Timeframe::Unknown().into()
    }

    #[getter(years)]
    #[inline]
    pub fn py_years(&self) -> Option<usize> {
        self.inner.years()
    }

    #[getter(months)]
    #[inline]
    pub fn py_months(&self) -> Option<usize> {
        self.inner.months()
    }

    #[getter(weeks)]
    #[inline]
    pub fn py_weeks(&self) -> Option<usize> {
        self.inner.weeks()
    }

    #[getter(days)]
    #[inline]
    pub fn py_days(&self) -> Option<usize> {
        self.inner.days()
    }

    #[getter(hours)]
    #[inline]
    pub fn py_hours(&self) -> Option<usize> {
        self.inner.hours()
    }

    #[getter(minutes)]
    #[inline]
    pub fn py_minutes(&self) -> Option<usize> {
        self.inner.minutes()
    }

    #[getter(seconds)]
    #[inline]
    pub fn py_seconds(&self) -> Option<usize> {
        self.inner.seconds()
    }

    #[getter(ticks)]
    #[inline]
    pub fn py_ticks(&self) -> Option<usize> {
        self.inner.ticks()
    }

    #[getter(ranges)]
    #[inline]
    pub fn py_ranges(&self) -> Option<usize> {
        self.inner.ranges()
    }

    #[getter(unknown)]
    #[inline]
    pub fn py_unknown(&self) -> bool {
        self.inner.unknown()
    }

    #[getter(duration)]
    #[inline]
    pub fn py_duration(&self) -> PyResult<Duration> {
        let duration = TryInto::<Duration>::try_into(self.inner);
        match duration {
            Ok(dur) => Ok(dur),
            Err(_) => Err(PyValueError::new_err("Invalid timeframe")),
        }
    }

    #[staticmethod]
    #[pyo3(name = "from_duration")]
    #[inline]
    pub fn py_from_duration(duration: Duration) -> PyResult<Self> {
        let timeframe = TryInto::<Timeframe>::try_into(duration);
        match timeframe {
            Ok(tf) => Ok(tf.into()),
            Err(_) => Err(PyValueError::new_err("Invalid duration")),
        }
    }
}
