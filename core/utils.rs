use chrono::{DateTime, TimeZone, Utc};
cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3::{types::{PySlice}};
  use pyo3_stub_gen::PyStubType;
  use pyo3_stub_gen::TypeInfo;
}}
use std::{ffi::OsStr, ops::Range, path::Path};
cfg_if::cfg_if! { if #[cfg(feature = "polars")] {
  use polars::prelude::*;
  use polars::frame::DataFrame;
}}
use std::time::Duration;

#[inline]
pub fn get_filename_extension(path: &Path) -> Option<&str> {
    return path.extension().and_then(OsStr::to_str);
}

#[inline]
pub fn get_oldest_possible_datetime() -> DateTime<Utc> {
    return Utc.ymd(1, 1, 1).and_hms(0, 0, 0);
}

#[cfg(feature = "bindings_py")]
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

#[cfg(feature = "polars")]
#[inline]
pub fn read_df_csv(path: &Path) -> Result<DataFrame, PolarsError> {
    let mut file = std::fs::File::open(path).unwrap();
    return CsvReader::new(&mut file).infer_schema(None).finish();
}

#[cfg(feature = "polars")]
#[inline]
pub fn read_df_parquet(path: &Path) -> Result<DataFrame, PolarsError> {
    let mut file = std::fs::File::open(path).unwrap();
    return ParquetReader::new(&mut file).finish();
}

#[cfg(feature = "polars")]
#[inline]
pub fn read_df(path: &Path) -> Result<DataFrame, String> {
    let ext = get_filename_extension(path);
    let err = match ext {
        Some("parquet") => read_df_parquet(path).map_err(|e| e.to_string()),
        Some("csv") => read_df_csv(path).map_err(|e| e.to_string()),
        Some(_) => Err("Unsupported file format".into()),
        None => Err("No file extension".into()),
    };
    match err {
        Ok(df) => Ok(df),
        Err(e) => Err(format!("Error reading file {:?}: {}", path.display(), e)),
    }
}

#[cfg(feature = "polars")]
#[inline]
pub fn write_df_csv(path: &Path, df: &mut DataFrame) -> Result<(), PolarsError> {
    let mut file = std::fs::File::create(path).unwrap();
    CsvWriter::new(&mut file).has_header(true).finish(df)
}

#[cfg(feature = "polars")]
#[inline]
pub fn write_df_parquet(path: &Path, df: &mut DataFrame) -> Result<u64, PolarsError> {
    let mut file = std::fs::File::create(path).unwrap();
    ParquetWriter::new(&mut file).finish(df)
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
}
}

#[cfg(feature = "bindings_py")]
#[cfg(feature = "polars")]
pub struct PandasDataFrame(pub PyObject);

#[cfg(feature = "bindings_py")]
#[cfg(feature = "polars")]
impl PyStubType for PandasDataFrame {
    fn type_output() -> TypeInfo {
        TypeInfo::with_module("pandas.DataFrame", "pandas".into())
    }
}

#[cfg(feature = "bindings_py")]
#[cfg(feature = "polars")]
impl Into<PandasDataFrame> for PyObject {
    fn into(self) -> PandasDataFrame {
        PandasDataFrame(self)
    }
}

#[cfg(feature = "bindings_py")]
#[cfg(feature = "polars")]
impl IntoPy<PyObject> for PandasDataFrame {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.0
    }
}

pub fn with_suffix(suffix: &str) -> impl Fn(f64) -> String {
    let suffix = suffix.to_string();
    move |value| format!("{:0.2}{}", value, suffix)
}
