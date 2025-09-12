use chrono::{DateTime, Utc};

use crate::{
    metrics::annualization_factor,
    ohlcv::{Ohlcv, OhlcvBar, OhlcvReader},
    sym::Sym,
};

#[derive(Debug)]
pub struct Ctx {
    ohlcv: Box<dyn OhlcvReader>,
    sym: Sym,
    bar_index: usize,
    is_initialized: bool,
}

impl Ctx {
    #[inline]
    pub fn new() -> Self {
        Self {
            ohlcv: Ohlcv::default().into_box(),
            sym: Sym::default(),
            bar_index: 0,
            is_initialized: false,
        }
    }

    #[inline]
    pub fn set_ohlcv(&mut self, ohlcv: Box<dyn OhlcvReader>) {
        self.ohlcv = ohlcv;
    }

    #[inline]
    pub fn set_sym(&mut self, sym: Sym) {
        self.sym = sym;
    }

    #[inline]
    pub fn copy(&self) -> Self {
        Self {
            ohlcv: self.ohlcv.clone_box(),
            sym: self.sym.clone(),
            bar_index: self.bar_index,
            is_initialized: self.is_initialized,
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.bar_index = 0;
        self.is_initialized = false;
    }

    #[inline]
    pub fn bar_index(&self) -> usize {
        self.bar_index
    }

    #[inline]
    pub fn last_bar_index(&self) -> usize {
        self.ohlcv.len().saturating_sub(1)
    }

    #[inline]
    pub fn bar(&self) -> OhlcvBar {
        self.ohlcv.get(self.bar_index()).unwrap()
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
    pub fn sym(&self) -> &Sym {
        &self.sym
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

    #[inline]
    pub fn skip(&mut self, skip: CtxSkip) {
        self.bar_index = skip.get_target_bar_index(&self)
    }

    #[inline]
    pub fn annualization_factor(&self) -> f64 {
        return annualization_factor(self.ohlcv.timeframe(), self.sym.kind().periods());
    }
}

impl Iterator for &mut Ctx {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        return Ctx::next(self);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CtxSkip {
    End,
    Bars(usize),
    BarIndex(usize),
    OpenTimeEq(DateTime<Utc>),
    OpenTimeGeq(DateTime<Utc>),
}

impl CtxSkip {
    #[inline]
    pub fn get_target_bar_index(&self, ctx: &Ctx) -> usize {
        let current = ctx.bar_index();
        let all = ctx.ohlcv.len();
        let target: usize = match self {
            CtxSkip::End => all - 1,
            CtxSkip::Bars(bars) => current + bars,
            CtxSkip::BarIndex(bar_index) => *bar_index,
            CtxSkip::OpenTimeEq(open_time) => {
                if let Some(index) = ctx.ohlcv.find_bar_index_open_time_eq(open_time) {
                    index
                } else {
                    all - 1
                }
            }
            CtxSkip::OpenTimeGeq(open_time) => {
                if let Some(index) = ctx.ohlcv.find_bar_index_open_time_geq(open_time) {
                    index
                } else {
                    all - 1
                }
            }
        };
        return target.min(all - 1);
    }
}
