use std::path::Path;

use crate::ohlcv::Ohlcv;
use crate::{
    ohlcv::{
        zip_ohlcv_bars, ArcOhlcv, OhlcvBar, OhlcvReader, OhlcvReaderOps, OhlcvWriter,
        OhlcvWriterOps,
    },
    timeframe_py::PyTimeframe,
    utils::pyslice_to_range,
};
use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::PySlice;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pyfunction, gen_stub_pymethods};

cfg_if::cfg_if! { if #[cfg(feature = "polars")] {
    use crate::utils::PandasDataFrame;
}}

impl IntoPy<PyResult<PyObject>> for OhlcvBar {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        dict.set_item("open_time", self.open_time())?;
        dict.set_item("close_time", self.close_time())?;
        dict.set_item("open", self.open())?;
        dict.set_item("high", self.high())?;
        dict.set_item("low", self.low())?;
        dict.set_item("close", self.close())?;
        dict.set_item("volume", self.volume())?;
        Ok(dict.into())
    }
}

impl OhlcvBar {
    #[inline]
    pub fn from_py(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        let open_time: Option<DateTime<Utc>> = obj.getattr("open_time")?.extract()?;
        let close_time: Option<DateTime<Utc>> = obj.getattr("close_time")?.extract()?;
        let open: f64 = obj.getattr("open")?.extract()?;
        let high: f64 = obj.getattr("high")?.extract()?;
        let low: f64 = obj.getattr("low")?.extract()?;
        let close: f64 = obj.getattr("close")?.extract()?;
        let volume: f64 = obj.getattr("volume")?.extract()?;
        return Ok(OhlcvBar::new(
            open_time, close_time, open, high, low, close, volume,
        ));
    }
}

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
        Self::new(
            open_time,
            close_time,
            open.unwrap_or(f64::NAN),
            high.unwrap_or(f64::NAN),
            low.unwrap_or(f64::NAN),
            close.unwrap_or(f64::NAN),
            volume.unwrap_or(f64::NAN),
        )
    }

    #[pyo3(name = "__str__")]
    #[inline]
    pub fn py_str(&self) -> String {
        format!("{:?}", self)
    }

    #[pyo3(name = "__repr__")]
    #[inline]
    pub fn py_repr(&self) -> String {
        format!("{:?}", self)
    }

    #[getter(open_time)]
    #[inline]
    pub fn py_open_time(&self) -> Option<DateTime<Utc>> {
        self.open_time().copied()
    }

    #[setter(open_time)]
    #[inline]
    pub fn py_set_open_time(&mut self, open_time: Option<DateTime<Utc>>) {
        self.set_open_time(open_time);
    }

    #[getter(close_time)]
    #[inline]
    pub fn py_close_time(&self) -> Option<DateTime<Utc>> {
        self.close_time().copied()
    }

    #[setter(close_time)]
    #[inline]
    pub fn py_set_close_time(&mut self, close_time: Option<DateTime<Utc>>) {
        self.set_close_time(close_time);
    }

    #[getter(open)]
    #[inline]
    pub fn py_open(&self) -> f64 {
        self.open()
    }

    #[setter(open)]
    #[inline]
    pub fn py_set_open(&mut self, open: f64) {
        self.set_open(open);
    }

    #[getter(high)]
    #[inline]
    pub fn py_high(&self) -> f64 {
        self.high()
    }

    #[setter(high)]
    #[inline]
    pub fn py_set_high(&mut self, high: f64) {
        self.set_high(high);
    }

    #[getter(low)]
    #[inline]
    pub fn py_low(&self) -> f64 {
        self.low()
    }

    #[setter(low)]
    #[inline]
    pub fn py_set_low(&mut self, low: f64) {
        self.set_low(low);
    }

    #[getter(close)]
    #[inline]
    pub fn py_close(&self) -> f64 {
        self.close()
    }

    #[setter(close)]
    #[inline]
    pub fn py_set_close(&mut self, close: f64) {
        self.set_close(close);
    }

    #[getter(volume)]
    #[inline]
    pub fn py_volume(&self) -> f64 {
        self.volume()
    }

    #[setter(volume)]
    #[inline]
    pub fn py_set_volume(&mut self, volume: f64) {
        self.set_volume(volume);
    }

    #[pyo3(name = "merge")]
    #[inline]
    pub fn py_merge(&mut self, other: &Self) -> Self {
        self.merge(other)
    }

    #[pyo3(name = "to_dict")]
    #[inline]
    pub fn py_to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.into_py(py)
    }

    #[staticmethod]
    #[pyo3(name = "from_dict")]
    #[inline]
    pub fn py_from_dict(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        Self::from_py(obj)
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "Ohlcv")]
#[derive(Debug, Clone)]
pub struct PyOhlcv {
    inner: ArcOhlcv,
}

impl Default for PyOhlcv {
    #[inline]
    fn default() -> Self {
        Self::from(ArcOhlcv::new())
    }
}

impl Into<PyOhlcv> for Ohlcv {
    #[inline]
    fn into(self) -> PyOhlcv {
        PyOhlcv { inner: self.into() }
    }
}

impl Into<ArcOhlcv> for PyOhlcv {
    #[inline]
    fn into(self) -> ArcOhlcv {
        self.inner
    }
}

impl From<ArcOhlcv> for PyOhlcv {
    #[inline]
    fn from(inner: ArcOhlcv) -> Self {
        PyOhlcv { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyOhlcv {
    #[new]
    pub fn py_new() -> Self {
        Self::from(ArcOhlcv::new())
    }

    #[staticmethod]
    #[pyo3(name = "from_bars")]
    pub fn py_from_bars(bars: Vec<OhlcvBar>) -> Self {
        ArcOhlcv::from_bars(bars).into()
    }

    #[getter(timeframe)]
    #[inline]
    pub fn py_timeframe(&self) -> PyTimeframe {
        self.inner.timeframe().into()
    }

    #[setter(timeframe)]
    #[inline]
    pub fn py_set_timeframe(&mut self, timeframe: PyTimeframe) {
        self.inner.set_timeframe(timeframe.into());
    }

    #[getter(open_time)]
    #[inline]
    pub fn py_open_time(&self) -> Vec<Option<DateTime<Utc>>> {
        self.inner.open_time()
    }

    #[getter(close_time)]
    #[inline]
    pub fn py_close_time(&self) -> Vec<Option<DateTime<Utc>>> {
        self.inner.close_time()
    }

    #[getter(open)]
    #[inline]
    pub fn py_open(&self) -> Vec<f64> {
        self.inner.open()
    }

    #[getter(high)]
    #[inline]
    pub fn py_high(&self) -> Vec<f64> {
        self.inner.high()
    }

    #[getter(low)]
    #[inline]
    pub fn py_low(&self) -> Vec<f64> {
        self.inner.low()
    }

    #[getter(close)]
    #[inline]
    pub fn py_close(&self) -> Vec<f64> {
        self.inner.close()
    }

    #[getter(volume)]
    #[inline]
    pub fn py_volume(&self) -> Vec<f64> {
        self.inner.volume()
    }

    #[getter(bars)]
    #[inline]
    pub fn py_bars(&self) -> Vec<OhlcvBar> {
        self.inner.bars()
    }

    #[pyo3(name = "__getitem__")]
    #[inline]
    pub fn py_getitem(&self, index: i32) -> Option<OhlcvBar> {
        self.inner.at(index)
    }

    #[pyo3(name = "__len__")]
    #[inline]
    pub fn py_len(&self) -> usize {
        self.inner.len()
    }

    #[pyo3(name = "slice")]
    #[inline]
    pub fn py_slice(&self, slice: &Bound<'_, PySlice>) -> PyOhlcv {
        let range = pyslice_to_range(slice, self.inner.len());
        let mut ohlcv = Ohlcv::from_bars(self.inner.slice(range));
        ohlcv.set_timeframe(self.inner.timeframe().into());
        return ohlcv.into();
    }

    #[pyo3(name = "head")]
    #[inline]
    pub fn py_head(&self, count: usize) -> PyOhlcv {
        let mut ohlcv = Ohlcv::from_bars(self.inner.head(count));
        ohlcv.set_timeframe(self.inner.timeframe().into());
        return ohlcv.into();
    }

    #[pyo3(name = "tail")]
    #[inline]
    pub fn py_tail(&self, count: usize) -> PyOhlcv {
        let mut ohlcv = Ohlcv::from_bars(self.inner.tail(count));
        ohlcv.set_timeframe(self.inner.timeframe().into());
        return ohlcv.into();
    }

    #[pyo3(name = "copy")]
    #[inline]
    pub fn py_copy(&self) -> Self {
        self.inner.clone().into()
    }

    #[pyo3(name = "extend")]
    #[inline]
    pub fn py_extend(&mut self, other: &Self) {
        self.inner.extend(&other.inner);
    }

    #[pyo3(name = "resample")]
    #[inline]
    pub fn py_resample(&self, timeframe: PyTimeframe, align_utc: bool) -> Self {
        self.inner.resample(timeframe.into(), align_utc).into()
    }

    #[pyo3(name = "sort")]
    #[inline]
    pub fn py_sort(&mut self, ascending: bool) {
        self.inner.sort(ascending);
    }

    #[pyo3(name = "reverse")]
    #[inline]
    pub fn py_reverse(&mut self) {
        self.inner.reverse()
    }

    #[pyo3(name = "clear")]
    #[inline]
    pub fn py_clear(&mut self) {
        self.inner.clear();
    }

    #[pyo3(name = "pop")]
    #[inline]
    pub fn py_pop(&mut self) -> Option<OhlcvBar> {
        self.inner.pop()
    }

    #[pyo3(name = "shift")]
    #[inline]
    pub fn py_shift(&mut self) -> Option<OhlcvBar> {
        self.inner.shift()
    }

    #[pyo3(name = "push")]
    #[inline]
    pub fn py_push(&mut self, bar: OhlcvBar) {
        self.inner.push(bar);
    }

    #[pyo3(name = "push_many")]
    #[inline]
    pub fn py_push_many(&mut self, bars: Vec<OhlcvBar>) {
        self.inner.push_many(bars);
    }

    #[staticmethod]
    #[pyo3(name = "from_pandas")]
    #[inline]
    pub fn py_from_pandas(py: Python<'_>, df: &Bound<'_, PyAny>) -> PyResult<PyOhlcv> {
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

        let ohlcv = ArcOhlcv::from_bars(zip_ohlcv_bars(
            open_time, close_time, open, high, low, close, volume,
        ));
        Ok(ohlcv.into())
    }

    #[pyo3(name = "to_pandas")]
    #[inline]
    pub fn py_to_pandas(&self, py: Python<'_>) -> PyResult<PandasDataFrame> {
        let open_time = self.inner.open_time();
        let close_time = self.inner.close_time();
        let open = self.inner.open();
        let high = self.inner.high();
        let low = self.inner.low();
        let close = self.inner.close();
        let volume = self.inner.volume();
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
        let py_obj: PyObject = df.into();
        Ok(py_obj.into())
    }

    #[cfg(feature = "polars")]
    #[staticmethod]
    #[pyo3(name = "read_csv")]
    #[inline]
    pub fn py_read_csv(path: String) -> Self {
        let mut ohlcv = Ohlcv::new();
        ohlcv.read_csv(Path::new(&path));
        let ohlcv: ArcOhlcv = ohlcv.into();
        return ohlcv.into();
    }

    #[cfg(feature = "polars")]
    #[staticmethod]
    #[pyo3(name = "read_parquet")]
    #[inline]
    pub fn py_read_parquet(path: String) -> Self {
        let mut ohlcv = Ohlcv::new();
        ohlcv.read_parquet(Path::new(&path));
        let ohlcv: ArcOhlcv = ohlcv.into();
        return ohlcv.into();
    }

    #[cfg(feature = "polars")]
    #[pyo3(name = "write_csv")]
    #[inline]
    pub fn py_write_csv(&self, path: String) {
        self.inner.write_csv(Path::new(&path));
    }

    #[cfg(feature = "polars")]
    #[pyo3(name = "write_parquet")]
    #[inline]
    pub fn py_write_parquet(&self, path: String) {
        self.inner.write_parquet(Path::new(&path));
    }

    #[pyo3(name = "sanity_check")]
    #[inline]
    pub fn py_sanity_check(&self) -> (bool, Vec<String>) {
        match self.inner.sanity_check() {
            Ok(_) => (true, vec![]),
            Err(e) => (false, e),
        }
    }
}

#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature=(open_time=None, close_time=None, open=None, high=None, low=None, close=None, volume=None))]
#[inline]
pub fn py_zip_ohlcv_bars(
    open_time: Option<Vec<Option<DateTime<Utc>>>>,
    close_time: Option<Vec<Option<DateTime<Utc>>>>,
    open: Option<Vec<f64>>,
    high: Option<Vec<f64>>,
    low: Option<Vec<f64>>,
    close: Option<Vec<f64>>,
    volume: Option<Vec<f64>>,
) -> Vec<OhlcvBar> {
    return zip_ohlcv_bars(open_time, close_time, open, high, low, close, volume);
}
