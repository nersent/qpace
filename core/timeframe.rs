use std::fmt;

use chrono::Duration;

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(try_from = "String", into = "String"))]
pub enum Timeframe {
    Years(usize),
    Months(usize),
    Weeks(usize),
    Days(usize),
    Hours(usize),
    Minutes(usize),
    Seconds(usize),
    Ticks(usize),
    Ranges(usize),
    Unknown(),
}

impl Default for Timeframe {
    #[inline]
    fn default() -> Self {
        Timeframe::Unknown()
    }
}

impl Into<String> for &Timeframe {
    #[inline]
    fn into(self) -> String {
        return match self {
            Timeframe::Years(value) => format!("{}Y", value),
            Timeframe::Months(value) => format!("{}M", value),
            Timeframe::Weeks(value) => format!("{}W", value),
            Timeframe::Days(value) => format!("{}D", value),
            Timeframe::Hours(value) => format!("{}h", value),
            Timeframe::Minutes(value) => format!("{}m", value),
            Timeframe::Seconds(value) => format!("{}s", value),
            Timeframe::Ticks(value) => format!("{}T", value),
            Timeframe::Ranges(value) => format!("{}R", value),
            Timeframe::Unknown() => String::from("?"),
        };
    }
}

impl Into<String> for Timeframe {
    #[inline]
    fn into(self) -> String {
        (&self).into()
    }
}

impl fmt::Display for Timeframe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&Into::<String>::into(self))
    }
}

impl From<String> for Timeframe {
    #[inline]
    fn from(value: String) -> Self {
        if value == "" || value == "?" {
            return Timeframe::Unknown();
        }
        let mut chars = value.chars();
        let mut num = String::new();
        let mut unit = String::new();
        while let Some(c) = chars.next() {
            if c.is_numeric() {
                num.push(c);
            } else {
                unit.push(c);
            }
        }
        let num = num.parse::<usize>();
        if num.is_err() {
            return Timeframe::Unknown();
        }
        let num = num.unwrap();
        return match unit.as_str() {
            "Y" => Timeframe::Years(num),
            "M" => Timeframe::Months(num),
            "W" => Timeframe::Weeks(num),
            "D" => Timeframe::Days(num),
            "h" => Timeframe::Hours(num),
            "m" => Timeframe::Minutes(num),
            "s" => Timeframe::Seconds(num),
            "T" => Timeframe::Ticks(num),
            "R" => Timeframe::Ranges(num),
            _ => Timeframe::Unknown(),
        };
    }
}

impl TryInto<Duration> for Timeframe {
    type Error = String;

    #[inline]
    fn try_into(self) -> Result<Duration, Self::Error> {
        return match self {
            Timeframe::Years(value) => Ok(Duration::days((value * 365) as i64)),
            Timeframe::Months(value) => Ok(Duration::days((value * 30) as i64)),
            Timeframe::Weeks(value) => Ok(Duration::weeks(value as i64)),
            Timeframe::Days(value) => Ok(Duration::days(value as i64)),
            Timeframe::Hours(value) => Ok(Duration::hours(value as i64)),
            Timeframe::Minutes(value) => Ok(Duration::minutes(value as i64)),
            Timeframe::Seconds(value) => Ok(Duration::seconds(value as i64)),
            _ => Err("Unknown timeframe".to_string()),
        };
    }
}

impl TryInto<Timeframe> for Duration {
    type Error = String;

    #[inline]
    fn try_into(self) -> Result<Timeframe, Self::Error> {
        let seconds = self.num_seconds();
        let minutes = self.num_minutes();
        let hours = self.num_hours();
        let days = self.num_days();
        let weeks = self.num_weeks();
        let months = (self.num_days() / 30) as usize;
        let years = (self.num_days() / 365) as usize;

        if years > 0 {
            return Ok(Timeframe::Years(years));
        } else if months > 0 {
            return Ok(Timeframe::Months(months));
        } else if weeks > 0 {
            return Ok(Timeframe::Weeks(weeks as usize));
        } else if days > 0 {
            return Ok(Timeframe::Days(days as usize));
        } else if hours > 0 {
            return Ok(Timeframe::Hours(hours as usize));
        } else if minutes > 0 {
            return Ok(Timeframe::Minutes(minutes as usize));
        } else if seconds > 0 {
            return Ok(Timeframe::Seconds(seconds as usize));
        }

        Err("Unknown timeframe".to_string())
    }
}

impl Timeframe {
    #[inline]
    pub fn years(&self) -> Option<usize> {
        match self {
            Timeframe::Years(value) => Some(*value),
            _ => None,
        }
    }

    #[inline]
    pub fn months(&self) -> Option<usize> {
        match self {
            Timeframe::Months(value) => Some(*value),
            _ => None,
        }
    }

    #[inline]
    pub fn weeks(&self) -> Option<usize> {
        match self {
            Timeframe::Weeks(value) => Some(*value),
            _ => None,
        }
    }

    #[inline]
    pub fn days(&self) -> Option<usize> {
        match self {
            Timeframe::Days(value) => Some(*value),
            _ => None,
        }
    }

    #[inline]
    pub fn hours(&self) -> Option<usize> {
        match self {
            Timeframe::Hours(value) => Some(*value),
            _ => None,
        }
    }

    #[inline]
    pub fn minutes(&self) -> Option<usize> {
        match self {
            Timeframe::Minutes(value) => Some(*value),
            _ => None,
        }
    }

    #[inline]
    pub fn seconds(&self) -> Option<usize> {
        match self {
            Timeframe::Seconds(value) => Some(*value),
            _ => None,
        }
    }

    #[inline]
    pub fn ticks(&self) -> Option<usize> {
        match self {
            Timeframe::Ticks(value) => Some(*value),
            _ => None,
        }
    }

    #[inline]
    pub fn ranges(&self) -> Option<usize> {
        match self {
            Timeframe::Ranges(value) => Some(*value),
            _ => None,
        }
    }

    #[inline]
    pub fn unknown(&self) -> bool {
        matches!(self, Timeframe::Unknown())
    }

    //     const qpTimeframeToTradingView = (
    //   timeframe: qp.Timeframe,
    // ): TradingViewTimeframe => {
    //   if (timeframe.minutes) {
    //     return `${timeframe.minutes}` as TradingViewTimeframe;
    //   }
    //   if (timeframe.hours) {
    //     return `${timeframe.hours * 60}` as TradingViewTimeframe;
    //   }
    //   return timeframe.toString() as TradingViewTimeframe;
    // };
    // #[inline]
    // pub fn to_tradingview() -> String {
    //     match self {

    //     }
    // }
}
