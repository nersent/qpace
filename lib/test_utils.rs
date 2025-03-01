use colored::Colorize;
use std::path::PathBuf;

use crate::rs_utils::Float64Utils;

#[cfg(test)]
#[cfg(feature = "polars_utils")]
pub fn format_fixture_path(path: &str) -> PathBuf {
    let cwd = std::env::current_dir().unwrap();
    let normalized_path = cwd.join("fixtures").join(path);
    return normalized_path;
}

#[cfg(test)]
pub struct ArraySnapshot<T> {
    pub debug_mode: bool,
    pub print_max_index: Option<usize>,
    pub actual: Vec<T>,
    pub name: Option<String>,
    pub precision: f64,
}

#[cfg(test)]
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

#[cfg(test)]
impl ArraySnapshot<f64> {
    pub fn assert(&self, expected: &[f64]) {
        self.assert_iter(expected, |actual, expected| {
            (*actual).compare_with_precision(*expected, self.precision)
        });
    }
}
