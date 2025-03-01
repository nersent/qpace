cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
  use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    types::{PySequence, PySlice, PySliceIndices},
  };
  use crate::rs_utils::{pyslice_to_range};
}}
cfg_if::cfg_if! { if #[cfg(feature = "polars_utils")] {
    use polars::frame::DataFrame;
    use crate::rs_utils::{read_df};
    use crate::ohlcv::{ohlcv_bars_from_polars};
}}
use crate::ohlcv::{zip_ohlcv_bars, Ohlcv};
use crate::{
    ohlcv::{ArcOhlcv, OhlcvBar, OhlcvLoader, OhlcvReader, OhlcvWriter},
    rs_utils::get_oldest_possible_datetime,
};
use chrono::{DateTime, Utc};
use std::{ffi::OsStr, ops::Range, path::Path};

#[cfg(feature = "bindings_py")]
#[gen_stub_pymethods]
#[pymethods]
impl OhlcvBar {
    #[new]
    #[pyo3(signature = (open_time=None, close_time=None, open=None, high=None, low=None, close=None, volume=None))]
    #[inline]
    pub fn py_new(
        open_time: Option<DateTime<Utc>>,
        close_time: Option<DateTime<Utc>>,
        open: Option<f64>,
        high: Option<f64>,
        low: Option<f64>,
        close: Option<f64>,
        volume: Option<f64>,
    ) -> Self {
        let open_time = open_time.unwrap_or_else(|| get_oldest_possible_datetime());
        let close_time = close_time.unwrap_or_else(|| get_oldest_possible_datetime());
        let open = open.unwrap_or(f64::NAN);
        let high = high.unwrap_or(f64::NAN);
        let low = low.unwrap_or(f64::NAN);
        let close = close.unwrap_or(f64::NAN);
        let volume = volume.unwrap_or(f64::NAN);
        Self::new(open_time, close_time, open, high, low, close, volume)
    }

    #[getter(open_time)]
    #[inline]
    pub fn py_open_time(&self) -> DateTime<Utc> {
        *self.open_time()
    }

    #[getter(close_time)]
    #[inline]
    pub fn py_close_time(&self) -> DateTime<Utc> {
        *self.close_time()
    }

    #[getter(open)]
    #[inline]
    pub fn py_open(&self) -> f64 {
        self.open()
    }

    #[getter(high)]
    #[inline]
    pub fn py_high(&self) -> f64 {
        self.high()
    }

    #[getter(low)]
    #[inline]
    pub fn py_low(&self) -> f64 {
        self.low()
    }

    #[getter(close)]
    #[inline]
    pub fn py_close(&self) -> f64 {
        self.close()
    }

    #[getter(volume)]
    #[inline]
    pub fn py_volume(&self) -> f64 {
        self.volume()
    }

    #[getter(hl2)]
    #[inline]
    pub fn py_hl2(&self) -> f64 {
        self.hl2()
    }

    #[getter(hlc3)]
    #[inline]
    pub fn py_hlc3(&self) -> f64 {
        self.hlc3()
    }

    #[getter(hlcc4)]
    #[inline]
    pub fn py_hlcc4(&self) -> f64 {
        self.hlcc4()
    }

    #[pyo3(name = "__format__", signature = (format_spec=None))]
    #[inline]
    pub fn py_format(&self, format_spec: Option<String>) -> String {
        self.fmt()
    }
}

cfg_if::cfg_if! { if #[cfg(feature = "polars_utils")] {
    impl Ohlcv {
        #[inline]
        pub fn from_polars(df: &DataFrame) -> Ohlcv {
            Self::from_bars(ohlcv_bars_from_polars(&df))
        }

        #[inline]
        pub fn read_path(path: &str) -> Ohlcv {
            let path = Path::new(path);
            let df = read_df(&path);
            Self::from_polars(&df)
        }
    }
}}

cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
    impl Ohlcv {
        #[inline]
        pub fn py_from_pandas(py: Python<'_>, df: &Bound<'_, PyAny>) -> PyResult<Ohlcv> {
            let open: Option<Vec<f64>> = if df.hasattr("open")? {
                df.getattr("open")?.extract()?
            } else {
                None
            };
            let high: Option<Vec<f64>> = if df.hasattr("high")? {
                df.getattr("high")?.extract()?
            } else {
                None
            };
            let low: Option<Vec<f64>> = if df.hasattr("low")? {
                df.getattr("low")?.extract()?
            } else {
                None
            };
            let close: Option<Vec<f64>> = if df.hasattr("close")? {
                df.getattr("close")?.extract()?
            } else {
                None
            };
            let volume: Option<Vec<f64>> = if df.hasattr("volume")? {
                df.getattr("volume")?.extract()?
            } else {
                None
            };
            let open_time: Option<Vec<Option<DateTime<Utc>>>> = if df.hasattr("open_time")? {
                df.getattr("open_time")?.extract()?
            } else if df.hasattr("time")? {
                df.getattr("time")?.extract()?
            } else {
                None
            };
            let close_time: Option<Vec<Option<DateTime<_>>>> = if df.hasattr("close_time")? {
                df.getattr("close_time")?.extract()?
            } else if df.hasattr("time")? {
                df.getattr("time")?.extract()?
            } else {
                None
            };

            Ok(Self::from_bars(zip_ohlcv_bars(open_time, close_time, open, high, low, close, volume)))
        }
    }
}}

#[cfg(feature = "bindings_py")]
#[gen_stub_pymethods]
#[pymethods]
impl ArcOhlcv {
    #[new]
    #[pyo3(signature = (bars=None))]
    #[inline]
    pub fn py_new(bars: Option<Vec<OhlcvBar>>) -> Self {
        let bars = bars.unwrap_or_default();
        Self::from_bars(bars)
    }

    #[getter(bars)]
    #[inline]
    pub fn py_bars(&self) -> Vec<OhlcvBar> {
        self.all_bars().to_vec()
    }

    #[pyo3(name = "bars_from_slice")]
    #[inline]
    pub fn py_bars_from_slice(&self, slice: &Bound<'_, PySlice>) -> Vec<OhlcvBar> {
        let range = pyslice_to_range(slice, self.len());
        self.bars(range).to_vec()
    }

    #[pyo3(name = "bar")]
    #[inline]
    pub fn py_bar(&self, index: usize) -> OhlcvBar {
        *self.bar(index)
    }

    #[pyo3(name = "__len__")]
    #[inline]
    pub fn py_len(&self) -> usize {
        self.len()
    }

    #[pyo3(name = "__format__", signature = (format_spec=None))]
    #[inline]
    pub fn py_format(&self, format_spec: Option<String>) -> String {
        self.fmt()
    }

    #[cfg(feature = "polars_utils")]
    #[staticmethod]
    #[pyo3(name = "read_path")]
    #[inline]
    pub fn py_read_path(path: String) -> Self {
        Ohlcv::read_path(&path).into()
    }

    #[staticmethod]
    #[pyo3(name = "from_bars")]
    #[inline]
    pub fn py_from_bars(bars: Vec<OhlcvBar>) -> Self {
        ArcOhlcv::from_bars(bars)
    }

    #[staticmethod]
    #[pyo3(name = "from_pandas")]
    #[inline]
    pub fn py_from_pandas(py: Python<'_>, df: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ohlcv::py_from_pandas(py, df).map(|x| x.into())
    }
}

#[cfg(feature = "bindings_py")]
#[gen_stub_pymethods]
#[pymethods]
impl OhlcvLoader {
    #[new]
    #[pyo3(signature = (bars=None))]
    #[inline]
    pub fn py_new(bars: Option<Vec<OhlcvBar>>) -> Self {
        let bars = bars.unwrap_or_default();
        Self::from_bars(bars)
    }

    #[getter(bars)]
    #[inline]
    pub fn py_bars(&self) -> Vec<OhlcvBar> {
        self.all_bars().to_vec()
    }

    #[pyo3(name = "bars_from_slice")]
    #[inline]
    pub fn py_bars_from_slice(&self, slice: &Bound<'_, PySlice>) -> Vec<OhlcvBar> {
        let range = pyslice_to_range(slice, self.len());
        self.bars(range).to_vec()
    }

    #[pyo3(name = "bar")]
    #[inline]
    pub fn py_bar(&self, index: usize) -> OhlcvBar {
        *self.bar(index)
    }

    #[pyo3(name = "__len__")]
    #[inline]
    pub fn py_len(&self) -> usize {
        self.len()
    }

    #[pyo3(name = "__format__", signature = (format_spec=None))]
    #[inline]
    pub fn py_format(&self, format_spec: Option<String>) -> String {
        self.fmt()
    }

    #[pyo3(name = "push")]
    #[inline]
    pub fn py_push(&mut self, bar: OhlcvBar) {
        self.push(bar);
    }

    #[pyo3(name = "push_many")]
    #[inline]
    pub fn py_push_many(&mut self, bars: Vec<OhlcvBar>) {
        self.push_many(&bars);
    }

    #[cfg(feature = "polars_utils")]
    #[staticmethod]
    #[pyo3(name = "read_path")]
    #[inline]
    pub fn py_read_path(path: String) -> Self {
        Ohlcv::read_path(&path).into()
    }

    #[staticmethod]
    #[pyo3(name = "from_bars")]
    #[inline]
    pub fn py_from_bars(bars: Vec<OhlcvBar>) -> Self {
        OhlcvLoader::from_bars(bars)
    }

    #[staticmethod]
    #[pyo3(name = "from_pandas")]
    #[inline]
    pub fn py_from_pandas(py: Python<'_>, df: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ohlcv::py_from_pandas(py, df).map(|x| x.into())
    }
}
