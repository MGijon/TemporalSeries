use std::fmt;

/// Errors that can be returned by `temporalseries` operations.
#[derive(Debug)]
pub enum TemporalSeriesError {
    /// `index` and `values` have different lengths.
    ///
    /// Both must have the same length to form a valid [`TimeSeries`](crate::series::TimeSeries).
    LengthMismatch { index_len: usize, values_len: usize },

    /// The series contains no elements.
    EmptySeries,

    /// The rolling window is larger than the series.
    ///
    /// A window of size `window` requires at least `window` observations,
    /// but the series only has `series_len`.
    InvalidWindow { window: usize, series_len: usize },

    /// A filesystem or IO operation failed.
    ///
    /// Wraps [`std::io::Error`]. The underlying cause is accessible via
    /// [`std::error::Error::source`].
    IoError(std::io::Error),

    /// A field in a CSV row could not be parsed.
    ///
    /// The message includes the line number and the offending value.
    ParseError(String),
}

impl fmt::Display for TemporalSeriesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemporalSeriesError::LengthMismatch {
                index_len,
                values_len,
            } => {
                write!(
                    f,
                    "length mismatch: index has {index_len} elements, values has {values_len}"
                )
            }
            TemporalSeriesError::EmptySeries => {
                write!(f, "series is empty")
            }
            TemporalSeriesError::InvalidWindow { window, series_len } => {
                write!(
                    f,
                    "window {window} is larger than series length {series_len}"
                )
            }
            TemporalSeriesError::IoError(e) => {
                write!(f, "IO error: {e}")
            }
            TemporalSeriesError::ParseError(msg) => {
                write!(f, "parse error: {msg}")
            }
        }
    }
}

impl std::error::Error for TemporalSeriesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TemporalSeriesError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for TemporalSeriesError {
    fn from(e: std::io::Error) -> Self {
        TemporalSeriesError::IoError(e)
    }
}
