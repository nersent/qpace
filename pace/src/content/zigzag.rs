use std::time::Duration;

use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
    pinescript::common::PineScriptFloat64,
};

pub enum DifferencePriceMode {
    Absolute,
    Percent,
}

pub struct ZigZagConfig {
    pub dev_threshold: f64,
    pub depth: usize,
    pub difference_price_mode: DifferencePriceMode,
    pub allow_zig_zag_on_one_bar: bool,
    pub use_close: bool,
}

impl Default for ZigZagConfig {
    fn default() -> Self {
        Self {
            dev_threshold: 5.0,
            depth: 10,
            difference_price_mode: DifferencePriceMode::Absolute,
            allow_zig_zag_on_one_bar: true,
            use_close: false,
        }
    }
}

// https://www.tradingview.com/pine-script-reference/v5/#type_chart.point
#[derive(Clone, Debug)]
pub struct ChartPoint {
    pub index: Option<usize>,
    pub time: Option<Duration>,
    pub price: f64,
}

// https://www.tradingview.com/pine-script-reference/v5/#type_line
// https://www.tradingview.com/pine-script-reference/v5/#fun_line.new
#[derive(Clone, Debug)]
pub struct Line {
    pub first_point: ChartPoint,
    pub second_point: ChartPoint,
}

#[derive(Clone, Debug)]
pub struct Label {}

#[derive(Clone, Debug)]
pub struct Pivot {
    pub ln: Line,
    pub lb: Option<Label>,
    pub is_high: bool,
    pub vol: f64,
    pub start: ChartPoint,
    pub end: ChartPoint,
}

impl Pivot {
    fn is_more_price(&self, point: &ChartPoint) -> bool {
        // int m = this.isHigh ? 1 : -1
        // bool result = point.price * m > this.end.price * m
        let m = if self.is_high { 1.0 } else { -1.0 };
        return point.price * m > self.end.price * m;
    }

    fn update_pivot(&mut self, end: &ChartPoint, vol: f64) {
        // this.end := end
        // this.vol := vol
        self.end = end.clone();
        self.vol = vol;
        // if not na(this.lb)
        //  this.lb.set_point(this.end)
        //  this.lb.set_text(priceRotationAggregate(this.start.price, this.end.price, this.vol, settings))
        self.ln.second_point = end.clone();
    }
}

pub struct ZigZag {
    pub ctx: Context,
    pub config: ZigZagConfig,
    pub sum_vol: f64,
    pub pivots: Vec<Pivot>,
    pub depth: usize, // math.max(2, math.floor(this.settings.depth / 2))
    pub high_find_pivot: ZigZagFindPivot,
    pub low_find_pivot: ZigZagFindPivot,
}

impl ZigZag {
    pub fn new(ctx: Context, config: ZigZagConfig) -> Self {
        let depth = f64::max(2.0, f64::floor(config.depth as f64 / 2.0)) as usize;
        return Self {
            ctx: ctx.clone(),
            sum_vol: 0.0,
            pivots: vec![],
            depth,
            high_find_pivot: ZigZagFindPivot::new(ctx.clone(), true, depth),
            low_find_pivot: ZigZagFindPivot::new(ctx.clone(), false, depth),
            config,
        };
    }

    pub fn update(&mut self) -> bool {
        // this.sumVol += nz(volume[depth])
        self.sum_vol += self.ctx.volume(self.depth).ps_nz();

        let _high = if self.config.use_close {
            self.ctx.bar.close()
        } else {
            self.ctx.bar.high()
        };
        let _low = if self.config.use_close {
            self.ctx.bar.close()
        } else {
            self.ctx.bar.low()
        };

        let high_pivot = self.high_find_pivot.next(_high);
        let low_pivot = self.low_find_pivot.next(_low);

        let mut something_changed = false;

        if let Some(mut high_pivot) = high_pivot {
            something_changed = self.new_pivot_point_found(true, &high_pivot);
        }
        if self.config.allow_zig_zag_on_one_bar || !something_changed {
            if let Some(mut low_pivot) = low_pivot {
                something_changed = self.new_pivot_point_found(false, &low_pivot);
            }
        }

        // bool somethingChanged = this.tryFindPivot(high, true, depth)

        return something_changed;
    }

    fn new_pivot_point_found(&mut self, is_high: bool, chart_point: &ChartPoint) -> bool {
        let mut result = false;

        let pivots_len = self.pivots.len();
        let last_pivot = self.pivots.last_mut();

        let mut pivot_to_push: Option<Pivot> = None;

        if let Some(last_pivot) = last_pivot {
            if last_pivot.is_high == is_high {
                if last_pivot.is_more_price(chart_point) {
                    if pivots_len == 1 {
                        last_pivot.start = chart_point.clone();
                    }
                    last_pivot.update_pivot(chart_point, last_pivot.vol + self.sum_vol);
                    self.sum_vol = 0.0;

                    result = true;
                }
            } else {
                // float dev = calcDev(lastPivot.end.price, point.price)
                // let dev = self.calc_dev(last_pivot.end.price, chart_point.price);
                // let dev = 100.0 * (price - base_price) / base_price.abs();
                let dev =
                    100.0 * (chart_point.price - last_pivot.end.price) / last_pivot.end.price.abs();
                //     if (not lastPivot.isHigh and dev >= this.settings.devThreshold) or
                //     (lastPivot.isHigh and dev <= -1 * this.settings.devThreshold)
                //    newPivotFound(this, newPivot(lastPivot.end, point, this.sumVol, isHigh, this.settings))
                //    result := true
                if (!last_pivot.is_high && dev >= self.config.dev_threshold)
                    || (last_pivot.is_high && dev <= -1.0 * self.config.dev_threshold)
                {
                    // newPivotFound(this, newPivot(lastPivot.end, point, this.sumVol, isHigh, this.settings))
                    pivot_to_push = Some(Pivot {
                        ln: Line {
                            first_point: last_pivot.end.clone(),
                            second_point: chart_point.clone(),
                        },
                        lb: None,
                        is_high,
                        vol: self.sum_vol,
                        start: last_pivot.end.clone(),
                        end: chart_point.clone(),
                    });
                    self.sum_vol = 0.0;
                    result = true;
                }
            }
        } else {
            //  this.newPivotFound(newPivot(point, point, this.sumVol, isHigh, this.settings))
            pivot_to_push = Some(Pivot {
                ln: Line {
                    first_point: chart_point.clone(),
                    second_point: chart_point.clone(),
                },
                lb: None,
                is_high,
                vol: self.sum_vol,
                start: chart_point.clone(),
                end: chart_point.clone(),
            });
            self.sum_vol = 0.0;
            result = true;
        }

        if let Some(pivot) = pivot_to_push {
            self.pivots.push(pivot);
        }

        return result;
    }
}

impl Incremental<(), ()> for ZigZag {
    fn next(&mut self, x: ()) {}
}

pub struct ZigZagFindPivot {
    pub ctx: Context,
    pub is_high: bool,
    pub length: usize,
    src: FloatSeries,
}

impl ZigZagFindPivot {
    pub fn new(ctx: Context, is_high: bool, length: usize) -> Self {
        return Self {
            src: FloatSeries::new(ctx.clone()),
            ctx,
            is_high,
            length,
        };
    }
}

impl Incremental<f64, Option<ChartPoint>> for ZigZagFindPivot {
    fn next(&mut self, x: f64) -> Option<ChartPoint> {
        // float pivotPrice = nz(src[length])
        self.src.next(x);
        let pivot_price = self.src.get(self.length).ps_nz();
        // if length == 0
        //  chart.point.new(time, bar_index, pivotPrice)
        if self.length == 0 {
            return Some(ChartPoint {
                index: Some(self.ctx.bar.index()),
                time: self.ctx.bar.time(),
                price: pivot_price,
            });
        }
        // else if length * 2 <= bar_index
        if self.length * 2 <= self.ctx.bar.index() {
            // bool isFound = true
            let mut is_found = true;
            // for i = 0 to math.abs(length - 1)
            for i in 0..=(self.length - 1) {
                //  if (isHigh and src[i] > pivotPrice) or (not isHigh and src[i] < pivotPrice)
                if (self.is_high && self.src.get(i) > pivot_price)
                    || (!self.is_high && self.src.get(i) < pivot_price)
                {
                    //  isFound := false
                    // break
                    is_found = false;
                    break;
                }
            }
            // for i = length + 1 to 2 * length
            for i in (self.length + 1)..=(2 * self.length) {
                // if (isHigh and src[i] >= pivotPrice) or (not isHigh and src[i] <= pivotPrice)
                if (self.is_high && self.src.get(i) >= pivot_price)
                    || (!self.is_high && self.src.get(i) <= pivot_price)
                {
                    // isFound := false
                    // break
                    is_found = false;
                    break;
                }
            }
            if is_found {
                return Some(ChartPoint {
                    index: self.ctx.bar_index(self.length),
                    time: self.ctx.time(self.length),
                    price: pivot_price,
                });
            }
        }

        return None;
    }
}
