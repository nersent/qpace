use core::f64;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use qpace_core::ctx::Ctx;
use qpace_core::ohlcv::{hl2, hlc3, hlcc4, OhlcvBar};
use std::cell::{Ref, RefMut};
use std::marker::PhantomData;
use std::ops::{AddAssign, SubAssign};
use std::{cell::RefCell, rc::Rc};

// #region qpace glue
#[derive(Clone)]
pub struct PaceContext {
    inner: Rc<RefCell<Ctx>>,
}

impl std::fmt::Debug for PaceContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaceContext").finish()
    }
}

impl Into<PaceContext> for Rc<RefCell<Ctx>> {
    #[inline]
    fn into(self) -> PaceContext {
        PaceContext { inner: self }
    }
}

impl PaceContext {
    pub fn next(&self) -> Option<usize> {
        let mut ctx = self.inner.borrow_mut();
        ctx.next()
    }

    pub fn bar_index(&self) -> usize {
        self.inner.borrow().bar_index()
    }

    pub fn last_bar_index(&self) -> usize {
        self.inner.borrow().len().saturating_sub(1)
    }

    pub fn bar(&self) -> OhlcvBar {
        self.inner.borrow().bar()
    }
}

// #endregion

// #region series
trait PaceSeriesGetter<T> {
    fn get(values: &[T]) -> T;
    fn get_at(values: &[T], idx: usize) -> T;
}

impl<T: Clone + From<PaceNa>> PaceSeriesGetter<T> for PhantomData<T> {
    fn get(values: &[T]) -> T {
        values.last().cloned().unwrap_or_else(|| T::from(PaceNa))
    }

    fn get_at(values: &[T], idx: usize) -> T {
        values.get(idx).cloned().unwrap_or_else(|| T::from(PaceNa))
    }
}

impl<T: Copy + From<PaceNa>> PaceSeriesGetter<T> for T {
    fn get(values: &[T]) -> T {
        *values.last().unwrap_or(&T::from(PaceNa))
    }

    fn get_at(values: &[T], idx: usize) -> T {
        *values.get(idx).unwrap_or(&T::from(PaceNa))
    }
}

#[derive(Debug, Clone)]
pub struct PaceSeries<T: Clone + From<PaceNa>> {
    values: Vec<T>,
    _marker: PhantomData<T>,
}

impl<T: Clone + From<PaceNa>> PaceSeries<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn from(values: Vec<T>) -> Self {
        Self {
            values,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn get(&self) -> T {
        <PhantomData<T> as PaceSeriesGetter<T>>::get(&self.values)
    }

    #[inline]
    pub fn get_at(&self, idx: usize) -> T {
        <PhantomData<T> as PaceSeriesGetter<T>>::get_at(&self.values, idx)
    }

    #[inline]
    pub fn get_at_offset(&self, offset: usize) -> T {
        if offset >= self.values.len() {
            return T::from(PaceNa);
        }
        let idx = self.values.len() - 1 - offset;
        return self.get_at(idx);
    }

    #[inline]
    pub fn next(&mut self) {
        if self.values.len() == 0 {
            return;
        }
        self.values.push(self.get());
    }

    #[inline]
    pub fn set(&mut self, value: T) {
        self.values.pop();
        self.values.push(value);
    }
}

// #endregion

// #region type declarations
#[derive(Debug, Clone, Copy)]
pub struct PaceNa;

#[derive(Debug, Clone, Copy)]
pub struct PaceBool(pub Option<bool>);

#[derive(Debug, Clone, Copy)]
pub struct PaceInt(pub Option<i64>);

#[derive(Debug, Clone, Copy)]
pub struct PaceFloat(pub f64);

#[derive(Debug, Clone)]
pub struct PaceString(pub Option<String>);

#[derive(Debug, Clone)]
pub enum PaceAny {
    PaceNa,
    PaceFloat(PaceFloat),
    PaceInt(PaceInt),
    PaceString(PaceString),
    PaceBool(PaceBool),
    // PaceColor(PaceColor),
}
// #endregion

// #region na ops
pub trait IsNa {
    fn is_na(&self) -> bool;
}

impl IsNa for PaceNa {
    #[inline]
    fn is_na(&self) -> bool {
        return true;
    }
}

impl IsNa for PaceBool {
    #[inline]
    fn is_na(&self) -> bool {
        return self.0.is_none();
    }
}

impl IsNa for PaceInt {
    #[inline]
    fn is_na(&self) -> bool {
        return self.0.is_none();
    }
}

impl IsNa for PaceFloat {
    #[inline]
    fn is_na(&self) -> bool {
        return self.0.is_nan();
    }
}

impl IsNa for PaceString {
    #[inline]
    fn is_na(&self) -> bool {
        return self.0.is_none();
    }
}

impl<T: Clone + From<PaceNa> + IsNa> IsNa for PaceSeries<T> {
    #[inline]
    fn is_na(&self) -> bool {
        return self.get().is_na();
    }
}

// pub trait UnwrapNaOr<T> {
//     fn unwrap_na_or(&self, default: T) -> T;
// }

// impl UnwrapNaOr<PaceBool> for PaceBool {
//     #[inline]
//     fn unwrap_na_or(&self, default: PaceBool) -> PaceBool {
//         return if self.is_na() { default } else { *self };
//     }
// }

// impl UnwrapNaOr<PaceInt> for PaceInt {
//     #[inline]
//     fn unwrap_na_or(&self, default: PaceInt) -> PaceInt {
//         return if self.is_na() { default } else { *self };
//     }
// }

// impl UnwrapNaOr<PaceFloat> for PaceFloat {
//     #[inline]
//     fn unwrap_na_or(&self, default: PaceFloat) -> PaceFloat {
//         return if self.is_na() { default } else { *self };
//     }
// }

// impl UnwrapNaOr<PaceString> for PaceString {
//     #[inline]
//     fn unwrap_na_or(&self, default: PaceString) -> PaceString {
//         return if self.is_na() { default } else { self.clone() };
//     }
// }

// impl UnwrapNaOr<PaceNa> for PaceNa {
//     #[inline]
//     fn unwrap_na_or(&self, default: PaceNa) -> PaceNa {
//         return if self.is_na() { default } else { *self };
//     }
// }

// #endregion

// #region bin ops
impl std::ops::Add for PaceFloat {
    type Output = PaceFloat;
    #[inline]
    fn add(self, other: PaceFloat) -> PaceFloat {
        if self.is_na() || other.is_na() {
            return PaceFloat(f64::NAN);
        }
        return PaceFloat(self.0 + other.0);
    }
}

impl std::ops::Sub for PaceFloat {
    type Output = PaceFloat;
    #[inline]
    fn sub(self, other: PaceFloat) -> PaceFloat {
        if self.is_na() || other.is_na() {
            return PaceFloat(f64::NAN);
        }
        return PaceFloat(self.0 - other.0);
    }
}

impl std::ops::Mul for PaceFloat {
    type Output = PaceFloat;
    #[inline]
    fn mul(self, other: PaceFloat) -> PaceFloat {
        if self.is_na() || other.is_na() {
            return PaceFloat(f64::NAN);
        }
        return PaceFloat(self.0 * other.0);
    }
}

impl std::ops::Div for PaceFloat {
    type Output = PaceFloat;
    #[inline]
    fn div(self, other: PaceFloat) -> PaceFloat {
        if self.is_na() || other.is_na() {
            return PaceFloat(f64::NAN);
        }
        return PaceFloat(self.0 / other.0);
    }
}

impl std::ops::Add for PaceInt {
    type Output = PaceInt;
    #[inline]
    fn add(self, other: PaceInt) -> PaceInt {
        if self.is_na() || other.is_na() {
            return PaceInt(None);
        }
        return (self.0.unwrap() + other.0.unwrap()).into();
    }
}

impl std::ops::Sub for PaceInt {
    type Output = PaceInt;
    #[inline]
    fn sub(self, other: PaceInt) -> PaceInt {
        if self.is_na() || other.is_na() {
            return PaceInt(None);
        }
        return (self.0.unwrap() - other.0.unwrap()).into();
    }
}

impl std::ops::Mul for PaceInt {
    type Output = PaceInt;
    #[inline]
    fn mul(self, other: PaceInt) -> PaceInt {
        if self.is_na() || other.is_na() {
            return PaceInt(None);
        }
        return (self.0.unwrap() * other.0.unwrap()).into();
    }
}

impl std::ops::Div for PaceInt {
    type Output = PaceInt;
    #[inline]
    fn div(self, other: PaceInt) -> PaceInt {
        if self.is_na() || other.is_na() {
            return PaceInt(None);
        }
        if other.0.unwrap() == 0 {
            return PaceInt(None);
        }
        return (self.0.unwrap() / other.0.unwrap()).into();
    }
}

impl std::cmp::PartialOrd for PaceFloat {
    #[inline]
    fn partial_cmp(&self, other: &PaceFloat) -> Option<std::cmp::Ordering> {
        if self.is_na() || other.is_na() {
            return None;
        }
        return self.0.partial_cmp(&other.0);
    }
}

impl std::cmp::PartialOrd for PaceInt {
    #[inline]
    fn partial_cmp(&self, other: &PaceInt) -> Option<std::cmp::Ordering> {
        if self.is_na() || other.is_na() {
            return None;
        }
        return self.0.partial_cmp(&other.0);
    }
}

impl std::cmp::PartialOrd for PaceBool {
    #[inline]
    fn partial_cmp(&self, other: &PaceBool) -> Option<std::cmp::Ordering> {
        if self.is_na() || other.is_na() {
            return None;
        }
        return self.0.partial_cmp(&other.0);
    }
}

impl std::cmp::PartialOrd for PaceString {
    #[inline]
    fn partial_cmp(&self, other: &PaceString) -> Option<std::cmp::Ordering> {
        if self.is_na() || other.is_na() {
            return None;
        }
        return self.0.partial_cmp(&other.0);
    }
}

impl std::cmp::PartialEq for PaceFloat {
    #[inline]
    fn eq(&self, other: &PaceFloat) -> bool {
        if self.is_na() || other.is_na() {
            return false;
        }
        return self.0 == other.0;
    }
}

impl std::cmp::PartialEq for PaceInt {
    #[inline]
    fn eq(&self, other: &PaceInt) -> bool {
        if self.is_na() || other.is_na() {
            return false;
        }
        return self.0 == other.0;
    }
}

impl std::cmp::PartialEq for PaceBool {
    #[inline]
    fn eq(&self, other: &PaceBool) -> bool {
        if self.is_na() || other.is_na() {
            return false;
        }
        return self.0 == other.0;
    }
}

impl std::cmp::PartialEq for PaceString {
    #[inline]
    fn eq(&self, other: &PaceString) -> bool {
        if self.is_na() || other.is_na() {
            return false;
        }
        return self.0 == other.0;
    }
}

// #endregion

// #region conversions
impl From<bool> for PaceBool {
    #[inline]
    fn from(x: bool) -> PaceBool {
        return PaceBool(Some(x));
    }
}

impl From<&bool> for PaceBool {
    #[inline]
    fn from(x: &bool) -> PaceBool {
        return PaceBool(Some(*x));
    }
}

impl From<PaceBool> for bool {
    #[inline]
    fn from(x: PaceBool) -> bool {
        return x.0.unwrap_or(false);
    }
}

impl From<PaceInt> for i64 {
    #[inline]
    fn from(x: PaceInt) -> i64 {
        return x.0.unwrap_or(0);
    }
}

impl From<i32> for PaceInt {
    #[inline]
    fn from(x: i32) -> PaceInt {
        return PaceInt(Some(x as i64));
    }
}

impl From<i64> for PaceInt {
    #[inline]
    fn from(x: i64) -> PaceInt {
        return PaceInt(Some(x));
    }
}

impl From<usize> for PaceInt {
    #[inline]
    fn from(x: usize) -> PaceInt {
        return PaceInt(Some(x as i64));
    }
}

impl From<&chrono::DateTime<chrono::Utc>> for PaceInt {
    #[inline]
    fn from(x: &chrono::DateTime<chrono::Utc>) -> PaceInt {
        return PaceInt(Some(x.timestamp_millis()));
    }
}

impl From<Option<&chrono::DateTime<chrono::Utc>>> for PaceInt {
    #[inline]
    fn from(x: Option<&chrono::DateTime<chrono::Utc>>) -> PaceInt {
        return match x {
            Some(dt) => PaceInt(Some(dt.timestamp_millis())),
            None => PaceInt(None),
        };
    }
}

impl From<PaceInt> for usize {
    #[inline]
    fn from(x: PaceInt) -> usize {
        return x.0.unwrap_or(0) as usize;
    }
}

impl From<PaceFloat> for f64 {
    #[inline]
    fn from(x: PaceFloat) -> f64 {
        return x.0;
    }
}

impl From<f64> for PaceFloat {
    #[inline]
    fn from(x: f64) -> PaceFloat {
        return PaceFloat(x);
    }
}

impl From<usize> for PaceFloat {
    #[inline]
    fn from(x: usize) -> PaceFloat {
        return PaceFloat(x as f64);
    }
}

impl From<PaceFloat> for PaceInt {
    #[inline]
    fn from(x: PaceFloat) -> PaceInt {
        return PaceInt(Some(x.0 as i64));
    }
}

impl From<PaceInt> for PaceFloat {
    #[inline]
    fn from(x: PaceInt) -> PaceFloat {
        return PaceFloat(x.0.map(|x| x as f64).unwrap_or(f64::NAN));
    }
}

impl From<PaceNa> for PaceInt {
    #[inline]
    fn from(_: PaceNa) -> PaceInt {
        return PaceInt(None);
    }
}

impl From<PaceNa> for PaceFloat {
    #[inline]
    fn from(_: PaceNa) -> PaceFloat {
        return PaceFloat(f64::NAN);
    }
}

impl From<PaceNa> for PaceBool {
    #[inline]
    fn from(_: PaceNa) -> PaceBool {
        return PaceBool(None);
    }
}

impl From<PaceNa> for PaceString {
    #[inline]
    fn from(_: PaceNa) -> PaceString {
        return PaceString(None);
    }
}

impl PaceBool {
    #[inline]
    pub fn and(self, other: PaceBool) -> PaceBool {
        return PaceBool(Some(self.0.unwrap_or(false) && other.0.unwrap_or(false)));
    }

    #[inline]
    pub fn or(self, other: PaceBool) -> PaceBool {
        return PaceBool(Some(self.0.unwrap_or(false) || other.0.unwrap_or(false)));
    }
}

impl<T> IntoPy<PyObject> for PaceSeries<T>
where
    T: Clone + From<PaceNa> + IntoPy<PyObject>,
{
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.values.into_py(py)
    }
}

impl<'py, T> FromPyObject<'py> for PaceSeries<T>
where
    T: Clone + From<PaceNa> + FromPyObject<'py>,
{
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let values = ob.extract::<Vec<T>>()?;
        Ok(PaceSeries::from(values))
    }
}
// #endregion

// #region defaults

impl Default for PaceInt {
    #[inline]
    fn default() -> PaceInt {
        return PaceInt(None);
    }
}

impl Default for PaceFloat {
    #[inline]
    fn default() -> PaceFloat {
        return PaceFloat(f64::NAN);
    }
}

impl Default for PaceBool {
    #[inline]
    fn default() -> PaceBool {
        return PaceBool(None);
    }
}

impl Default for PaceString {
    #[inline]
    fn default() -> PaceString {
        return PaceString(None);
    }
}

impl Default for PaceSeries<PaceFloat> {
    fn default() -> Self {
        return PaceSeries::new();
    }
}

impl Default for PaceSeries<PaceInt> {
    fn default() -> Self {
        return PaceSeries::new();
    }
}

impl Default for PaceSeries<PaceBool> {
    fn default() -> Self {
        return PaceSeries::new();
    }
}

impl Default for PaceSeries<PaceString> {
    fn default() -> Self {
        return PaceSeries::new();
    }
}

impl IntoPy<PyObject> for PaceFloat {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.0.into_py(py)
    }
}

impl IntoPy<PyObject> for PaceString {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self.0 {
            Some(s) => s.into_py(py),
            None => "".into_py(py),
        }
    }
}

impl IntoPy<PyObject> for PaceInt {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self.0 {
            Some(i) => i.into_py(py),
            None => 0.into_py(py),
        }
    }
}

impl IntoPy<PyObject> for PaceBool {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self.0 {
            Some(b) => b.into_py(py),
            None => false.into_py(py),
        }
    }
}

impl<'py> FromPyObject<'py> for PaceFloat {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(float) = ob.extract::<f64>() {
            Ok(PaceFloat(float))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Expected a float"))
        }
    }
}

impl<'py> FromPyObject<'py> for PaceString {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(string) = ob.extract::<String>() {
            Ok(PaceString(Some(string)))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Expected a string"))
        }
    }
}

impl<'py> FromPyObject<'py> for PaceInt {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(int) = ob.extract::<i64>() {
            Ok(PaceInt(Some(int)))
        } else if let Ok(int) = ob.extract::<i32>() {
            Ok(PaceInt(Some(int as i64)))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "Expected an integer",
            ))
        }
    }
}

impl<'py> FromPyObject<'py> for PaceBool {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(bool) = ob.extract::<bool>() {
            Ok(PaceBool(Some(bool)))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Expected a boolean"))
        }
    }
}
// impl Default for PaceSeries<PaceAny> {
//     fn default() -> Self {
//         return PaceSeries::new();
//     }
// }
// #endregion

#[derive(Debug, Clone)]
pub struct GlobalSeries {
    ctx: PaceContext,
    pub open: PaceSeries<PaceFloat>,
    pub high: PaceSeries<PaceFloat>,
    pub low: PaceSeries<PaceFloat>,
    pub close: PaceSeries<PaceFloat>,
    pub volume: PaceSeries<PaceFloat>,
    pub hl2: PaceSeries<PaceFloat>,
    pub hlc3: PaceSeries<PaceFloat>,
    pub hlcc4: PaceSeries<PaceFloat>,
    pub bar_index: PaceSeries<PaceInt>,
    pub last_bar_index: PaceSeries<PaceInt>,
    pub time: PaceSeries<PaceInt>,
    pub time_close: PaceSeries<PaceInt>,
}

impl GlobalSeries {
    pub fn new(ctx: PaceContext) -> Self {
        return Self {
            ctx,
            open: PaceSeries::new(),
            high: PaceSeries::new(),
            low: PaceSeries::new(),
            close: PaceSeries::new(),
            volume: PaceSeries::new(),
            hl2: PaceSeries::new(),
            hlc3: PaceSeries::new(),
            hlcc4: PaceSeries::new(),
            bar_index: PaceSeries::new(),
            last_bar_index: PaceSeries::new(),
            time: PaceSeries::new(),
            time_close: PaceSeries::new(),
        };
    }
}

impl GlobalSeries {
    pub fn next(&mut self) {
        let bar = self.ctx.bar();
        self.open.next();
        self.high.next();
        self.low.next();
        self.close.next();
        self.volume.next();
        self.hl2.next();
        self.hlc3.next();
        self.hlcc4.next();
        //
        self.open.set(bar.open().into());
        self.high.set(bar.high().into());
        self.low.set(bar.low().into());
        self.close.set(bar.close().into());
        self.volume.set(bar.volume().into());
        self.hl2.set(hl2(bar.high(), bar.low()).into());
        self.hlc3
            .set(hlc3(bar.high(), bar.low(), bar.close()).into());
        self.hlcc4
            .set(hlcc4(bar.high(), bar.low(), bar.close()).into());

        self.bar_index.set(self.ctx.bar_index().into());
        self.last_bar_index.set(self.ctx.last_bar_index().into());

        self.time.set(bar.open_time().into());
        self.time_close.set(bar.close_time().into());
    }

    pub fn from_name(&self, name: &str) -> Option<&PaceSeries<PaceFloat>> {
        match name {
            "open" => Some(&self.open),
            "high" => Some(&self.high),
            "low" => Some(&self.low),
            "close" => Some(&self.close),
            "volume" => Some(&self.volume),
            "hl2" => Some(&self.hl2),
            "hlc3" => Some(&self.hlc3),
            "hlcc4" => Some(&self.hlcc4),
            _ => None,
        }
    }
}
