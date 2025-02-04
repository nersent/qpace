#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use polars::frame::DataFrame;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            williams_percent_range::{WilliamsPercentRange, WilliamsPercentRangeConfig},
            zigzag::{DifferencePriceMode, ZigZag, ZigZagConfig, ZigZagFindPivot},
        },
        core::incremental::Incremental,
        polars::{dataframe::DataFrameUtils, series::SeriesCastUtils},
        ta::{
            moving_average::{Ma, MaKind},
            simple_moving_average::Sma,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
        utils::float::OptionFloatUtils,
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/content/zigzag/{}", path))
    }

    #[test]
    fn find_pivot_high() {
        for is_high in [true, false].iter().cloned() {
            let (df, ctx) = Fixture::load(&format_path("default.csv"));

            let mut target = ZigZagFindPivot::new(ctx.clone(), is_high, 14);

            let mut snapshot_time = ArraySnapshot::<f64>::new();
            let mut snapshot_bar_index = ArraySnapshot::<f64>::new();
            let mut snapshot_price = ArraySnapshot::<f64>::new();

            for _ in target.ctx.clone() {
                let output = target.next(ctx.bar.close());
                if let Some(output) = output {
                    snapshot_time.push(output.time.map(|x| x.as_secs_f64() * 1000.0).unwrap_nan());
                    snapshot_bar_index.push(output.index.unwrap_nan());
                    snapshot_price.push(output.price);
                } else {
                    snapshot_time.push(f64::NAN);
                    snapshot_bar_index.push(f64::NAN);
                    snapshot_price.push(f64::NAN);
                }
            }

            let target_col = if is_high { "high" } else { "low" };

            snapshot_time.assert(
                &df.column(&format!("_target_pivot_point_{}_time_", target_col))
                    .unwrap()
                    .to_f64(),
            );
            snapshot_bar_index.assert(
                &df.column(&format!("_target_pivot_point_{}_bar_index_", target_col))
                    .unwrap()
                    .to_f64(),
            );
            snapshot_price.assert(
                &df.column(&format!("_target_pivot_point_{}_price_", target_col))
                    .unwrap()
                    .to_f64(),
            );
        }
    }

    fn _test(target: &mut ZigZag, expected: &DataFrame) {
        let mut snapshot_something_changed = ArraySnapshot::<Option<bool>>::new();
        let mut snapshot_sum_vol = ArraySnapshot::<f64>::new();
        let mut snapshot_pivots_count = ArraySnapshot::<f64>::new();
        let mut snapshot_last_pivot_is_high = ArraySnapshot::<Option<bool>>::new();
        let mut snapshot_last_pivot_vol = ArraySnapshot::<f64>::new();
        let mut snapshot_last_pivot_start_bar_index = ArraySnapshot::<Option<usize>>::new();
        let mut snapshot_last_pivot_end_bar_index = ArraySnapshot::<Option<usize>>::new();
        let mut snapshot_last_pivot_start_price = ArraySnapshot::<f64>::new();
        let mut snapshot_last_pivot_end_price = ArraySnapshot::<f64>::new();

        for _ in target.ctx.clone() {
            let output = target.next(());
            let something_changed = target.update();
            let sum_vol = target.sum_vol;
            let pivots_count = target.pivots.len();
            let last_pivot = target.pivots.last();
            snapshot_something_changed.push(Some(something_changed));
            snapshot_sum_vol.push(sum_vol);
            snapshot_pivots_count.push(pivots_count as f64);
            snapshot_last_pivot_is_high.push(last_pivot.map(|x| x.is_high));
            snapshot_last_pivot_vol.push(last_pivot.map(|x| x.vol).unwrap_nan());
            snapshot_last_pivot_start_bar_index
                .push(last_pivot.map(|x| x.start.index).unwrap_or(None));
            snapshot_last_pivot_end_bar_index.push(last_pivot.map(|x| x.end.index).unwrap_or(None));
            snapshot_last_pivot_start_price.push(last_pivot.map(|x| x.start.price).unwrap_nan());
            snapshot_last_pivot_end_price.push(last_pivot.map(|x| x.end.price).unwrap_nan());
        }

        snapshot_something_changed.assert(
            &expected
                .column("_target_zigzag_something_changed_")
                .unwrap()
                .to_bool(),
        );

        snapshot_sum_vol.assert(&expected.column("_target_zigzag_sum_vol_").unwrap().to_f64());

        snapshot_pivots_count.assert(
            &expected
                .column("_target_zigzag_pivots_count_")
                .unwrap()
                .to_f64(),
        );

        snapshot_last_pivot_is_high.assert(
            &expected
                .column("_target_zigzag_last_pivot_is_high_")
                .unwrap()
                .to_bool(),
        );

        snapshot_last_pivot_vol.assert(
            &expected
                .column("_target_zigzag_last_pivot_vol_")
                .unwrap()
                .to_f64(),
        );

        snapshot_last_pivot_start_bar_index.assert(
            &expected
                .column("_target_zigzag_last_pivot_start_index_")
                .unwrap()
                .to_usize(),
        );

        snapshot_last_pivot_end_bar_index.assert(
            &expected
                .column("_target_zigzag_last_pivot_end_index_")
                .unwrap()
                .to_usize(),
        );

        snapshot_last_pivot_start_price.assert(
            &expected
                .column("_target_zigzag_last_pivot_start_price_")
                .unwrap()
                .to_f64(),
        );

        snapshot_last_pivot_end_price.assert(
            &expected
                .column("_target_zigzag_last_pivot_end_price_")
                .unwrap()
                .to_f64(),
        );
    }

    #[test]
    fn default() {
        let (df, ctx) = Fixture::load(&format_path("default.csv"));

        _test(
            &mut ZigZag::new(
                ctx.clone(),
                ZigZagConfig {
                    allow_zig_zag_on_one_bar: true,
                    depth: 10,
                    dev_threshold: 5.0,
                    difference_price_mode: DifferencePriceMode::Absolute,
                },
            ),
            &df,
        );
    }

    #[test]
    fn dev_1_legs_20() {
        let (df, ctx) = Fixture::load(&format_path("dev_1_legs_20.csv"));

        _test(
            &mut ZigZag::new(
                ctx.clone(),
                ZigZagConfig {
                    allow_zig_zag_on_one_bar: true,
                    depth: 20,
                    dev_threshold: 1.0,
                    difference_price_mode: DifferencePriceMode::Absolute,
                },
            ),
            &df,
        );
    }
}
