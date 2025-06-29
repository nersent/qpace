use crate::ohlcv::OhlcvReader;
use crate::ohlcv::OhlcvWriter;
use crate::{ctx::Ctx, ohlcv::Ohlcv};
use colored::Colorize;
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
};
cfg_if::cfg_if! { if #[cfg(feature = "polars")] {
use polars::frame::DataFrame;
use crate::utils::{read_df, SeriesCastUtils};
}}

/// Recursive building block that accepts an input and produces an output imlicitly.
pub trait Incremental<T, R> {
    /// It is recommended that `next` method is called on every tick, even if the input is `None`.
    fn next(&mut self, input: T) -> R;

    fn to_box(self) -> Box<Self>
    where
        Self: Sized,
    {
        return Box::new(self);
    }

    // /// Should be called before calling `next`
    // fn next_bar(&mut self) {}
}

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

pub trait PineFloat64 {
    fn ps_is_zero(self) -> bool;
    fn ps_is_not_zero(self) -> bool;
    /// Same as `sign` in Pine.
    fn ps_sign(self) -> f64;
    /// Same as `na` in Pine.
    fn ps_na(self) -> bool;
    /// Same as `nz` in Pine.
    fn ps_nz(self) -> f64;
    /// Same as `nz(value, replacement)` in Pine.
    fn ps_nz_with(self, replacement: f64) -> f64;
    /// Returns the maximum of two values. Same as `math.max` in Pine.
    fn ps_max(self, other: f64) -> f64;
    /// Returns the minimum of two values. Same as `math.min` in Pine.
    fn ps_min(self, other: f64) -> f64;
    fn ps_to_bool(self) -> bool;
    fn ps_normalize(self) -> f64;
    fn ps_log(self) -> f64;
    fn ps_exp(self) -> f64;
}

/// Returns the absolute value of a number. Same as `math.abs` in Pine.
// fn ps_abs(value: f64) -> f64;

impl PineFloat64 for f64 {
    fn ps_is_zero(self) -> bool {
        return !self.is_nan() && self == 0.0;
    }

    fn ps_is_not_zero(self) -> bool {
        return !self.is_nan() && self != 0.0;
    }

    fn ps_sign(self) -> f64 {
        if self.is_nan() {
            return f64::NAN;
        }
        if self > 0.0 {
            return 1.0;
        }
        if self < 0.0 {
            return -1.0;
        }
        return 0.0;
    }

    fn ps_na(self) -> bool {
        return self.is_nan();
    }

    fn ps_nz(self) -> f64 {
        if self.is_nan() {
            return 0.0;
        }
        return self;
    }

    fn ps_nz_with(self, replacement: f64) -> f64 {
        if self.is_nan() {
            return replacement;
        }
        return self;
    }

    fn ps_max(self, other: f64) -> f64 {
        if self.is_nan() || other.is_nan() {
            return f64::NAN;
        }
        return f64::max(self, other);
    }

    fn ps_min(self, other: f64) -> f64 {
        if self.is_nan() || other.is_nan() {
            return f64::NAN;
        }
        return f64::min(self, other);
    }

    fn ps_to_bool(self) -> bool {
        return !self.is_zero();
    }

    fn ps_normalize(self) -> f64 {
        if self.is_infinite() {
            return f64::NAN;
        }
        return self;
    }

    fn ps_log(self) -> f64 {
        return self.ln().ps_normalize();
    }

    fn ps_exp(self) -> f64 {
        return self.exp().ps_normalize();
    }

    // fn ps_abs(value: f64) -> f64 {
    //     if value.is_nan() {
    //         return f64::NAN;
    //     }
    //     return f64::abs(value);
    // }
}

pub struct FloatSeries {
    pub values: Vec<f64>,
    current_offset: usize,
}

impl FloatSeries {
    pub fn new() -> Self {
        return Self {
            values: vec![],
            current_offset: 0,
        };
    }

    pub fn with_initial_value(mut self, value: f64) -> Self {
        self.values.push(value);
        return self;
    }

    fn push(&mut self, value: f64) {
        self.current_offset = self.values.len();
        self.values.push(value);
    }

    pub fn size(&self) -> usize {
        return self.values.len();
    }

    pub fn is_filled(&self, size: usize) -> bool {
        return self.values.len() >= size;
    }

    /// Returns **`N - I`** previous value.
    pub fn get(&self, index: usize) -> f64 {
        if index >= self.values.len() {
            return f64::NAN;
        }
        let index = (self.values.len() - 1) - index;
        return self.values[index];
    }

    pub fn at(&self, index: usize) -> f64 {
        if index >= self.values.len() {
            return f64::NAN;
        }
        return self.values[index];
    }

    // /// Returns all **`N`** previous values.
    pub fn window(&self, length: usize) -> &[f64] {
        return &self.values[self.values.len() - length..];
    }

    // pub fn offset_window(&self, length: usize, offset: usize) -> &[f64] {
    //     let size = self.values.len();
    //     return &self.values[size - length - offset..size - offset];
    // }

    // /// Returns previous value.
    // pub fn last(&mut self) -> Option<&T> {
    //     return self.values.last();
    // }

    // /// Returns **`N`** previous value (first value of the window).
    // pub fn first(&mut self) -> Option<&T> {
    //     let size = self.values.len();
    //     if size < self.length {
    //         return None;
    //     }
    //     return self.get(self.length - 1);
    // }
}

impl std::ops::Index<usize> for FloatSeries {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.values[self.current_offset - index];
    }
}

impl Incremental<f64, ()> for FloatSeries {
    fn next(&mut self, value: f64) {
        self.push(value);
    }
}

pub fn format_pace_fixture_path(path: &str) -> PathBuf {
    // let mut normalized_path = Path::new("pace/lib/fixtures").join(path);
    // let test_mode = std::env::var("NEXTEST").is_ok();

    // if test_mode {
    //     normalized_path = Path::new("../").join(normalized_path);
    // }

    // return normalized_path;
    let cwd = std::env::current_dir().unwrap();
    let normalized_path = cwd.join("fixtures").join(path);
    return normalized_path;
}

pub struct Fixture {}

impl Fixture {
    #[cfg(feature = "polars")]
    pub fn load(path: &Path) -> (DataFrame, Rc<RefCell<Ctx>>) {
        let df = read_df(&path).unwrap();
        let mut ohlcv = Ohlcv::new();
        ohlcv.read_polars(&df);
        let mut ctx = Ctx::new();
        ctx.set_ohlcv(ohlcv.into_box());
        return (df, Rc::new(RefCell::new(ctx)));
    }
}

pub trait DataFrameFixtureUtils {
    fn test_target(&self) -> Vec<f64>;
}

#[cfg(feature = "polars")]
impl DataFrameFixtureUtils for DataFrame {
    fn test_target(&self) -> Vec<f64> {
        return self.column("_target_").unwrap().to_f64();
    }
}

pub struct ArraySnapshot<T> {
    pub debug_mode: bool,
    pub print_max_index: Option<usize>,
    pub actual: Vec<T>,
    pub name: Option<String>,
    pub precision: f64,
}

pub trait Compare<T> {
    fn compare(&self, other: &T) -> bool;
}

impl<T: std::fmt::Debug> ArraySnapshot<T> {
    pub fn new() -> Self {
        return ArraySnapshot::<T> {
            actual: Vec::new(),
            debug_mode: false,
            print_max_index: None,
            name: None,
            precision: 0.00001,
        };
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        return self;
    }

    pub fn with_precision(mut self, precision: f64) -> Self {
        self.precision = precision;
        return self;
    }

    pub fn debug_mode(&mut self) {
        self.debug_mode = true;
    }

    pub fn debug_mode_max(&mut self, max_index: usize) {
        self.print_max_index = Some(max_index);
        self.debug_mode();
    }

    pub fn actual(&mut self, value: Vec<T>) {
        self.actual = value;
    }

    pub fn push(&mut self, value: T) {
        self.actual.push(value);
    }

    pub fn assert_iter(&self, expected: &[T], compare_delegate: impl Fn(&T, &T) -> bool) {
        assert_eq!(
            self.actual.len(),
            expected.len(),
            "Got different sizes | Actual: {} | Expected: {}",
            format!("{}", self.actual.len()).red(),
            format!("{}", expected.len()).green(),
        );
        for i in 0..self.actual.len() {
            let actual = &self.actual[i];
            let expected = &expected[i];
            let is_equal = compare_delegate(actual, expected);
            if !is_equal {
                println!(
                    "{}: {} | {}\n",
                    format!("[{:?}]", i).red().bold(),
                    format!("{:?}", actual).black().on_bright_red().bold(),
                    format!("{:?}", expected).black().on_green().bold(),
                );
                if !self.debug_mode {
                    let mut prefix: String = "".to_string();
                    if let Some(name) = &self.name {
                        println!("Test {} failed", name.bright_red().black().bold());
                        prefix = format!("[{}]: ", name);
                    }
                    panic!("{}Array snapshot assertion failed at index {}", prefix, i);
                }
            }
            if self.debug_mode
                && (self.print_max_index.is_none() || i < self.print_max_index.unwrap())
            {
                println!(
                    "{}: {}",
                    format!("[{:?}]", i).bright_cyan().bold(),
                    format!("{:?}", actual).white(),
                );
            }
        }
    }
}

impl ArraySnapshot<f64> {
    pub fn assert(&self, expected: &[f64]) {
        self.assert_iter(expected, |actual, expected| {
            (*actual).compare_with_precision(*expected, self.precision)
        });
    }
}

impl ArraySnapshot<(f64, f64)> {
    pub fn assert(&self, expected: &[(f64, f64)]) {
        self.assert_iter(expected, |actual, expected| {
            (actual.0).compare(expected.0) && (actual.1).compare(expected.1)
        });
    }
}

impl ArraySnapshot<(f64, f64, f64)> {
    pub fn assert(&self, expected: &[(f64, f64, f64)]) {
        self.assert_iter(expected, |actual, expected| {
            (actual.0).compare(expected.0)
                && (actual.1).compare(expected.1)
                && (actual.2).compare(expected.2)
        });
    }
}

impl ArraySnapshot<(f64, f64, f64, f64)> {
    pub fn assert(&self, expected: &[(f64, f64, f64, f64)]) {
        self.assert_iter(expected, |actual, expected| {
            (actual.0).compare(expected.0)
                && (actual.1).compare(expected.1)
                && (actual.2).compare(expected.2)
                && (actual.3).compare(expected.3)
        });
    }
}

impl ArraySnapshot<Option<i32>> {
    pub fn assert(&self, expected: &[Option<i32>]) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (Some(actual), Some(expected)) => actual == expected,
            (None, None) => true,
            _ => false,
        });
    }
}

impl ArraySnapshot<Option<f64>> {
    pub fn assert(&self, expected: &[Option<f64>]) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (Some(actual), Some(expected)) => (*actual).compare(*expected),
            (None, None) => true,
            _ => false,
        });
    }
}

impl ArraySnapshot<Option<bool>> {
    pub fn assert(&self, expected: &[Option<bool>]) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (Some(actual), Some(expected)) => actual == expected,
            (None, None) => true,
            _ => false,
        });
    }
}

impl ArraySnapshot<bool> {
    pub fn assert(&self, expected: &[bool]) {
        self.assert_iter(expected, |actual, expected| actual == expected);
    }
}

impl ArraySnapshot<Option<usize>> {
    pub fn assert(&self, expected: &[Option<usize>]) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (Some(actual), Some(expected)) => actual == expected,
            (None, None) => true,
            _ => false,
        });
    }
}

impl ArraySnapshot<(Option<f64>, Option<f64>, bool)> {
    pub fn assert(&self, expected: &[(Option<f64>, Option<f64>, bool)]) {
        self.assert_iter(expected, |actual, expected| {
            let is_first_valid = match (actual.0, expected.0) {
                (None, None) => true,
                (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                _ => false,
            };
            let is_second_valid = match (actual.1, expected.1) {
                (None, None) => true,
                (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                _ => false,
            };
            return is_first_valid && is_second_valid && actual.2 == expected.2;
        });
    }
}

impl ArraySnapshot<Option<(f64, f64)>> {
    pub fn assert(&self, expected: &[Option<(f64, f64)>]) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (None, None) => true,
            (Some(actual), Some(expected)) => {
                actual.0.compare(expected.0) && actual.1.compare(expected.1)
            }
            _ => false,
        })
    }
}

impl ArraySnapshot<Option<(Option<f64>, Option<f64>)>> {
    pub fn assert(&self, expected: &[Option<(Option<f64>, Option<f64>)>]) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (None, None) => true,
            (Some(actual), Some(expected)) => match (actual.0, expected.0) {
                (None, None) => true,
                (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                _ => false,
            },
            _ => false,
        })
    }
}

impl ArraySnapshot<Option<(Option<f64>, Option<f64>, Option<f64>)>> {
    pub fn assert(&self, expected: &[Option<(Option<f64>, Option<f64>, Option<f64>)>]) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (None, None) => true,
            (Some(actual), Some(expected)) => {
                let is_first_valid = match (actual.0, expected.0) {
                    (None, None) => true,
                    (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                    _ => false,
                };
                let is_second_valid = match (actual.1, expected.1) {
                    (None, None) => true,
                    (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                    _ => false,
                };
                let is_third_valid = match (actual.2, expected.2) {
                    (None, None) => true,
                    (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                    _ => false,
                };
                return is_first_valid && is_second_valid && is_third_valid;
            }
            _ => false,
        })
    }
}

impl ArraySnapshot<Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>> {
    pub fn assert(
        &self,
        expected: &[Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>],
    ) {
        self.assert_iter(expected, |actual, expected| match (actual, expected) {
            (None, None) => true,
            (Some(actual), Some(expected)) => {
                let is_first_valid = match (actual.0, expected.0) {
                    (None, None) => true,
                    (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                    _ => false,
                };
                let is_second_valid = match (actual.1, expected.1) {
                    (None, None) => true,
                    (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                    _ => false,
                };
                let is_third_valid = match (actual.2, expected.2) {
                    (None, None) => true,
                    (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                    _ => false,
                };
                let is_fourth_valid = match (actual.3, expected.3) {
                    (None, None) => true,
                    (Some(_actual), Some(_expected)) => _actual.compare(_expected),
                    _ => false,
                };
                return is_first_valid && is_second_valid && is_third_valid && is_fourth_valid;
            }
            _ => false,
        })
    }
}
