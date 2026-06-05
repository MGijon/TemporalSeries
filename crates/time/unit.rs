/// The unit in which `i64` index values are expressed.
///
/// A [`TemporalSeries`](crate::series::TemporalSeries) stores its time axis as
/// a `Vec<i64>`. `TimeUnit` records what those integers mean so that callers
/// can convert between raw integers and calendar representations without
/// ambiguity.
///
/// # Example
///
/// ```rust
/// use temporalseries::time::TimeUnit;
///
/// let unit = TimeUnit::Milliseconds;
/// assert_eq!(unit, TimeUnit::Milliseconds);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    /// Seconds since the Unix epoch.
    Seconds,
    /// Milliseconds since the Unix epoch.
    Milliseconds,
    /// Microseconds since the Unix epoch.
    Microseconds,
    /// Nanoseconds since the Unix epoch.
    ///
    /// Note: nanosecond timestamps overflow `i64` for dates beyond roughly
    /// year 2262. Use [`TimeUnit::Microseconds`] or coarser units for distant
    /// future dates.
    Nanoseconds,
}

impl TimeUnit {
    /// Converts a [`chrono::DateTime<Utc>`] to an `i64` in this unit.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::time::TimeUnit;
    /// use chrono::{DateTime, Utc, TimeZone};
    ///
    /// let dt: DateTime<Utc> = Utc.timestamp_opt(1_000, 0).unwrap();
    /// assert_eq!(TimeUnit::Seconds.from_datetime(dt), 1_000);
    /// assert_eq!(TimeUnit::Milliseconds.from_datetime(dt), 1_000_000);
    /// ```
    #[cfg(feature = "chrono")]
    pub fn from_datetime(&self, dt: chrono::DateTime<chrono::Utc>) -> i64 {
        match self {
            TimeUnit::Seconds => dt.timestamp(),
            TimeUnit::Milliseconds => dt.timestamp_millis(),
            TimeUnit::Microseconds => dt.timestamp_micros(),
            TimeUnit::Nanoseconds => dt
                .timestamp_nanos_opt()
                .expect("timestamp overflows i64 nanoseconds"),
        }
    }

    /// Converts an `i64` in this unit to a [`chrono::DateTime<Utc>`].
    ///
    /// Returns `None` if the value is out of the representable range.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::time::TimeUnit;
    ///
    /// let dt = TimeUnit::Seconds.to_datetime(0).unwrap();
    /// assert_eq!(dt.timestamp(), 0);
    /// ```
    #[cfg(feature = "chrono")]
    pub fn to_datetime(&self, ts: i64) -> Option<chrono::DateTime<chrono::Utc>> {
        use chrono::TimeZone;
        match self {
            TimeUnit::Seconds => chrono::Utc.timestamp_opt(ts, 0).single(),
            TimeUnit::Milliseconds => {
                let secs = ts.div_euclid(1_000);
                let nanos = ts.rem_euclid(1_000) * 1_000_000;
                chrono::Utc.timestamp_opt(secs, nanos as u32).single()
            }
            TimeUnit::Microseconds => {
                let secs = ts.div_euclid(1_000_000);
                let nanos = ts.rem_euclid(1_000_000) * 1_000;
                chrono::Utc.timestamp_opt(secs, nanos as u32).single()
            }
            TimeUnit::Nanoseconds => {
                let secs = ts.div_euclid(1_000_000_000);
                let nanos = ts.rem_euclid(1_000_000_000);
                chrono::Utc.timestamp_opt(secs, nanos as u32).single()
            }
        }
    }
}
