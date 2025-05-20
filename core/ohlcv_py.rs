cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
  use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    types::{PySequence, PySlice, PySliceIndices},
  };
  use pyo3::types::PyDict;
  use crate::rs_utils::{pyslice_to_range};
  use crate::rs_utils::PandasDataFrame;
  use crate::timeframe_py::PyTimeframe;
}}
cfg_if::cfg_if! { if #[cfg(feature = "polars")] {
    use polars::frame::DataFrame;
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

    // #[getter(open_time_ms)]
    // #[inline]
    // pub fn py_open_time_ms(&self) -> i64 {
    //     self.open_time().timestamp_millis()
    // }

    // #[getter(close_time_ms)]
    // #[inline]
    // pub fn py_close_time_ms(&self) -> i64 {
    //     self.close_time().timestamp_millis()
    // }

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

    #[pyo3(name = "__str__")]
    #[inline]
    pub fn py_str(&self) -> String {
        self.fmt()
    }

    #[pyo3(name = "to_dict")]
    #[inline]
    pub fn py_to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("open_time", self.open_time())?;
        dict.set_item("close_time", self.close_time())?;
        dict.set_item("open", self.open())?;
        dict.set_item("high", self.high())?;
        dict.set_item("low", self.low())?;
        dict.set_item("close", self.close())?;
        dict.set_item("volume", self.volume())?;
        return Ok(dict.to_object(py));
    }

    #[pyo3(name = "merge")]
    #[inline]
    pub fn py_merge(&self, other: &OhlcvBar) -> OhlcvBar {
        self.merge(other)
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
        let open_time = self.open_time();
        let close_time = self.close_time();
        let open = self.open();
        let high = self.high();
        let low = self.low();
        let close = self.close();
        let volume = self.volume();

        let dict = PyDict::new(py);
        dict.set_item("open_time", open_time)?;
        dict.set_item("close_time", close_time)?;
        dict.set_item("open", open)?;
        dict.set_item("high", high)?;
        dict.set_item("low", low)?;
        dict.set_item("close", close)?;
        dict.set_item("volume", volume)?;

        let pd = py.import("pandas")?;
        let df = pd.getattr("DataFrame")?.call1((dict,))?;

        Ok(df.into())
    }
}

#[cfg(feature = "bindings_py")]
#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "Ohlcv"))]
#[derive(Clone, Debug)]
#[doc = "Multi-thread mutable OHLCV dataframe. Uses `Arc<RwLock<Ohlcv>>` internally."]
pub struct PyOhlcv {
    inner: Arc<RwLock<Ohlcv>>,
    timeframe: PyTimeframe,
}

#[cfg(feature = "bindings_py")]
impl Into<PyOhlcv> for Ohlcv {
    #[inline]
    fn into(self) -> PyOhlcv {
        PyOhlcv {
            inner: Arc::new(RwLock::new(self)),
            timeframe: Timeframe::Unknown().into(),
        }
    }
}

#[cfg(feature = "bindings_py")]
impl Into<Ohlcv> for PyOhlcv {
    #[inline]
    fn into(self) -> Ohlcv {
        self.inner.read().unwrap().clone()
    }
}

#[cfg(feature = "bindings_py")]
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

#[cfg(feature = "bindings_py")]
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
    #[new]
    #[inline]
    pub fn py_new() -> Self {
        Ohlcv::empty().into()
    }

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

    #[setter(timeframe)]
    #[inline]
    pub fn py_set_timeframe(&mut self, timeframe: PyTimeframe) {
        self.timeframe = timeframe;
    }

    #[getter(timeframe)]
    #[inline]
    pub fn py_timeframe(&self) -> PyTimeframe {
        self.timeframe.clone()
    }

    #[getter(bars)]
    #[inline]
    pub fn py_bars(&self) -> Vec<OhlcvBar> {
        self.all_bars().to_vec()
    }

    #[pyo3(name = "slice")]
    #[inline]
    pub fn py_slice(&self, slice: &Bound<'_, PySlice>) -> Vec<OhlcvBar> {
        let range = pyslice_to_range(slice, self.len());
        self.bars(range).to_vec()
    }

    #[pyo3(name = "__getitem__")]
    #[inline]
    pub fn py_getitem(&self, index: usize) -> OhlcvBar {
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

    // #[getter(open_time_ms)]
    // #[inline]
    // pub fn py_open_time_ms(&self) -> Vec<i64> {
    //     self.open_time_ms()
    // }

    // #[getter(close_time_ms)]
    // #[inline]
    // pub fn py_close_time_ms(&self) -> Vec<i64> {
    //     self.close_time_ms()
    // }

    #[pyo3(name = "add")]
    #[inline]
    pub fn py_add(&mut self, bar: OhlcvBar) {
        self.push(bar);
    }

    #[pyo3(name = "add_list")]
    #[inline]
    pub fn py_add_list(&mut self, bars: Vec<OhlcvBar>) {
        self.push_many(&bars);
    }

    #[pyo3(name = "__str__")]
    #[inline]
    pub fn py_str(&self) -> String {
        let timeframe: Timeframe = self.timeframe.into();
        let timeframe: String = timeframe.into();
        format!("Ohlcv(timeframe={}, len={})", timeframe, self.len())
    }

    #[pyo3(name = "to_pandas")]
    pub fn py_to_pandas(&self, py: Python<'_>) -> PandasDataFrame {
        let inner = self.inner.read().unwrap();
        PandasDataFrame(inner.py_to_pandas(py).unwrap())
    }

    #[cfg(feature = "polars")]
    #[staticmethod]
    #[pyo3(name = "read_csv", signature = (path, time_unit="s".to_string()))]
    #[inline]
    #[doc = "`time_unit: 's' | 'ms'`"]
    pub fn py_read_csv(path: String, time_unit: String) -> Self {
        Ohlcv::read_csv(&Path::new(&path), &time_unit).into()
    }

    #[cfg(feature = "polars")]
    #[staticmethod]
    #[pyo3(name = "read_parquet", signature = (path, time_unit="s".to_string()))]
    #[inline]
    #[doc = "`time_unit: 's' | 'ms'`"]
    pub fn py_read_parquet(path: String, time_unit: String) -> Self {
        Ohlcv::read_parquet(&Path::new(&path), &time_unit).into()
    }

    #[pyo3(name = "resample", signature = (timeframe, align_utc=true))]
    #[inline]
    #[doc = "Resamples OHLCV bars into the specified timeframe.
    If align_utc is true, bars are pinned to calendar-based UTC boundaries;
    otherwise, a rolling time window is used.\n`align_utc`: boolean. Default: true"]
    pub fn py_resample(&self, timeframe: PyTimeframe, align_utc: bool) -> Self {
        let timeframe: Timeframe = timeframe.into();
        let ohlcv = self.inner.read().unwrap().clone();
        let resampled = ohlcv.resample(timeframe, align_utc);
        let mut resampled: Self = resampled.into();
        resampled.py_set_timeframe(timeframe.into());
        return resampled;
    }
}
