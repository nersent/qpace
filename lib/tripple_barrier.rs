cfg_if::cfg_if! { if #[cfg(feature = "bindings_pyo3")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyfunction, gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
}}

#[cfg_attr(feature = "bindings_pyo3", gen_stub_pyclass_enum)]
#[cfg_attr(feature = "bindings_pyo3", pyclass(eq, eq_int))]
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum BarrierType {
    Upper = 1,
    Lower = -1,
    Vertical = 0,
}

pub fn get_tripple_barrier_label(
    price: &Vec<f64>,
    start_at: usize,
    upper_barrier: f64,
    lower_barrier: f64,
    hold_bars: Option<usize>,
) -> (BarrierType, usize) {
    let end_index = hold_bars.map_or(price.len(), |x| (start_at + x).min(price.len()));

    for i in start_at..end_index {
        let price = price[i];
        if price >= upper_barrier {
            return (BarrierType::Upper, i);
        } else if price <= lower_barrier {
            return (BarrierType::Lower, i);
        }
    }

    return (BarrierType::Vertical, end_index);
}

#[cfg_attr(feature = "bindings_pyo3", gen_stub_pyfunction(module = "labels"))]
#[cfg_attr(
    feature = "bindings_pyo3",
    pyfunction(name = "tripple_barrier", signature = (price, upper_barrier, lower_barrier, hold_bars=None))
)]
pub fn py_get_tripple_barrier_labels(
    py: Python<'_>,
    price: Vec<f64>,
    upper_barrier: Vec<f64>,
    lower_barrier: Vec<f64>,
    hold_bars: Option<usize>,
) -> (Vec<i32>, Vec<usize>) {
    let bars = price.len();
    let mut labels = vec![0; bars];
    let mut touch_bars = vec![0; bars];

    for i in 0..bars {
        let (label, touch_bar) =
            get_tripple_barrier_label(&price, i, upper_barrier[i], lower_barrier[i], hold_bars);

        labels[i] = label as i32;
        touch_bars[i] = touch_bar;
    }

    (labels, touch_bars)
}
