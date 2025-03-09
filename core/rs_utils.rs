cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::gen_stub_pyfunction};
  use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    types::{PySequence, PySlice, PySliceIndices},
  };
  use pyo3_stub_gen::PyStubType;
  use pyo3_stub_gen::TypeInfo;
}}
cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}
cfg_if::cfg_if! { if #[cfg(feature = "polars")] {
  use polars::prelude::*;
  use polars::frame::DataFrame;
}}
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use pyo3::conversion::FromPyObjectBound;
use std::time::Duration;
use std::{ffi::OsStr, ops::Range, path::Path};

cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  #[inline]
  pub fn pyslice_to_range(pyslice: &Bound<'_, PySlice>, max_length: usize) -> Range<usize> {
    let idx = pyslice.indices(max_length as isize).unwrap();
    let start = idx.start;
    let stop = idx.stop;
    let step = idx.step;
    assert!(step >= 0, "Negative step is not supported");
    // @TODO: .step_by(step as usize);
    return start as usize..stop as usize;
  }

  pub struct PandasDataFrame(pub PyObject);

  impl PyStubType for PandasDataFrame {
      fn type_output() -> TypeInfo {
          TypeInfo::with_module("pandas.DataFrame", "pandas".into())
      }
  }

  impl Into<PandasDataFrame> for PyObject {
      fn into(self) -> PandasDataFrame {
          PandasDataFrame(self)
      }
  }

  impl IntoPy<PyObject> for PandasDataFrame {
      fn into_py(self, py: Python<'_>) -> PyObject {
          self.0
      }
  }
}}

#[inline]
pub fn get_oldest_possible_datetime() -> DateTime<Utc> {
    return Utc.ymd(1, 1, 1).and_hms(0, 0, 0);
}

#[inline]
pub fn get_filename_extension(path: &Path) -> Option<&str> {
    return path.extension().and_then(OsStr::to_str);
}

cfg_if::cfg_if! { if #[cfg(feature = "polars")] {
  pub trait SeriesCastUtils {
      fn to_bool(&self) -> Vec<Option<bool>>;
      fn to_f64(&self) -> Vec<f64>;
      fn to_i32(&self) -> Vec<Option<i32>>;
      fn to_i64(&self) -> Vec<Option<i64>>;
      fn to_i128(&self) -> Vec<Option<i128>>;
      fn to_usize(&self) -> Vec<Option<usize>>;
      fn to_duration(&self) -> Vec<Option<Duration>>;
      fn to_str(&self) -> Vec<Option<String>>;
      fn to_datetime_from_s(&self) -> Vec<Option<DateTime<Utc>>>;
      fn to_datetime_from_ms(&self) -> Vec<Option<DateTime<Utc>>>;
  }

  impl SeriesCastUtils for Series {
      fn to_bool(&self) -> Vec<Option<bool>> {
          let f64 = self.to_f64();
          return f64
              .into_iter()
              .map(|val| {
                  if val.is_nan() {
                      None
                  } else {
                      Some(val as i32 == 1)
                  }
              })
              .collect::<Vec<_>>();
      }

      fn to_str(&self) -> Vec<Option<String>> {
          return self
              .cast(&DataType::Utf8)
              .unwrap()
              .utf8()
              .unwrap()
              .into_iter()
              .map(|val| {
                  if val.is_none() {
                      None
                  } else {
                      val.map(|x| x.to_string())
                  }
              })
              .collect::<Vec<_>>();
      }

      fn to_f64(&self) -> Vec<f64> {
          return self
              .cast(&DataType::Float64)
              .unwrap()
              .f64()
              .unwrap()
              .into_iter()
              .map(|val| {
                  if val.is_none() || val.unwrap().is_nan() {
                      f64::NAN
                  } else {
                      val.unwrap()
                  }
              })
              .collect::<Vec<_>>();
      }

      fn to_i32(&self) -> Vec<Option<i32>> {
        return self
            .cast(&DataType::Int32)
            .unwrap()
            .i32()
            .unwrap()
            .into_iter()
            .map(|val| if val.is_none() { None } else { val })
            .collect::<Vec<_>>();
    }


      fn to_i64(&self) -> Vec<Option<i64>> {
          return self
              .cast(&DataType::Int64)
              .unwrap()
              .i64()
              .unwrap()
              .into_iter()
              .map(|val| if val.is_none() { None } else { val })
              .collect::<Vec<_>>();
      }


        fn to_i128(&self) -> Vec<Option<i128>> {
            return self
                .cast(&DataType::Int64)
                .unwrap()
                .i64()
                .unwrap()
                .into_iter()
                .map(|val| if val.is_none() { None } else { Some(val.unwrap() as i128) })
                .collect::<Vec<_>>();
        }

      fn to_usize(&self) -> Vec<Option<usize>> {
          return self
              .cast(&DataType::UInt64)
              .unwrap()
              .u64()
              .unwrap()
              .into_iter()
              .map(|val| {
                  if val.is_none() {
                      None
                  } else {
                      val.map(|x| x as usize)
                  }
              })
              .collect::<Vec<_>>();
      }

      fn to_duration(&self) -> Vec<Option<Duration>> {
          return self
              .cast(&DataType::Float64)
              .unwrap()
              .f64()
              .unwrap()
              .into_iter()
              .map(|val| {
                  if val.is_none() {
                      None
                  } else {
                      Some(Duration::from_secs_f64(val.unwrap()))
                  }
              })
              .collect::<Vec<_>>();
      }

      fn to_datetime_from_s(&self) -> Vec<Option<DateTime<Utc>>> {
        return self
            .to_i64()
            .into_iter()
            .map(|val| {
                if val.is_none() {
                    None
                } else {
                    Some(DateTime::from_timestamp(val.unwrap(), 0).unwrap())
                }
            })
            .collect::<Vec<_>>();
    }

      fn to_datetime_from_ms(&self) -> Vec<Option<DateTime<Utc>>> {
          return self
              .to_i64()
              .into_iter()
              .map(|val| {
                  if val.is_none() {
                      None
                  } else {
                      Some(DateTime::from_timestamp_millis(val.unwrap()).unwrap())
                  }
              })
              .collect::<Vec<_>>();
      }
  }

  #[inline]
  pub fn read_df_csv(path: &Path) -> DataFrame {
      let mut file = std::fs::File::open(path).unwrap();
      let df = CsvReader::new(&mut file)
          .infer_schema(None)
          .finish()
          .unwrap();
      return df;
  }

  #[inline]
  pub fn read_df_parquet(path: &Path) -> DataFrame {
      let mut file = std::fs::File::open(path).unwrap();
      let df = ParquetReader::new(&mut file).finish().unwrap();
      return df;
  }

  #[inline]
  pub fn read_df(path: &Path) -> DataFrame {
      let extension = get_filename_extension(path);
      match extension {
          Some("parquet") => read_df_parquet(path),
          Some("csv") => read_df_csv(path),
          Some(&_) => panic!("Unsupported file type"),
          None => panic!("Unsupported file type for path {:?}", path.display()),
      }
  }
}}

pub trait Float64Utils {
    const PRICE_PRECISION: f64;
    fn to_option(self) -> Option<f64>;
    fn normalize(self) -> f64;
    fn is_zero(self) -> bool;
    fn compare_with_precision(&self, target: f64, precision: f64) -> bool;
    fn compare(&self, target: f64) -> bool;
    fn is_non_zero(self) -> bool;
}

impl Float64Utils for f64 {
    const PRICE_PRECISION: f64 = 0.01;

    fn to_option(self) -> Option<f64> {
        if self.is_nan() {
            return None;
        }
        return Some(self);
    }

    fn normalize(self) -> f64 {
        if self.is_normal() {
            return self;
        }
        return f64::NAN;
    }

    fn is_zero(self) -> bool {
        return !self.is_nan() && self.compare(0.0);
    }

    fn is_non_zero(self) -> bool {
        return !self.is_nan() && !self.compare(0.0);
    }

    fn compare_with_precision(&self, target: f64, precision: f64) -> bool {
        if self.is_nan() {
            return target.is_nan();
        }
        return (self - target).abs() < precision;
    }

    fn compare(&self, target: f64) -> bool {
        return self.compare_with_precision(target, 0.00001);
    }
}

pub trait OptionFloatUtils {
    fn unwrap_nan(self) -> f64;
}

impl OptionFloatUtils for Option<f64> {
    fn unwrap_nan(self) -> f64 {
        return self.unwrap_or(f64::NAN);
    }
}

impl OptionFloatUtils for Option<i32> {
    fn unwrap_nan(self) -> f64 {
        return self.map_or(f64::NAN, |v| v as f64);
    }
}

impl OptionFloatUtils for Option<usize> {
    fn unwrap_nan(self) -> f64 {
        return self.map_or(f64::NAN, |v| v as f64);
    }
}

pub fn with_suffix(suffix: &str) -> impl Fn(f64) -> String {
    let suffix = suffix.to_string();
    move |value| format!("{:0.2}{}", value, suffix)
}
