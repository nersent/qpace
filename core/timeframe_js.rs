cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
  use wasm_bindgen::prelude::*;
}}
use crate::{rs_utils::get_oldest_possible_datetime, timeframe::Timeframe};

cfg_if::cfg_if! { if #[cfg(feature = "bindings_wasm")] {
    #[wasm_bindgen(js_name = "Timeframe")]
    pub struct JsTimeframe {
        inner: Timeframe,
    }

    impl From<Timeframe> for JsTimeframe {
        fn from(inner: Timeframe) -> Self {
            JsTimeframe { inner }
        }
    }

    impl Into<Timeframe> for JsTimeframe {
        fn into(self) -> Timeframe {
            self.inner
        }
    }

    #[wasm_bindgen(js_class = Timeframe)]
    impl JsTimeframe {
        #[wasm_bindgen(js_name = toString)]
        pub fn to_string(&self) -> String {
            self.inner.into()
        }

        #[wasm_bindgen(js_name = years)]
        pub fn years(value: usize) -> JsTimeframe {
            Timeframe::Years(value).into()
        }

        #[wasm_bindgen(js_name = months)]
        pub fn months(value: usize) -> JsTimeframe {
            Timeframe::Months(value).into()
        }

        #[wasm_bindgen(js_name = weeks)]
        pub fn weeks(value: usize) -> JsTimeframe {
            Timeframe::Weeks(value).into()
        }

        #[wasm_bindgen(js_name = days)]
        pub fn days(value: usize) -> JsTimeframe {
            Timeframe::Days(value).into()
        }

        #[wasm_bindgen(js_name = hours)]
        pub fn hours(value: usize) -> JsTimeframe {
            Timeframe::Hours(value).into()
        }

        #[wasm_bindgen(js_name = minutes)]
        pub fn minutes(value: usize) -> JsTimeframe {
            Timeframe::Minutes(value).into()
        }

        #[wasm_bindgen(js_name = seconds)]
        pub fn seconds(value: usize) -> JsTimeframe {
            Timeframe::Seconds(value).into()
        }

        #[wasm_bindgen(js_name = ticks)]
        pub fn ticks(value: usize) -> JsTimeframe {
            Timeframe::Ticks(value).into()
        }

        #[wasm_bindgen(js_name = ranges)]
        pub fn ranges(value: usize) -> JsTimeframe {
            Timeframe::Ranges(value).into()
        }
    }
}}
