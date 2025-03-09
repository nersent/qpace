cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
}}
use crate::timeframe::Timeframe;

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "Timeframe"))]
#[derive(Debug, Clone, Copy)]
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

#[cfg(feature = "bindings_py")]
#[gen_stub_pymethods]
#[pymethods]
impl PyTimeframe {
    #[pyo3(name = "__str__")]
    #[inline]
    pub fn py_str(&self) -> String {
        format!("{:?}", self)
    }

    #[staticmethod]
    #[pyo3(name = "from_str")]
    #[inline]
    pub fn py_from_str(timeframe: String) -> Self {
        Timeframe::from(timeframe).into()
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
        match self.inner {
            Timeframe::Years(value) => Some(value),
            _ => None,
        }
    }

    #[getter(months)]
    #[inline]
    pub fn py_months(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Months(value) => Some(value),
            _ => None,
        }
    }

    #[getter(weeks)]
    #[inline]
    pub fn py_weeks(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Weeks(value) => Some(value),
            _ => None,
        }
    }

    #[getter(days)]
    #[inline]
    pub fn py_days(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Days(value) => Some(value),
            _ => None,
        }
    }

    #[getter(hours)]
    #[inline]
    pub fn py_hours(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Hours(value) => Some(value),
            _ => None,
        }
    }

    #[getter(minutes)]
    #[inline]
    pub fn py_minutes(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Minutes(value) => Some(value),
            _ => None,
        }
    }

    #[getter(seconds)]
    #[inline]
    pub fn py_seconds(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Seconds(value) => Some(value),
            _ => None,
        }
    }

    #[getter(ticks)]
    #[inline]
    pub fn py_ticks(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Ticks(value) => Some(value),
            _ => None,
        }
    }

    #[getter(ranges)]
    #[inline]
    pub fn py_ranges(&self) -> Option<usize> {
        match self.inner {
            Timeframe::Ranges(value) => Some(value),
            _ => None,
        }
    }

    #[getter(unknown)]
    #[inline]
    pub fn py_unknown(&self) -> bool {
        matches!(self.inner, Timeframe::Unknown())
    }
}
