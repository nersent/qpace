#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "ArcOhlcv"))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = ArcOhlcv))]
#[derive(Clone, Debug)]
#[doc = "Multi-threaded immutable OHLCV data."]
pub struct ArcOhlcv {
    inner: Arc<Ohlcv>,
}

impl ArcOhlcv {
    pub fn from_bars(bars: Vec<OhlcvBar>) -> Self {
        Self {
            inner: Arc::new(Ohlcv::from_bars(bars)),
        }
    }
}

impl OhlcvReader for ArcOhlcv {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    fn bar(&self, index: usize) -> &OhlcvBar {
        self.inner.bar(index)
    }

    #[inline]
    fn bars(&self, range: Range<usize>) -> &[OhlcvBar] {
        self.inner.bars(range)
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

impl ArcOhlcv {
    pub fn fmt(&self) -> String {
        format!("Ohlcv(len={})", self.len())
    }
}

#[cfg_attr(feature = "bindings_py", gen_stub_pyclass)]
#[cfg_attr(feature = "bindings_py", pyclass(name = "Ohlcv", unsendable))]
#[cfg_attr(feature = "bindings_wasm", wasm_bindgen(js_name = Ohlcv))]
#[derive(Clone, Debug)]
#[doc = "Single-threaded mutable OHLCV data."]
pub struct OhlcvLoader {
    inner: Rc<RefCell<Ohlcv>>,
}

impl OhlcvReader for OhlcvLoader {
    #[inline]
    fn len(&self) -> usize {
        self.inner.borrow().len()
    }

    #[inline]
    fn bar(&self, index: usize) -> &OhlcvBar {
        let borrowed = self.inner.borrow();
        let ptr: *const OhlcvBar = &borrowed.bars[index];
        unsafe { &*ptr }
    }

    #[inline]
    fn bars(&self, range: Range<usize>) -> &[OhlcvBar] {
        let borrowed = self.inner.borrow();
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

impl OhlcvWriter for OhlcvLoader {
    #[inline]
    fn push(&mut self, bar: OhlcvBar) {
        self.inner.borrow_mut().push(bar);
    }
}

impl OhlcvLoader {
    pub fn from_bars(bars: Vec<OhlcvBar>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(Ohlcv::from_bars(bars))),
        }
    }

    pub fn fmt(&self) -> String {
        format!("OhlcvLoader(len={})", self.len())
    }
}

impl Into<Option<ArcOhlcv>> for &dyn OhlcvReader {
    fn into(self) -> Option<ArcOhlcv> {
        self.as_any().downcast_ref::<ArcOhlcv>().cloned()
    }
}

impl Into<Option<OhlcvLoader>> for &dyn OhlcvReader {
    fn into(self) -> Option<OhlcvLoader> {
        self.as_any().downcast_ref::<OhlcvLoader>().cloned()
    }
}

impl Into<ArcOhlcv> for Ohlcv {
    fn into(self) -> ArcOhlcv {
        ArcOhlcv {
            inner: Arc::new(self),
        }
    }
}

impl Into<OhlcvLoader> for Ohlcv {
    fn into(self) -> OhlcvLoader {
        OhlcvLoader {
            inner: Rc::new(RefCell::new(self)),
        }
    }
}

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

    #[cfg(feature = "polars")]
    #[staticmethod]
    #[pyo3(name = "read_path")]
    #[inline]
    pub fn py_read_path(path: String) -> Self {
        Ohlcv::read_path(&Path::new(&path)).into()
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

    #[cfg(feature = "polars")]
    #[staticmethod]
    #[pyo3(name = "read_path")]
    #[inline]
    pub fn py_read_path(path: String) -> Self {
        Ohlcv::read_path(&Path::new(&path)).into()
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
}
