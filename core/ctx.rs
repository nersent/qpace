use std::{
    cell::{Cell, Ref, RefCell, RefMut},
    rc::Rc,
    sync::Arc,
};

use crate::{
    ohlcv::{OhlcvBar, OhlcvReader},
    sym::SymInfo,
};
cfg_if::cfg_if! { if #[cfg(feature = "bindings_py")] {
  use pyo3::prelude::*;
  use pyo3_stub_gen::{derive::{gen_stub_pyclass, gen_stub_pymethods, gen_stub_pyclass_enum}};
  use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    types::{PySequence, PySlice, PySliceIndices},
  };
  use crate::rs_utils::{pyslice_to_range};
}}
cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}

pub struct Ctx {
    ohlcv: Box<dyn OhlcvReader>,
    sym_info: SymInfo,
    bar_index: usize,
    is_initialized: bool,
}

impl Ctx {
    #[inline]
    pub fn new(ohlcv: Box<dyn OhlcvReader>, sym_info: SymInfo) -> Self {
        Self {
            ohlcv,
            sym_info,
            bar_index: 0,
            is_initialized: false,
        }
    }

    #[inline]
    pub fn fork(&self) -> Self {
        Self::new(self.ohlcv.clone_box(), self.sym_info.clone())
    }

    #[inline]
    pub fn bar_index(&self) -> usize {
        self.bar_index
    }

    #[inline]
    pub fn bar_at(&self, index: usize) -> &OhlcvBar {
        self.ohlcv.bar(index)
    }

    #[inline]
    pub fn bar(&self) -> &OhlcvBar {
        self.bar_at(self.bar_index())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.ohlcv.len()
    }

    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    #[inline]
    pub fn sym_info(&self) -> &SymInfo {
        &self.sym_info
    }

    #[inline]
    pub fn ohlcv(&self) -> &dyn OhlcvReader {
        self.ohlcv.as_ref()
    }

    #[inline]
    pub fn next(&mut self) -> Option<usize> {
        let bar_index = if self.is_initialized {
            self.bar_index + 1
        } else {
            0
        };
        if bar_index >= self.ohlcv.len() {
            return None;
        }
        self.bar_index = bar_index;
        self.is_initialized = true;
        return Some(bar_index);
    }
}

impl Iterator for &mut Ctx {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        return Ctx::next(self);
    }
}
