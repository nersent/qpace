cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
  use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    types::{PySequence, PySlice, PySliceIndices},
  };
  use pyo3::types::PyDict;
  use crate::rs_utils::{pyslice_to_range};
}}
cfg_if::cfg_if! { if #[cfg(feature = "polars")] {
    use polars::frame::DataFrame;
    use crate::rs_utils::{read_df};
    use crate::ohlcv::{ohlcv_bars_from_polars};
    use crate::rs_utils::PandasDataFrame;
}}
use crate::ohlcv::{zip_ohlcv_bars, Ohlcv};
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

    #[getter(open_time_ms)]
    #[inline]
    pub fn py_open_time_ms(&self) -> i64 {
        self.open_time().timestamp_millis()
    }

    #[getter(close_time_ms)]
    #[inline]
    pub fn py_close_time_ms(&self) -> i64 {
        self.close_time().timestamp_millis()
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

#[cfg(feature = "bindings_py")]
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

        Ok(Self::from_bars(zip_ohlcv_bars(
            open_time, close_time, open, high, low, close, volume,
        )))
    }

    #[inline]
    pub fn py_to_pandas(&self, py: Python<'_>) -> PyResult<PyObject> {
        let open = self.open();
        let high = self.high();
        let low = self.low();
        let close = self.close();
        let volume = self.volume();
        let open_time = self.open_time();
        let close_time = self.close_time();

        let dict = PyDict::new(py);
        dict.set_item("open", open)?;
        dict.set_item("high", high)?;
        dict.set_item("low", low)?;
        dict.set_item("close", close)?;
        dict.set_item("volume", volume)?;
        dict.set_item("open_time", open_time)?;
        dict.set_item("close_time", close_time)?;

        let pd = py.import("pandas")?;
        let df = pd.getattr("DataFrame")?.call1((dict,))?;

        Ok(df.into())
    }
}

#[cfg(feature = "polars")]
impl Ohlcv {
    #[inline]
    pub fn from_polars(df: &DataFrame, time_unit: &str) -> Ohlcv {
        Self::from_bars(ohlcv_bars_from_polars(&df, time_unit))
    }

    #[inline]
    pub fn read_path(path: &Path, time_unit: &str) -> Ohlcv {
        let df = read_df(path);
        Self::from_polars(&df, time_unit)
    }
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "Ohlcv"))]
#[derive(Clone, Debug)]
#[doc = "Multi-thread mutable OHLCV dataframe. Uses `Arc<RwLock<Ohlcv>>` internally."]
pub struct PyOhlcv {
    inner: Arc<RwLock<Ohlcv>>,
}

impl Into<PyOhlcv> for Ohlcv {
    #[inline]
    fn into(self) -> PyOhlcv {
        PyOhlcv {
            inner: Arc::new(RwLock::new(self)),
        }
    }
}

impl Into<Ohlcv> for PyOhlcv {
    #[inline]
    fn into(self) -> Ohlcv {
        self.inner.read().unwrap().clone()
    }
}

impl OhlcvReader for PyOhlcv {
    #[inline]
    fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }

    #[inline]
    fn bar(&self, index: usize) -> &OhlcvBar {
        let borrowed = self.inner.read().unwrap();
        let ptr: *const OhlcvBar = &borrowed.bars[index];
        unsafe { &*ptr }
    }

    #[inline]
    fn bars(&self, range: Range<usize>) -> &[OhlcvBar] {
        let borrowed = self.inner.read().unwrap();
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

impl OhlcvWriter for PyOhlcv {
    #[inline]
    fn push(&mut self, bar: OhlcvBar) {
        self.inner.write().unwrap().push(bar);
    }

    #[inline]
    fn push_many(&mut self, bars: &[OhlcvBar]) {
        self.inner.write().unwrap().push_many(bars);
    }
}

#[cfg(feature = "bindings_py")]
#[gen_stub_pymethods]
#[pymethods]
impl PyOhlcv {
    #[staticmethod]
    #[pyo3(name = "from_bars")]
    #[inline]
    pub fn py_from_bars(bars: Vec<OhlcvBar>) -> Self {
        Ohlcv::from_bars(bars).into()
    }

    #[staticmethod]
    #[pyo3(name = "from_pandas")]
    #[inline]
    pub fn py_from_pandas(py: Python<'_>, df: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ohlcv::py_from_pandas(py, df).map(|x| x.into())
    }

    #[cfg(feature = "polars")]
    #[staticmethod]
    #[pyo3(name = "read_path", signature = (path, time_unit="s".to_string()))]
    #[inline]
    #[doc = "`time_unit: 's' | 'ms'`"]
    pub fn py_read_path(path: String, time_unit: String) -> Self {
        Ohlcv::read_path(&Path::new(&path), &time_unit).into()
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

    #[getter(open)]
    #[inline]
    pub fn py_open(&self) -> Vec<f64> {
        self.open()
    }

    #[getter(high)]
    #[inline]
    pub fn py_high(&self) -> Vec<f64> {
        self.high()
    }

    #[getter(low)]
    #[inline]
    pub fn py_low(&self) -> Vec<f64> {
        self.low()
    }

    #[getter(close)]
    #[inline]
    pub fn py_close(&self) -> Vec<f64> {
        self.close()
    }

    #[getter(volume)]
    #[inline]
    pub fn py_volume(&self) -> Vec<f64> {
        self.volume()
    }

    #[getter(open_time)]
    #[inline]
    pub fn py_open_time(&self) -> Vec<DateTime<Utc>> {
        self.open_time()
    }

    #[getter(close_time)]
    #[inline]
    pub fn py_close_time(&self) -> Vec<DateTime<Utc>> {
        self.close_time()
    }

    #[getter(open_time_ms)]
    #[inline]
    pub fn py_open_time_ms(&self) -> Vec<i64> {
        self.open_time_ms()
    }

    #[getter(close_time_ms)]
    #[inline]
    pub fn py_close_time_ms(&self) -> Vec<i64> {
        self.close_time_ms()
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

    #[pyo3(name = "__format__", signature = (format_spec=None))]
    #[inline]
    pub fn py_format(&self, format_spec: Option<String>) -> String {
        format!("Ohlcv(len={})", self.len())
    }

    #[pyo3(name = "to_pandas")]
    pub fn py_to_pandas(&self, py: Python<'_>) -> PandasDataFrame {
        let inner = self.inner.read().unwrap();
        PandasDataFrame(inner.py_to_pandas(py).unwrap())
    }
}
